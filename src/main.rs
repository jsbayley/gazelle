use gazelle::prelude::*;
use gazelle::io::{ModelIO, ExportUtilities};
use gazelle::analysis::Analysis;
use gazelle::error::Validate;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use log::{info, error};

/// Gazelle: High-Performance Structural Analysis Engine
#[derive(Parser)]
#[command(name = "gazelle")]
#[command(about = "A high-performance structural analysis engine written in Rust")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run structural analysis on a model file
    Analyze {
        /// Input model file
        input: PathBuf,
        
        /// Output file for results
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Analysis type
        #[arg(short, long, default_value = "static")]
        analysis_type: String,
        
        /// Number of modes for modal analysis
        #[arg(short, long, default_value = "10")]
        modes: usize,
        
        /// Export VTK file for visualization
        #[arg(long)]
        vtk: Option<PathBuf>,
    },
    
    /// Create a new model file from scratch
    Create {
        /// Output file path
        output: PathBuf,
        
        /// File format (json, yaml, custom)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Create example model
        #[arg(long)]
        example: Option<String>,
    },
    
    /// Validate a model file
    Validate {
        /// Input model file
        input: PathBuf,
    },
    
    /// Convert between model file formats
    Convert {
        /// Input file
        input: PathBuf,
        
        /// Output file
        output: PathBuf,
    },
    
    /// Show model information
    Info {
        /// Input model file
        input: PathBuf,
    },
    
    /// Run benchmarks
    Benchmark {
        /// Benchmark type
        #[arg(default_value = "matrix")]
        bench_type: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Warn)
            .init();
    }

    println!("🦌 Gazelle Structural Analysis Engine 💨");
    
    let result = match cli.command {
        Commands::Analyze { input, output, analysis_type, modes, vtk } => {
            run_analysis(input, output, analysis_type, modes, vtk)
        }
        Commands::Create { output, format, example } => {
            create_model(output, format, example)
        }
        Commands::Validate { input } => {
            validate_model(input)
        }
        Commands::Convert { input, output } => {
            convert_model(input, output)
        }
        Commands::Info { input } => {
            show_model_info(input)
        }
        Commands::Benchmark { bench_type } => {
            run_benchmark(bench_type)
        }
    };

    if let Err(e) = result {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_analysis(
    input: PathBuf,
    output: Option<PathBuf>,
    analysis_type: String,
    modes: usize,
    vtk: Option<PathBuf>,
) -> Result<()> {
    info!("Loading model from: {}", input.display());
    let model = ModelIO::load_from_file(&input)?;
    
    let analysis = Analysis::new(model.clone());
    
    // Show model summary
    let summary = analysis.model_summary();
    summary.print();
    
    // Run analysis
    let results = match analysis_type.to_lowercase().as_str() {
        "static" => {
            info!("Running static analysis");
            analysis.static_analysis()?
        }
        "modal" => {
            info!("Running modal analysis for {} modes", modes);
            analysis.modal_analysis(modes)?
        }
        "dynamic" | "time-history" => {
            info!("Running time history analysis");
            analysis.time_history_analysis(0.01, 1.0)?
        }
        _ => {
            return Err(GazelleError::ValidationError(
                format!("Unknown analysis type: {}", analysis_type)
            ));
        }
    };
    
    // Print results summary
    results.print_summary();
    
    // Save results if output specified
    if let Some(output_path) = output {
        info!("Saving results to: {}", output_path.display());
        results.save_to_file(output_path.to_str().unwrap())?;
    }
    
    // Export VTK if requested
    if let Some(vtk_path) = vtk {
        info!("Exporting VTK to: {}", vtk_path.display());
        let vtk_content = ExportUtilities::export_results_vtk(&model, &results)?;
        std::fs::write(vtk_path, vtk_content)?;
    }
    
    Ok(())
}

fn create_model(output: PathBuf, _format: String, example: Option<String>) -> Result<()> {
    let model = if let Some(example_type) = example {
        create_example_model(&example_type)?
    } else {
        Model::new()
    };

    info!("Creating model file: {}", output.display());
    ModelIO::save_to_file(&model, &output)?;
    
    println!("Model created successfully!");
    Ok(())
}

fn create_example_model(example_type: &str) -> Result<Model> {
    match example_type {
        "truss" => create_simple_truss(),
        "beam" => create_cantilever_beam(),
        "frame" => create_simple_frame(),
        _ => Err(GazelleError::ValidationError(
            "Unknown example type. Available: truss, beam, frame".to_string()
        ))
    }
}

fn create_simple_truss() -> Result<Model> {
    let mut model = Model::new();
    
    // Create nodes
    model.add_node(Node::new(0, 0.0, 0.0, 0.0))?;
    model.add_node(Node::new(1, 1.0, 0.0, 0.0))?;
    model.add_node(Node::new(2, 0.5, 1.0, 0.0))?;
    
    // Create material (steel)
    let steel = Material::steel(0, "Steel".to_string());
    model.add_material(steel)?;
    
    // Create elements
    let props = ElementProperties::truss(0.01); // 10 cm²
    model.add_element(Element::new(0, ElementType::Truss2D, vec![0, 1], 0, props.clone()))?;
    model.add_element(Element::new(1, ElementType::Truss2D, vec![1, 2], 0, props.clone()))?;
    model.add_element(Element::new(2, ElementType::Truss2D, vec![2, 0], 0, props))?;
    
    // Add constraints (pin supports)
    model.add_constraint(Constraint::pinned_support(0, 0));
    model.add_constraint(Constraint::pinned_support(1, 1));
    
    // Add load (downward force at node 2)
    model.add_load(Load::nodal_force(0, 2, Dof::Uy, -1000.0, "Live".to_string()));
    
    Ok(model)
}

fn create_cantilever_beam() -> Result<Model> {
    let mut model = Model::new();
    
    // Create nodes
    model.add_node(Node::new(0, 0.0, 0.0, 0.0))?;
    model.add_node(Node::new(1, 1.0, 0.0, 0.0))?;
    
    // Create material (steel)
    let steel = Material::steel(0, "Steel".to_string());
    model.add_material(steel)?;
    
    // Create beam element
    let props = ElementProperties::beam(0.01, 8.33e-5, 8.33e-5, 1.67e-4); // Rectangular 10cm x 10cm
    model.add_element(Element::new(0, ElementType::Beam2D, vec![0, 1], 0, props))?;
    
    // Add constraints (fixed support at node 0)
    model.add_constraint(Constraint::fixed_support(0, 0));
    
    // Add load (tip load)
    model.add_load(Load::nodal_force(0, 1, Dof::Uy, -1000.0, "Live".to_string()));
    
    Ok(model)
}

fn create_simple_frame() -> Result<Model> {
    let mut model = Model::new();
    
    // Create nodes for a simple portal frame
    model.add_node(Node::new(0, 0.0, 0.0, 0.0))?; // Left base
    model.add_node(Node::new(1, 0.0, 3.0, 0.0))?; // Left top
    model.add_node(Node::new(2, 4.0, 3.0, 0.0))?; // Right top
    model.add_node(Node::new(3, 4.0, 0.0, 0.0))?; // Right base
    
    // Create material (steel)
    let steel = Material::steel(0, "Steel".to_string());
    model.add_material(steel)?;
    
    // Create frame elements
    let column_props = ElementProperties::beam(0.02, 1.67e-4, 1.67e-4, 3.33e-4); // 20cm x 10cm column
    let beam_props = ElementProperties::beam(0.015, 1.25e-4, 1.25e-4, 2.5e-4);   // 15cm x 10cm beam
    
    model.add_element(Element::new(0, ElementType::Frame2D, vec![0, 1], 0, column_props.clone()))?; // Left column
    model.add_element(Element::new(1, ElementType::Frame2D, vec![1, 2], 0, beam_props))?;          // Beam
    model.add_element(Element::new(2, ElementType::Frame2D, vec![2, 3], 0, column_props))?;        // Right column
    
    // Add constraints (fixed supports at bases)
    model.add_constraint(Constraint::fixed_support(0, 0));
    model.add_constraint(Constraint::fixed_support(1, 3));
    
    // Add loads (distributed load on beam - simplified as nodal loads)
    model.add_load(Load::nodal_force(0, 1, Dof::Uy, -2000.0, "Dead".to_string()));
    model.add_load(Load::nodal_force(1, 2, Dof::Uy, -2000.0, "Dead".to_string()));
    
    Ok(model)
}

fn validate_model(input: PathBuf) -> Result<()> {
    info!("Validating model: {}", input.display());
    let model = ModelIO::load_from_file(&input)?;
    
    match model.validate() {
        Ok(()) => {
            println!("✓ Model validation passed!");
            let summary = Analysis::new(model).model_summary();
            summary.print();
        }
        Err(e) => {
            println!("✗ Model validation failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

fn convert_model(input: PathBuf, output: PathBuf) -> Result<()> {
    info!("Converting {} to {}", input.display(), output.display());
    let model = ModelIO::load_from_file(&input)?;
    ModelIO::save_to_file(&model, &output)?;
    println!("Conversion completed successfully!");
    Ok(())
}

fn show_model_info(input: PathBuf) -> Result<()> {
    info!("Loading model info from: {}", input.display());
    let model = ModelIO::load_from_file(&input)?;
    
    let analysis = Analysis::new(model.clone());
    let summary = analysis.model_summary();
    summary.print();
    
    // Additional detailed information
    println!("\nDetailed Information:");
    println!("Nodes:");
    for node in model.nodes.values().take(5) {
        println!("  Node {}: ({:.3}, {:.3}, {:.3})", node.id, node.x, node.y, node.z);
    }
    if model.nodes.len() > 5 {
        println!("  ... and {} more nodes", model.nodes.len() - 5);
    }
    
    println!("Materials:");
    for material in model.materials.values() {
        println!("  {}: {} ({:?})", material.id, material.name, material.material_type);
        if let Some(e) = material.properties.young_modulus {
            println!("    E = {:.2e} Pa", e);
        }
    }
    
    Ok(())
}

fn run_benchmark(bench_type: String) -> Result<()> {
    match bench_type.as_str() {
        "matrix" => {
            println!("Running matrix operation benchmarks...");
            println!("This would run comprehensive matrix benchmarks in a real implementation.");
            println!("Use 'cargo bench' for actual benchmarking.");
        }
        "assembly" => {
            println!("Running stiffness matrix assembly benchmarks...");
            println!("This would test assembly performance with various model sizes.");
        }
        _ => {
            return Err(GazelleError::ValidationError(
                "Unknown benchmark type. Available: matrix, assembly".to_string()
            ));
        }
    }
    Ok(())
}
