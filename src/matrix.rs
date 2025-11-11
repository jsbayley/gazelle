//! High-performance matrix operations for structural analysis

use crate::error::{GazelleError, Result};
use nalgebra::{DMatrix, DVector, Cholesky, LU, QR, SVD, SymmetricEigen};
use rayon::prelude::*;
use std::sync::Arc;

/// High-performance matrix operations for structural analysis
pub struct MatrixOps;

impl MatrixOps {
    /// Solve linear system Ax = b using the most appropriate method
    pub fn solve_linear_system(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>> {
        // Check matrix dimensions
        if a.nrows() != a.ncols() {
            return Err(GazelleError::MatrixError(
                "Matrix must be square for linear system solving".to_string()
            ));
        }

        if a.nrows() != b.len() {
            return Err(GazelleError::MatrixError(
                "Matrix and vector dimensions must match".to_string()
            ));
        }

        // Choose solver based on matrix properties
        if Self::is_symmetric(a) && Self::is_positive_definite(a) {
            // Use Cholesky decomposition for symmetric positive definite matrices
            Self::solve_cholesky(a, b)
        } else if a.nrows() < 1000 {
            // Use LU decomposition for small to medium matrices
            Self::solve_lu(a, b)
        } else {
            // Use iterative methods for large matrices
            Self::solve_iterative(a, b)
        }
    }

    /// Solve using Cholesky decomposition (for SPD matrices)
    pub fn solve_cholesky(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>> {
        match Cholesky::new(a.clone()) {
            Some(chol) => Ok(chol.solve(b)),
            None => Err(GazelleError::MatrixError(
                "Cholesky decomposition failed - matrix may not be positive definite".to_string()
            )),
        }
    }

    /// Solve using LU decomposition
    pub fn solve_lu(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>> {
        let lu = LU::new(a.clone());
        match lu.solve(b) {
            Some(x) => Ok(x),
            None => Err(GazelleError::SingularMatrix),
        }
    }

    /// Solve using QR decomposition
    pub fn solve_qr(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>> {
        let qr = QR::new(a.clone());
        match qr.solve(b) {
            Some(x) => Ok(x),
            None => Err(GazelleError::SingularMatrix),
        }
    }

    /// Solve using iterative methods (Conjugate Gradient for SPD, GMRES otherwise)
    pub fn solve_iterative(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>> {
        if Self::is_symmetric(a) && Self::is_positive_definite(a) {
            Self::conjugate_gradient(a, b, 1e-10, 1000)
        } else {
            // For now, fall back to direct methods
            // TODO: Implement GMRES or BiCGSTAB
            Self::solve_lu(a, b)
        }
    }

    /// Conjugate Gradient method for symmetric positive definite matrices
    pub fn conjugate_gradient(
        a: &DMatrix<f64>,
        b: &DVector<f64>,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<DVector<f64>> {
        let n = a.nrows();
        let mut x = DVector::zeros(n);
        let mut r = b - a * &x;
        let mut p = r.clone();
        let mut rsold = r.dot(&r);

        for _iteration in 0..max_iterations {
            let ap = a * &p;
            let alpha = rsold / p.dot(&ap);
            x += alpha * &p;
            r -= alpha * &ap;
            let rsnew = r.dot(&r);

            if rsnew.sqrt() < tolerance {
                return Ok(x);
            }

            let beta = rsnew / rsold;
            p = &r + beta * &p;
            rsold = rsnew;
        }

        Err(GazelleError::ConvergenceFailure {
            iterations: max_iterations,
        })
    }

    /// Compute eigenvalues and eigenvectors for symmetric matrices
    pub fn eigensolve_symmetric(a: &DMatrix<f64>) -> Result<(DVector<f64>, DMatrix<f64>)> {
        if !Self::is_symmetric(a) {
            return Err(GazelleError::MatrixError(
                "Matrix must be symmetric for symmetric eigensolve".to_string()
            ));
        }

        match SymmetricEigen::new(a.clone()) {
            eigen => Ok((eigen.eigenvalues, eigen.eigenvectors)),
        }
    }

    /// Compute a subset of eigenvalues/eigenvectors using power iteration
    pub fn eigensolve_subset(
        a: &DMatrix<f64>,
        num_modes: usize,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<(DVector<f64>, DMatrix<f64>)> {
        let n = a.nrows();
        let mut eigenvalues = DVector::zeros(num_modes);
        let mut eigenvectors = DMatrix::zeros(n, num_modes);

        // Use inverse power iteration with shifts for each mode
        for mode in 0..num_modes {
            let shift = if mode == 0 { 0.0 } else { eigenvalues[mode - 1] + 1e-6 };
            let shifted_matrix = a - DMatrix::identity(n, n) * shift;
            
            // Solve for the lowest eigenvalue of the shifted system
            let (eigenval, eigenvec) = Self::power_iteration(&shifted_matrix, tolerance, max_iterations)?;
            eigenvalues[mode] = eigenval + shift;
            eigenvectors.set_column(mode, &eigenvec);
        }

        Ok((eigenvalues, eigenvectors))
    }

    /// Power iteration for finding dominant eigenvalue/eigenvector
    fn power_iteration(
        a: &DMatrix<f64>,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<(f64, DVector<f64>)> {
        let n = a.nrows();
        let mut v = DVector::from_fn(n, |_, _| rand::random::<f64>());
        v.normalize_mut();

        let mut lambda = 0.0;

        for _iteration in 0..max_iterations {
            let av = a * &v;
            let lambda_new = v.dot(&av);
            let v_new = av.normalize();

            if (lambda_new - lambda).abs() < tolerance {
                return Ok((lambda_new, v_new));
            }

            lambda = lambda_new;
            v = v_new;
        }

        Err(GazelleError::ConvergenceFailure {
            iterations: max_iterations,
        })
    }

    /// Assemble global stiffness matrix from element matrices
    pub fn assemble_global_stiffness(
        element_matrices: &[(Vec<usize>, DMatrix<f64>)],
        num_dofs: usize,
    ) -> DMatrix<f64> {
        let mut global_k = DMatrix::zeros(num_dofs, num_dofs);

        // Parallel assembly for large numbers of elements
        if element_matrices.len() > 100 {
            Self::assemble_parallel(element_matrices, num_dofs)
        } else {
            // Sequential assembly for smaller problems
            for (dof_indices, k_elem) in element_matrices {
                Self::add_element_matrix(&mut global_k, k_elem, dof_indices);
            }
            global_k
        }
    }

    /// Parallel assembly of global stiffness matrix
    fn assemble_parallel(
        element_matrices: &[(Vec<usize>, DMatrix<f64>)],
        num_dofs: usize,
    ) -> DMatrix<f64> {
        use std::sync::Mutex;
        
        let global_k = Arc::new(Mutex::new(DMatrix::zeros(num_dofs, num_dofs)));
        
        element_matrices.par_iter().for_each(|(dof_indices, k_elem)| {
            let mut k_guard = global_k.lock().unwrap();
            Self::add_element_matrix(&mut k_guard, k_elem, dof_indices);
        });

        Arc::try_unwrap(global_k).unwrap().into_inner().unwrap()
    }

    /// Add element matrix to global matrix
    fn add_element_matrix(
        global_k: &mut DMatrix<f64>,
        element_k: &DMatrix<f64>,
        dof_indices: &[usize],
    ) {
        for (i, &global_i) in dof_indices.iter().enumerate() {
            for (j, &global_j) in dof_indices.iter().enumerate() {
                global_k[(global_i, global_j)] += element_k[(i, j)];
            }
        }
    }

    /// Apply constraints to stiffness matrix and load vector
    pub fn apply_constraints(
        k: &mut DMatrix<f64>,
        f: &mut DVector<f64>,
        constrained_dofs: &[usize],
        prescribed_values: &[f64],
    ) -> Result<()> {
        if constrained_dofs.len() != prescribed_values.len() {
            return Err(GazelleError::ValidationError(
                "Number of constrained DOFs must match prescribed values".to_string()
            ));
        }

        // Penalty method for applying constraints
        let penalty_factor = 1e12;

        for (&dof, &value) in constrained_dofs.iter().zip(prescribed_values.iter()) {
            if dof >= k.nrows() {
                return Err(GazelleError::ValidationError(
                    format!("Constrained DOF {} exceeds matrix size", dof)
                ));
            }

            // Apply penalty method
            k[(dof, dof)] += penalty_factor;
            f[dof] += penalty_factor * value;
        }

        Ok(())
    }

    /// Check if matrix is symmetric
    pub fn is_symmetric(matrix: &DMatrix<f64>) -> bool {
        if matrix.nrows() != matrix.ncols() {
            return false;
        }

        let tolerance = 1e-12;
        for i in 0..matrix.nrows() {
            for j in i+1..matrix.ncols() {
                if (matrix[(i, j)] - matrix[(j, i)]).abs() > tolerance {
                    return false;
                }
            }
        }
        true
    }

    /// Check if matrix is positive definite (approximate check)
    pub fn is_positive_definite(matrix: &DMatrix<f64>) -> bool {
        // Simple check: all diagonal elements are positive
        // This is necessary but not sufficient
        matrix.diagonal().iter().all(|&x| x > 0.0)
    }

    /// Compute matrix condition number
    pub fn condition_number(matrix: &DMatrix<f64>) -> Result<f64> {
        let svd = SVD::new(matrix.clone(), true, true);
        let singular_values = svd.singular_values;
        
        if singular_values.len() == 0 {
            return Err(GazelleError::MatrixError("SVD failed".to_string()));
        }

        let max_sv = singular_values.max();
        let min_sv = singular_values.min();

        if min_sv < 1e-14 {
            Ok(f64::INFINITY)
        } else {
            Ok(max_sv / min_sv)
        }
    }

    /// Compute matrix norm (Frobenius norm)
    pub fn frobenius_norm(matrix: &DMatrix<f64>) -> f64 {
        matrix.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Compute residual for linear system Ax = b
    pub fn residual_norm(a: &DMatrix<f64>, x: &DVector<f64>, b: &DVector<f64>) -> f64 {
        let residual = b - a * x;
        residual.norm()
    }

    /// Extract submatrix based on DOF indices
    pub fn extract_submatrix(
        matrix: &DMatrix<f64>,
        row_indices: &[usize],
        col_indices: &[usize],
    ) -> DMatrix<f64> {
        let mut submatrix = DMatrix::zeros(row_indices.len(), col_indices.len());
        
        for (i, &row) in row_indices.iter().enumerate() {
            for (j, &col) in col_indices.iter().enumerate() {
                submatrix[(i, j)] = matrix[(row, col)];
            }
        }
        
        submatrix
    }

    /// Extract subvector based on DOF indices
    pub fn extract_subvector(vector: &DVector<f64>, indices: &[usize]) -> DVector<f64> {
        DVector::from_fn(indices.len(), |i, _| vector[indices[i]])
    }

    /// Expand solution vector to full DOF vector
    pub fn expand_solution(
        reduced_solution: &DVector<f64>,
        free_dofs: &[usize],
        constrained_dofs: &[usize],
        prescribed_values: &[f64],
        total_dofs: usize,
    ) -> DVector<f64> {
        let mut full_solution = DVector::zeros(total_dofs);
        
        // Set free DOF values
        for (i, &dof) in free_dofs.iter().enumerate() {
            full_solution[dof] = reduced_solution[i];
        }
        
        // Set prescribed values
        for (&dof, &value) in constrained_dofs.iter().zip(prescribed_values.iter()) {
            full_solution[dof] = value;
        }
        
        full_solution
    }

    /// Compute element forces from displacements
    pub fn compute_element_forces(
        element_stiffness: &DMatrix<f64>,
        element_displacements: &DVector<f64>,
    ) -> DVector<f64> {
        element_stiffness * element_displacements
    }

    /// Compute strain energy
    pub fn strain_energy(
        stiffness: &DMatrix<f64>,
        displacements: &DVector<f64>,
    ) -> f64 {
        0.5 * displacements.dot(&(stiffness * displacements))
    }
}

/// Sparse matrix operations for large-scale problems
pub struct SparseMatrixOps;

impl SparseMatrixOps {
    // TODO: Implement sparse matrix operations using sprs crate or similar
    // This would be crucial for large-scale problems
    
    /// Convert dense matrix to sparse format (placeholder)
    pub fn to_sparse(_matrix: &DMatrix<f64>) -> Result<()> {
        // TODO: Implement sparse matrix conversion
        Err(GazelleError::MatrixError(
            "Sparse matrix operations not yet implemented".to_string()
        ))
    }
}

/// Benchmarking utilities for matrix operations
#[cfg(feature = "benchmarks")]
pub mod benchmarks {
    use super::*;
    use criterion::{Criterion, BenchmarkId};

    pub fn benchmark_matrix_solve(c: &mut Criterion) {
        let mut group = c.benchmark_group("matrix_solve");
        
        for size in [10, 50, 100, 500, 1000].iter() {
            let a = DMatrix::from_fn(*size, *size, |i, j| {
                if i == j {
                    2.0
                } else if (i as i32 - j as i32).abs() == 1 {
                    -1.0
                } else {
                    0.0
                }
            });
            let b = DVector::from_element(*size, 1.0);

            group.bench_with_input(
                BenchmarkId::new("cholesky", size),
                size,
                |bench, _| {
                    bench.iter(|| MatrixOps::solve_cholesky(&a, &b))
                }
            );

            group.bench_with_input(
                BenchmarkId::new("lu", size),
                size,
                |bench, _| {
                    bench.iter(|| MatrixOps::solve_lu(&a, &b))
                }
            );
        }
        
        group.finish();
    }

    pub fn benchmark_stiffness_assembly(c: &mut Criterion) {
        let mut group = c.benchmark_group("stiffness_assembly");
        
        for num_elements in [10, 100, 1000].iter() {
            let element_matrices: Vec<(Vec<usize>, DMatrix<f64>)> = (0..*num_elements)
                .map(|i| {
                    let dofs = vec![i*2, i*2+1, i*2+2, i*2+3];
                    let k = DMatrix::identity(4, 4);
                    (dofs, k)
                })
                .collect();
            
            group.bench_with_input(
                BenchmarkId::new("assembly", num_elements),
                num_elements,
                |bench, _| {
                    bench.iter(|| {
                        MatrixOps::assemble_global_stiffness(&element_matrices, num_elements * 2 + 2)
                    })
                }
            );
        }
        
        group.finish();
    }
}