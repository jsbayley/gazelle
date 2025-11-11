# 🦌 Gazelle Architecture Documentation

This document provides comprehensive Mermaid diagrams illustrating Gazelle's architecture, workflows, and design patterns.

## Table of Contents

- [System Architecture](#system-architecture)
- [Daemon Service Architecture](#daemon-service-architecture)
- [Type Safety System](#type-safety-system)
- [Analysis Workflow](#analysis-workflow)
- [Material Domain Model](#material-domain-model)
- [Client Ecosystem](#client-ecosystem)
- [Development Roadmap](#development-roadmap)

---

## System Architecture

```mermaid
graph TB
    subgraph "Client Layer"
        CLI[CLI Client<br/>gazelle analyze]
        Python[Python Bindings<br/>import gazelle]
        Web[Web Interface<br/>Browser Dashboard]
        API[External Tools<br/>REST/HTTP]
    end
    
    subgraph "Service Layer"
        HTTP[HTTP/gRPC API<br/>localhost:3000]
        Session[Session Manager<br/>Multi-tenant]
        Auth[Authentication<br/>& Authorization]
    end
    
    subgraph "Core Engine"
        Daemon[Gazelle Daemon<br/>Rust Service]
        Analysis[Analysis Engine<br/>Static/Modal/Dynamic]
        Materials[Materials Library<br/>Type-Safe Concrete]
        Solver[Sparse Solvers<br/>nalgebra + faer]
    end
    
    subgraph "Data Layer"
        Models[Model Storage<br/>JSON/YAML]
        Results[Results Cache<br/>Session-based]
        Config[Configuration<br/>TOML/ENV]
    end
    
    CLI --> HTTP
    Python --> HTTP
    Web --> HTTP
    API --> HTTP
    
    HTTP --> Session
    HTTP --> Auth
    Session --> Daemon
    
    Daemon --> Analysis
    Daemon --> Materials
    Analysis --> Solver
    
    Daemon --> Models
    Daemon --> Results
    Daemon --> Config
    
    classDef client fill:#e1f5fe
    classDef service fill:#f3e5f5
    classDef core fill:#e8f5e8
    classDef data fill:#fff3e0
    
    class CLI,Python,Web,API client
    class HTTP,Session,Auth service
    class Daemon,Analysis,Materials,Solver core
    class Models,Results,Config data
```

---

## Daemon Service Architecture

```mermaid
sequenceDiagram
    participant C as Client
    participant D as Gazelle Daemon
    participant S as Session Manager
    participant A as Analysis Engine
    participant DB as Data Store
    
    Note over C,DB: Analysis Session Lifecycle
    
    C->>+D: POST /sessions
    D->>+S: Create Session
    S->>+DB: Store Model
    DB-->>-S: Session ID
    S-->>-D: Session Created
    D-->>-C: 201 Created {session_id}
    
    Note over C,DB: Analysis Execution
    
    C->>+D: POST /sessions/{id}/analyze
    D->>+S: Get Session
    S->>+DB: Load Model
    DB-->>-S: Model Data
    S-->>-D: Session Retrieved
    
    D->>+A: Execute Analysis
    A->>A: Solve Linear System
    A-->>-D: Analysis Results
    
    D->>+S: Store Results
    S->>+DB: Cache Results
    DB-->>-S: Stored
    S-->>-D: Results Cached
    
    D-->>-C: 200 OK {results}
    
    Note over C,DB: Session Management
    
    C->>+D: GET /sessions
    D->>+S: List Sessions
    S->>+DB: Query Sessions
    DB-->>-S: Session List
    S-->>-D: Sessions Retrieved
    D-->>-C: 200 OK [sessions]
    
    C->>+D: DELETE /sessions/{id}
    D->>+S: Delete Session
    S->>+DB: Remove Session
    DB-->>-S: Deleted
    S-->>-D: Session Deleted
    D-->>-C: 200 OK
```

---

## Type Safety System

```mermaid
classDiagram
    class Quantity~U~ {
        +value: f64
        +unit_symbol() String
        +new(value: f64) Self
    }
    
    class Length {
        <<type alias>>
        Quantity~Millimeter~
    }
    
    class Force {
        <<type alias>>
        Quantity~Kilonewton~
    }
    
    class Stress {
        +value: f64
        +mpa() f64
        +n_per_mm2() f64
        +new(value: f64) Self
    }
    
    class Area {
        <<enumeration>>
        +CrossSectional(f64)
        +Surface(f64)
        +cross_sectional(f64) Area
        +surface(f64) Area
        +value() f64
    }
    
    class Unit {
        <<trait>>
        +SYMBOL: &str
    }
    
    class Millimeter {
        +SYMBOL = "mm"
    }
    
    class Kilonewton {
        +SYMBOL = "kN"
    }
    
    Unit <|.. Millimeter
    Unit <|.. Kilonewton
    Quantity --> Unit : parameterized by
    Length --> Quantity : type alias
    Force --> Quantity : type alias
    
    note for Area "Prevents mixing cross-sectional\nand surface areas"
    note for Quantity "Compile-time unit checking\nprevents Mars Climate Orbiter disasters"
```

---

## Analysis Workflow

```mermaid
flowchart TD
    Start([Start Analysis]) --> LoadModel[Load Structural Model]
    LoadModel --> ValidateModel{Validate Model}
    ValidateModel -->|Invalid| ValidationError[Validation Error]
    ValidateModel -->|Valid| ProcessMaterials[Process Materials]
    
    ProcessMaterials --> CreateConcrete[Create Type-Safe Concrete]
    CreateConcrete --> ValidateAge{Age ≥ 3 days?}
    ValidateAge -->|No| AgeError[Age Validation Error]
    ValidateAge -->|Yes| CalculateProperties[Calculate Material Properties]
    
    CalculateProperties --> AssembleMatrix[Assemble Global Matrix]
    AssembleMatrix --> CheckDOF[Check DOF Mapping]
    CheckDOF --> ApplyConstraints[Apply Boundary Conditions]
    
    ApplyConstraints --> DetectSingularity{Matrix Singular?}
    DetectSingularity -->|Yes| AutoConstrain[Apply Auto-constraints]
    AutoConstrain --> DetectSingularity
    DetectSingularity -->|No| ChooseAnalysis{Analysis Type}
    
    ChooseAnalysis -->|Static| StaticSolver[Static Linear Solver]
    ChooseAnalysis -->|Modal| ModalSolver[Eigenvalue Solver]
    ChooseAnalysis -->|Dynamic| DynamicSolver[Time Integration]
    
    StaticSolver --> ProcessResults[Process Results]
    ModalSolver --> ProcessResults
    DynamicSolver --> ProcessResults
    
    ProcessResults --> ValidateResults{Results Valid?}
    ValidateResults -->|No| SolverError[Convergence Error]
    ValidateResults -->|Yes| FormatOutput[Format Output]
    
    FormatOutput --> ChooseFormat{Output Format}
    ChooseFormat -->|JSON| JSONOutput[JSON Serialization]
    ChooseFormat -->|YAML| YAMLOutput[YAML Serialization]
    ChooseFormat -->|VTK| VTKOutput[VTK Export]
    
    JSONOutput --> Success([Analysis Complete])
    YAMLOutput --> Success
    VTKOutput --> Success
    
    ValidationError --> End([End])
    AgeError --> End
    SolverError --> End
    
    classDef process fill:#e8f5e8
    classDef decision fill:#fff3e0
    classDef error fill:#ffebee
    classDef success fill:#e1f5fe
    
    class LoadModel,ProcessMaterials,CreateConcrete,CalculateProperties,AssembleMatrix,CheckDOF,ApplyConstraints,AutoConstrain,StaticSolver,ModalSolver,DynamicSolver,ProcessResults,FormatOutput,JSONOutput,YAMLOutput,VTKOutput process
    class ValidateModel,ValidateAge,DetectSingularity,ChooseAnalysis,ValidateResults,ChooseFormat decision
    class ValidationError,AgeError,SolverError error
    class Start,Success,End success
```

---

## Material Domain Model

```mermaid
erDiagram
    Concrete {
        Age age
        Aggregate aggregate
        Cement cement
        CylinderStrength grade
        WeightClass weight_class
    }
    
    CylinderStrength {
        UkConcreteGrade grade
    }
    
    UkConcreteGrade {
        string Fck12
        string Fck16
        string Fck20
        string Fck25
        string Fck30
        string Fck35
        string Fck40
        string Fck45
        string Fck50
        string Fck55
        string Fck60
        string Fck70
        string Fck80
        string Fck90
    }
    
    Aggregate {
        string Basalt
        string Limestone
        string Sandstone
        string Quartzite
    }
    
    Cement {
        string ClassR
        string ClassN
        string ClassS
    }
    
    WeightClass {
        string NormalWeight
    }
    
    Age {
        f64 days
    }
    
    Properties {
        f64 fck
        f64 fcm
        f64 fctm
        f64 ecm
        f64 density
    }
    
    Concrete ||--|| CylinderStrength : has
    CylinderStrength ||--|| UkConcreteGrade : contains
    Concrete ||--|| Aggregate : has
    Concrete ||--|| Cement : has
    Concrete ||--|| WeightClass : has
    Concrete ||--|| Age : has
    Concrete ||--o{ Properties : calculates
    
    Properties ||--|| Aggregate : "affects Ecm"
    Properties ||--|| Cement : "affects time-dependent"
    Properties ||--|| Age : "determines strength"
```

---

## Client Ecosystem

```mermaid
graph LR
    subgraph "Development Tools"
        VS[VS Code Extension<br/>Syntax Highlighting]
        Debug[Debug Tools<br/>Matrix Inspection]
        Test[Test Runner<br/>Property Testing]
    end
    
    subgraph "Python Ecosystem"
        Jupyter[Jupyter Notebooks<br/>Interactive Analysis]
        NumPy[NumPy Integration<br/>Array Operations]
        Pandas[Pandas DataFrames<br/>Results Processing]
        Plot[Matplotlib/Plotly<br/>Visualization]
    end
    
    subgraph "CLI Tools"
        Shell[Shell Scripts<br/>Batch Processing]
        CI[CI/CD Pipelines<br/>Automated Testing]
        Docker[Docker Containers<br/>Cloud Deployment]
    end
    
    subgraph "Web Applications"
        React[React Dashboard<br/>Interactive UI]
        API[REST API Clients<br/>Custom Tools]
        Mobile[Mobile Apps<br/>Field Inspection]
    end
    
    subgraph "Gazelle Daemon"
        Core[Core Engine<br/>localhost:3000]
    end
    
    VS --> Core
    Debug --> Core
    Test --> Core
    
    Jupyter --> Core
    NumPy --> Core
    Pandas --> Core
    Plot --> Core
    
    Shell --> Core
    CI --> Core
    Docker --> Core
    
    React --> Core
    API --> Core
    Mobile --> Core
    
    classDef dev fill:#e1f5fe
    classDef python fill:#fff3e0
    classDef cli fill:#e8f5e8
    classDef web fill:#f3e5f5
    classDef core fill:#ffebee
    
    class VS,Debug,Test dev
    class Jupyter,NumPy,Pandas,Plot python
    class Shell,CI,Docker cli
    class React,API,Mobile web
    class Core core
```

---

## Development Roadmap

```mermaid
timeline
    title Gazelle Development Roadmap
    
    2024 Q1-Q2 : Foundation Phase
                : Core Engine Implementation ✅
                : Type Safety System ✅
                : Test Suite & Validation ✅
    
    2024 Q3-Q4 : Architecture Phase  
                : Daemon Service Design ✅
                : HTTP API Implementation 🚧
                : Session Management 🚧
    
    2025 Q1    : Ecosystem Phase
                : Python Bindings 📋
                : CLI Enhancement 📋
                : Web Interface 📋
    
    2025 Q2-Q3 : Materials Phase
                : Steel Materials Library 📋
                : Design Code Plugins 📋
                : Advanced Concrete Models 📋
    
    2025 Q4    : Analysis Phase
                : Nonlinear Analysis 📋
                : Dynamic Analysis Enhancement 📋
                : Performance Optimization 📋
```

---

## REST API Endpoints

```mermaid
graph TD
    subgraph "Session Management"
        POST_S[POST /sessions<br/>Create analysis session]
        GET_S[GET /sessions<br/>List active sessions]
        GET_SI[GET /sessions/{id}<br/>Get session details]
        DEL_S[DELETE /sessions/{id}<br/>Delete session]
    end
    
    subgraph "Analysis Operations"
        POST_A[POST /sessions/{id}/analyze<br/>Run analysis]
        GET_R[GET /sessions/{id}/results<br/>Get cached results]
        POST_M[POST /sessions/{id}/model<br/>Update model]
    end
    
    subgraph "System Status"
        GET_ST[GET /status<br/>Daemon status]
        GET_H[GET /health<br/>Health check]
        GET_M[GET /metrics<br/>Performance metrics]
    end
    
    subgraph "Utilities"
        POST_V[POST /validate<br/>Validate model]
        POST_C[POST /convert<br/>Format conversion]
        GET_E[GET /examples<br/>Example models]
    end
    
    Client --> POST_S
    Client --> GET_S
    Client --> GET_SI
    Client --> DEL_S
    Client --> POST_A
    Client --> GET_R
    Client --> POST_M
    Client --> GET_ST
    Client --> GET_H
    Client --> GET_M
    Client --> POST_V
    Client --> POST_C
    Client --> GET_E
    
    classDef session fill:#e1f5fe
    classDef analysis fill:#e8f5e8
    classDef system fill:#fff3e0
    classDef util fill:#f3e5f5
    
    class POST_S,GET_S,GET_SI,DEL_S session
    class POST_A,GET_R,POST_M analysis
    class GET_ST,GET_H,GET_M system
    class POST_V,POST_C,GET_E util
```

---

## Error Handling Flow

```mermaid
stateDiagram-v2
    [*] --> Processing
    Processing --> ValidatingModel
    
    ValidatingModel --> InvalidModel : Validation fails
    ValidatingModel --> ValidModel : Validation succeeds
    
    ValidModel --> MaterialProcessing
    MaterialProcessing --> InvalidMaterial : Material validation fails
    MaterialProcessing --> ValidMaterial : Material validation succeeds
    
    ValidMaterial --> MatrixAssembly
    MatrixAssembly --> SingularMatrix : Matrix is singular
    MatrixAssembly --> ValidMatrix : Matrix is valid
    
    SingularMatrix --> AutoConstraints : Apply auto-constraints
    AutoConstraints --> MatrixAssembly : Retry assembly
    AutoConstraints --> UnrecoverableError : Max attempts reached
    
    ValidMatrix --> Solving
    Solving --> ConvergenceFailure : Solver diverges
    Solving --> SolutionFound : Solver converges
    
    SolutionFound --> ResultsProcessing
    ResultsProcessing --> Success : Results validated
    ResultsProcessing --> InvalidResults : Results invalid
    
    InvalidModel --> ErrorResponse
    InvalidMaterial --> ErrorResponse
    UnrecoverableError --> ErrorResponse
    ConvergenceFailure --> ErrorResponse
    InvalidResults --> ErrorResponse
    
    Success --> [*]
    ErrorResponse --> [*]
```

---

This comprehensive Mermaid documentation provides visual clarity for:

1. **System Architecture** - Overall component relationships
2. **Daemon Service** - HTTP API interaction patterns
3. **Type Safety** - Compile-time safety mechanisms
4. **Analysis Workflow** - Engineering process flow
5. **Material Domain** - Engineering material modeling
6. **Client Ecosystem** - Multi-language integration
7. **Development Roadmap** - Project timeline and priorities
8. **API Endpoints** - REST interface organization
9. **Error Handling** - Robust failure management

These diagrams can be embedded directly in GitHub README files and will render automatically, providing immediate visual understanding of Gazelle's architecture and workflows.