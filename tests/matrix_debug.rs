//! Debug test specifically for the matrix singularity issue

use gazelle::prelude::*;
use gazelle::error::Validate;
use gazelle::solvers::{StaticSolver, Solver};
use gazelle::constraints::ConstraintType;
use gazelle::loads::LoadType;

#[test]
fn test_matrix_debug() {
    println!("=== Matrix Assembly Debug Test ===");
    
    let mut model = Model::new();
    
    // Create minimal model - single truss element
    model.add_node(Node::new(0, 0.0, 0.0, 0.0)).unwrap(); // Fixed end
    model.add_node(Node::new(1, 1.0, 0.0, 0.0)).unwrap(); // Free end
    
    // Simple material for easy calculation
    let material = Material::linear_elastic(0, "Test".to_string(), 200e9, 0.3, 7850.0);
    model.add_material(material).unwrap();
    
    // Single truss element
    let props = ElementProperties::truss(0.01); // 100 cm²
    let element = Element::new(0, ElementType::Truss2D, vec![0, 1], 0, props);
    model.add_element(element).unwrap();
    
    // Fixed support at node 0
    model.add_constraint(Constraint::fixed_support(0, 0));
    
    // Load at node 1
    model.add_load(Load::nodal_force(0, 1, Dof::Ux, 10000.0, "Test Load".to_string()));
    
    println!("Model validation: {:?}", model.validate());
    
    // Debug info
    println!("Total nodes: {}", model.nodes.len());
    println!("Total elements: {}", model.elements.len());
    println!("Total materials: {}", model.materials.len());
    println!("Total loads: {}", model.loads.len());
    println!("Total constraints: {}", model.constraints.len());
    
    // Check element DOF mapping
    let element = model.elements.get(&0).unwrap();
    println!("Element type: {:?}", element.element_type);
    println!("Element DOFs per node: {}", element.dofs_per_node());
    println!("Element total DOFs: {}", element.total_dofs());
    println!("Element nodes: {:?}", element.nodes);
    
    // Check the constraints
    println!("Constraint details:");
    for (i, constraint) in model.constraints.iter().enumerate() {
        if let ConstraintType::NodalConstraint { node_id, constraints } = &constraint.constraint_type {
            println!("  Constraint {}: Node {} -> {:?}", i, node_id, constraints);
        }
    }
    
    // Check the load
    println!("Load details:");
    for (i, load) in model.loads.iter().enumerate() {
        if let LoadType::NodalForce { node_id, dof, magnitude } = &load.load_type {
            println!("  Load {}: Node {} DOF {:?} = {}", i, node_id, dof, magnitude);
        }
    }
    
    // Try to manually compute element stiffness matrix to check it
    use gazelle::elements::ElementFactory;
    
    let element = model.elements.get(&0).unwrap();
    let material = model.materials.get(&0).unwrap();
    let nodes: Vec<_> = element.nodes.iter()
        .map(|&id| model.get_node(id).unwrap().clone())
        .collect();
    
    println!("Node coordinates:");
    for (i, node) in nodes.iter().enumerate() {
        println!("  Node {}: ({}, {}, {})", i, node.x, node.y, node.z);
    }
    
    // First, let's check the local stiffness and transformation matrices separately
    use gazelle::elements::{Truss2D, ElementStiffness};
    
    let truss = Truss2D;
    
    match truss.local_stiffness_matrix(element, material) {
        Ok(k_local) => {
            println!("Local stiffness matrix ({}x{}):", k_local.nrows(), k_local.ncols());
            for i in 0..k_local.nrows() {
                print!("  [");
                for j in 0..k_local.ncols() {
                    print!("{:12.2e}", k_local[(i, j)]);
                    if j < k_local.ncols() - 1 { print!(", "); }
                }
                println!("]");
            }
        }
        Err(e) => {
            println!("✗ Failed to compute local stiffness matrix: {:?}", e);
        }
    }
    
    match truss.transformation_matrix(&nodes) {
        Ok(t) => {
            println!("Transformation matrix ({}x{}):", t.nrows(), t.ncols());
            for i in 0..t.nrows() {
                print!("  [");
                for j in 0..t.ncols() {
                    print!("{:12.2e}", t[(i, j)]);
                    if j < t.ncols() - 1 { print!(", "); }
                }
                println!("]");
            }
        }
        Err(e) => {
            println!("✗ Failed to compute transformation matrix: {:?}", e);
        }
    }

    match ElementFactory::compute_stiffness_matrix(element, material, &nodes) {
        Ok(k_elem) => {
            println!("Global stiffness matrix ({}x{}):", k_elem.nrows(), k_elem.ncols());
            for i in 0..k_elem.nrows() {
                print!("  [");
                for j in 0..k_elem.ncols() {
                    print!("{:12.2e}", k_elem[(i, j)]);
                    if j < k_elem.ncols() - 1 { print!(", "); }
                }
                println!("]");
            }
            
            // Check if matrix is singular
            let det = k_elem.determinant();
            println!("Global matrix determinant: {:.2e}", det);
            
            if det.abs() < 1e-12 {
                println!("⚠ Global stiffness matrix is singular!");
            }
        }
        Err(e) => {
            println!("✗ Failed to compute global stiffness matrix: {:?}", e);
        }
    }
    
    // Try manual solver to get more debug info
    let solver = StaticSolver::new();
    
    // This should give us more specific error information
    match solver.solve(&model) {
        Ok(results) => {
            println!("✓ Analysis succeeded!");
            results.print_summary();
        }
        Err(e) => {
            println!("✗ Analysis failed: {:?}", e);
        }
    }
}