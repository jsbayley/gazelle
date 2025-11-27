# ETABS API Reference for Gazelle Integration

## Key Concepts for F# Implementation

### Core API Structure
- **ETABSv1.Helper** - Main COM helper class for creating/connecting to ETABS instances
- **SapObject** - Primary application object (ETABSv1.cOAPI)
- **SapModel** - Core model interface containing all structural modeling methods

### Connection Patterns
```fsharp
// Pattern from CHM documentation:
let etabsType = Type.GetTypeFromProgID("ETABSv1.Helper")
let helper = Activator.CreateInstance(etabsType) 
let app = helper.CreateObject("ETABSv1.SapObject")  // New instance
// OR
let app = helper.GetObject("ETABSv1.SapObject")     // Existing instance
```

### Key API Requirements
- **.NET 4.7.1** compatibility (documentation specifies this version)
- **ETABS Installation Required** on both development and deployment machines
- **COM Interop** - All calls go through COM automation interface
- **Windows Only** - COM interop limits to Windows platform

### Critical SapModel Methods (from CHM)
- `InitializeNewModel(eUnits)` - Set up clean model environment
- `File.NewBlank()` - Create blank model canvas
- `File.OpenFile(path)` - Open existing model
- `File.Save()` / `File.Save(path)` - Save model
- `View.RefreshView()` - Update display

### Units System (eUnits enumeration)
- `kN_mm_C` - Kilonewtons, millimeters, Celsius
- `kN_m_C` - Kilonewtons, meters, Celsius  
- `N_mm_C` - Newtons, millimeters, Celsius
- Additional unit systems documented in API

### Error Handling Patterns
- Most methods return `int` status codes
- 0 = Success, non-zero = Error
- Use COM exception handling with `try/catch`

## Implementation Priorities for Gazelle

### Phase 1: Connection & Model Management
- [x] ETABSv1.Helper instantiation  
- [x] CreateObject/GetObject methods
- [x] Model initialization with units
- [ ] Complete error code mapping

### Phase 2: Geometry Creation
- [ ] Point/coordinate methods
- [ ] Frame/beam element creation  
- [ ] Material property definition
- [ ] Section property assignment

### Phase 3: Analysis & Results
- [ ] Load case definition
- [ ] Analysis execution
- [ ] Results extraction (forces, displacements)
- [ ] Export to Gazelle JSON format

## Files to Keep from CHM Documentation

### Essential Structure Files
- `CSI API ETABS v1.hhc` - Table of contents mapping
- `CSI API ETABS v1.hhk` - Keyword index for API lookup

### Key Documentation Pages (by title from hhc)
- Introduction - Basic API concepts
- Attaching to Manually Started Instance 
- Programming Concepts Used in CSi API
- Interactive Database concepts
- Plugin development information

### Remove from Extraction
- All individual HTML files with GUID names (1000+ files)
- Binary CHM artifacts (#IDXHDR, #ITBITS, etc.)
- CSS/JS styling files (not needed for API reference)
- Icon files (presentation only)

## Mapping to Current Gazelle Code

Your existing code in `/workspaces/gazelle/io/code/ETABS/ETABS.fs` already implements the core patterns correctly:

```fsharp
// ✅ Matches CHM documentation exactly
type Helper =
  | V17 of ETABSv17.cHelper
  | V19 of ETABSv1.cHelper

// ✅ Correct COM instantiation pattern  
let createHelper (v: Version) : Helper =
  match v with
  | Version.V17 -> Helper.V17(ETABSv17.Helper() :> ETABSv17.cHelper)
  | Version.V19 -> Helper.V19(ETABSv1.Helper() :> ETABSv1.cHelper)

// ✅ Proper model initialization sequence
let initialiseNewModel (s: SAPModel) : unit =
  match s with
  | V17 s -> s.InitializeNewModel(ETABSv17.eUnits.kN_mm_C)
  | V19 s -> s.InitializeNewModel(ETABSv1.eUnits.kN_mm_C)
```

## Next Development Steps

1. **Extract Method Signatures** - Parse CHM for complete method documentation
2. **Units Integration** - Map ETABS eUnits to Gazelle Units system  
3. **Type-Safe Wrappers** - Create F# discriminated unions for ETABS enums
4. **Error Handling** - Implement comprehensive error code interpretation
5. **Results Extraction** - Build bi-directional JSON conversion

The CHM documentation provides the complete specification needed to implement a full-featured ETABS integration.