# Gazelle AI Integration Guide

## AI Orchestration Features

### 1. Machine-Readable Outputs
All Gazelle commands support structured JSON output for easy AI parsing:

```bash
# Get model information in JSON
gz info model.json --format json

# Analysis results in structured format
gz analyze model.json --output results.json --format json --verbose

# Validation with detailed error reporting
gz validate model.json --format json --detailed
```

### 2. Batch Processing
Process multiple models:

```bash
# Batch analysis with status reporting
gz batch-analyze *.json --output-dir results/ --format json --progress

# Parallel processing for AI workloads
gz analyze-parallel models/ --workers 8 --format json
```

### 3. Template-Based Generation
AI can use predefined templates:

```bash
# Generate from templates
gz create --template truss --params params.json --output model.json

# List available templates
gz templates list --format json
```

## AI Integration Patterns

### 1. Model Generation Pipeline
```python
import subprocess
import json

def generate_model(params):
    """AI-friendly model generation"""
    cmd = [
        "gz", "create", 
        "--template", params["type"],
        "--output", f"{params['name']}.json",
        "--format", "json"
    ]
    
    # Add parameters dynamically
    for key, value in params.items():
        if key not in ["type", "name"]:
            cmd.extend([f"--{key}", str(value)])
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(result.stdout)

def analyze_model(model_file):
    """AI-friendly analysis"""
    cmd = ["gz", "analyze", model_file, "--format", "json"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(result.stdout)
```

### 2. Error Handling for AI
```python
def validate_with_ai_feedback(model_file):
    """Structured validation for AI decision making"""
    cmd = ["gz", "validate", model_file, "--format", "json", "--detailed"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode != 0:
        errors = json.loads(result.stderr)
        return {
            "valid": False,
            "errors": errors,
            "suggestions": generate_ai_suggestions(errors)
        }
    
    return {"valid": True, "summary": json.loads(result.stdout)}
```

### 3. Units and Type Safety
Gazelle's F# type system provides compile-time safety for AI-generated models:

```fsharp
// Units prevent dangerous mistakes in AI-generated models
let span = 10.0<m>          // Meters - type safe
let load = 25000.0<N>       // Newtons - type safe
let stress = load / (0.01<m^2>)  // Automatic unit checking
```

## AI Orchestration Examples

### Example 1: Optimization Loop
```python
def optimize_truss_design():
    best_design = None
    best_weight = float('inf')
    
    for iteration in range(100):
        # AI generates parameters
        params = ai_generate_truss_params()
        
        # Create model
        model = generate_model(params)
        
        # Validate
        validation = validate_with_ai_feedback(model['file'])
        if not validation['valid']:
            continue
            
        # Analyze
        results = analyze_model(model['file'])
        
        # AI evaluates results
        if results['weight'] < best_weight and meets_constraints(results):
            best_design = params
            best_weight = results['weight']
    
    return best_design
```

### Example 2: Design Space Exploration
```python
def explore_design_space(constraints):
    designs = []
    
    # AI generates design variants
    for params in ai_generate_design_variants(constraints):
        try:
            model = generate_model(params)
            validation = validate_with_ai_feedback(model['file'])
            
            if validation['valid']:
                results = analyze_model(model['file'])
                designs.append({
                    'params': params,
                    'results': results,
                    'feasible': check_feasibility(results, constraints)
                })
        except Exception as e:
            # Log and continue - AI learns from failures
            log_design_failure(params, str(e))
    
    return designs
```

## API-Like Usage Patterns

### Command Chaining
```bash
# Pipeline processing for AI workflows
gz create --template beam --span 6.0 --output temp.json | \
gz validate --format json | \
gz analyze --format json --output results.json
```

### Status Codes for AI Decision Making
- `0`: Success
- `1`: Input validation error
- `2`: Model validation error  
- `3`: Analysis convergence error
- `4`: File I/O error

### Environment Variables for AI
```bash
export GAZELLE_OUTPUT_FORMAT=json
export GAZELLE_PROGRESS_FORMAT=structured
export GAZELLE_LOG_LEVEL=info
export GAZELLE_UNITS=SI
```

## Integration Benefits

1. **Structured Data**: All I/O in JSON for easy parsing
2. **Type Safety**: F# prevents unit mixing disasters
3. **Reliable Analysis**: Consistent results for iteration
4. **Error Reporting**: Detailed, structured error information
5. **Batch Processing**: Support for AI workloads
6. **Template System**: Consistent model generation
7. **Validation Pipeline**: AI can validate before analysis

This makes Gazelle ideal for:
- Generative design workflows
- Optimization algorithms  
- Design space exploration
- Automated structural analysis
- AI-driven engineering tools