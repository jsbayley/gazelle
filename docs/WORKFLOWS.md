# 🦌 Gazelle Quick Start Workflows

This document provides visual workflows for common Gazelle usage patterns.

## User Journey: First Analysis

```mermaid
graph LR
    subgraph "Getting Started 🚀"
        Install[Install Gazelle<br/>⭐⭐⭐⭐⭐]
        Daemon[Start daemon<br/>⭐⭐⭐⭐]
        Model[Create model<br/>⭐⭐⭐]
    end
    
    subgraph "Running Analysis 🔬"
        Upload[Upload model<br/>⭐⭐⭐⭐]
        Analyze[Run analysis<br/>⭐⭐⭐⭐⭐]
        Results[Get results<br/>⭐⭐⭐⭐⭐]
    end
    
    subgraph "Understanding Results 📊"
        Displace[View displacements<br/>⭐⭐⭐⭐]
        Stress[Check stresses<br/>⭐⭐⭐⭐]
        Export[Export to CAD<br/>⭐⭐⭐]
    end
    
    Install --> Daemon
    Daemon --> Model
    Model --> Upload
    Upload --> Analyze
    Analyze --> Results
    Results --> Displace
    Displace --> Stress
    Stress --> Export
    
    classDef start fill:#e1f5fe
    classDef analysis fill:#e8f5e8
    classDef results fill:#fff3e0
    
    class Install,Daemon,Model start
    class Upload,Analyze,Results analysis
    class Displace,Stress,Export results
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
graph TB
    subgraph "Users"
        Engineer[Structural Engineer<br/>Creates and analyzes models]
        Student[Engineering Student<br/>Learning structural analysis]
        Researcher[Academic Researcher<br/>Developing new methods]
    end
    
    subgraph "Gazelle System"
        Core[Gazelle Engine<br/>Fast structural analysis daemon]
    end
    
    subgraph "External Systems"
        CAD[CAD Software<br/>AutoCAD, Rhino, etc.]
        FEA[FEA Tools<br/>Existing analysis software]  
        Cloud[Cloud Services<br/>AWS, Azure, GCP]
    end
    
    Engineer -->|CLI, Python| Core
    Student -->|Jupyter notebooks| Core
    Researcher -->|API, custom tools| Core
    
    Core <-->|Import/Export<br/>DXF, STEP| CAD
    Core <-->|Compare<br/>Results validation| FEA
    Core -->|Deploy<br/>Container services| Cloud
    
    classDef user fill:#e1f5fe
    classDef system fill:#e8f5e8
    classDef external fill:#fff3e0
    
    class Engineer,Student,Researcher user
    class Core system
    class CAD,FEA,Cloud external
```

## Development Workflow

```mermaid
graph TD
    Start([Initial commit]) --> Materials[Feature: Materials]
    Materials --> ConcreteMat[Add concrete materials]
    ConcreteMat --> SteelMat[Add steel materials]
    SteelMat --> MergeMat[Merge to main]
    
    Start --> Analysis[Feature: Analysis]
    Analysis --> Modal[Implement modal analysis]
    Modal --> Nonlinear[Add nonlinear solver]
    Nonlinear --> MergeAnal[Merge to main]
    
    MergeMat --> Python[Feature: Python bindings]
    MergeAnal --> Python
    Python --> PyO3[PyO3 integration]
    PyO3 --> PyTests[Python test suite]
    PyTests --> MergePy[Merge to main]
    
    MergePy --> Release[Release v1.0.0]
    Release --> Hotfix[Hotfix: Stability]
    Hotfix --> Fix[Fix convergence issue]
    Fix --> Patch[Release v1.0.1]
    
    classDef feature fill:#e1f5fe
    classDef commit fill:#e8f5e8
    classDef release fill:#ffebee
    
    class Materials,Analysis,Python,Hotfix feature
    class ConcreteMat,SteelMat,Modal,Nonlinear,PyO3,PyTests,Fix commit
    class Release,Patch release
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