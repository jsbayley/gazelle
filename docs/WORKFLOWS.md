# 🦌 Gazelle Quick Start Workflows

This document provides visual workflows for common Gazelle usage patterns.

## User Journey: First Analysis

```mermaid
journey
    title My First Structural Analysis with Gazelle
    section Getting Started
      Install Gazelle: 5: Me
      Start daemon: 4: Me
      Create model: 3: Me
    section Running Analysis
      Upload model: 4: Me
      Run analysis: 5: Me, Gazelle
      Get results: 5: Me, Gazelle
    section Understanding Results
      View displacements: 4: Me
      Check stresses: 4: Me
      Export to CAD: 3: Me
```

## CLI Usage Flow

```mermaid
flowchart LR
    Start([Start]) --> Install[cargo install gazelle]
    Install --> Daemon[gazelle daemon --start]
    Daemon --> Create[gazelle create truss]
    Create --> Edit[Edit model.json]
    Edit --> Analyze[gazelle analyze model.json]
    Analyze --> Results[View results.json]
    Results --> Export[gazelle export --format vtk]
    Export --> Success([Analysis Complete])
```

## Python Integration

```mermaid
sequenceDiagram
    participant User as Python User
    participant Lib as gazelle library
    participant Daemon as Gazelle Daemon
    participant Engine as Analysis Engine
    
    User->>Lib: import gazelle
    User->>Lib: gz = gazelle.Gazelle()
    Lib->>Daemon: Connect to localhost:3000
    
    User->>Lib: gz.create_truss(span=10.0)
    Lib->>Daemon: POST /sessions
    Daemon-->>Lib: session_id
    
    User->>Lib: results = gz.analyze()
    Lib->>Daemon: POST /sessions/{id}/analyze
    Daemon->>Engine: Execute analysis
    Engine-->>Daemon: Analysis results
    Daemon-->>Lib: JSON results
    Lib-->>User: Python objects
    
    User->>Lib: results.plot()
    Note over User,Lib: matplotlib visualization
```

## Multi-Client Architecture

```mermaid
C4Context
    title Gazelle Multi-Client System Context
    
    Person(engineer, "Structural Engineer", "Creates and analyzes structural models")
    Person(student, "Engineering Student", "Learning structural analysis")
    Person(researcher, "Academic Researcher", "Developing new analysis methods")
    
    System(gazelle, "Gazelle Engine", "Fast structural analysis daemon")
    
    System_Ext(cad, "CAD Software", "AutoCAD, Rhino, etc.")
    System_Ext(fea, "FEA Tools", "Existing analysis software")
    System_Ext(cloud, "Cloud Services", "AWS, Azure, GCP")
    
    Rel(engineer, gazelle, "Uses", "CLI, Python")
    Rel(student, gazelle, "Uses", "Jupyter notebooks")
    Rel(researcher, gazelle, "Uses", "API, custom tools")
    
    BiRel(gazelle, cad, "Import/Export", "DXF, STEP")
    BiRel(gazelle, fea, "Compare", "Results validation")
    Rel(gazelle, cloud, "Deploy", "Container services")
```

## Development Workflow

```mermaid
gitgraph
    commit id: "Initial commit"
    branch feature/materials
    checkout feature/materials
    commit id: "Add concrete materials"
    commit id: "Add steel materials"
    checkout main
    merge feature/materials
    
    branch feature/analysis
    checkout feature/analysis
    commit id: "Implement modal analysis"
    commit id: "Add nonlinear solver"
    checkout main
    merge feature/analysis
    
    branch feature/python-bindings
    checkout feature/python-bindings
    commit id: "PyO3 integration"
    commit id: "Python test suite"
    checkout main
    merge feature/python-bindings
    
    commit id: "Release v1.0.0"
    
    branch hotfix/stability
    checkout hotfix/stability
    commit id: "Fix convergence issue"
    checkout main
    merge hotfix/stability
    commit id: "Release v1.0.1"
```

## Testing Strategy

```mermaid
pie title Test Coverage Distribution
    "Unit Tests" : 45
    "Integration Tests" : 25
    "Property Tests" : 15
    "Documentation Tests" : 10
    "Benchmark Tests" : 5
```

## Error Recovery Flow

```mermaid
flowchart TD
    Error[Analysis Error] --> CheckType{Error Type}
    
    CheckType -->|Validation| FixModel[Fix Model Structure]
    CheckType -->|Material| UpdateMaterial[Update Material Properties]
    CheckType -->|Solver| TuneSolver[Adjust Solver Settings]
    CheckType -->|System| CheckSetup[Check System Requirements]
    
    FixModel --> Retry[Retry Analysis]
    UpdateMaterial --> Retry
    TuneSolver --> Retry
    CheckSetup --> Retry
    
    Retry --> Success{Success?}
    Success -->|Yes| Results[View Results]
    Success -->|No| Support[Contact Support]
    
    Results --> Done[Analysis Complete]
    Support --> Done
```

This visual documentation helps users understand:

1. **User Journey** - The emotional experience of using Gazelle
2. **CLI Flow** - Step-by-step command line usage
3. **Python Integration** - How the Python bindings work
4. **Multi-Client** - System context and relationships
5. **Development** - How the project evolves
6. **Testing** - Quality assurance approach
7. **Error Recovery** - How to handle problems

These diagrams complement the technical architecture documentation and provide user-focused guidance.