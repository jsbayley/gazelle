# ETABS API Table of Contents (Key Sections)

## From CSI API ETABS v1.hhc Analysis

### Core Documentation Structure

**Essential Concepts:**
- Introduction - Basic API overview and .NET 4.7.1 requirements
- Release Notes - Version-specific changes and compatibility
- Key Concepts - Fundamental programming patterns

**Connection & Lifecycle:**
- Attaching to Manually Started Instance - GetObject() pattern
- Launching Installed Version Automatically - CreateObject() pattern  
- Programming Concepts Used in CSi API - COM interop fundamentals
- Information for Plugin Developers - Extension development

**Data Management:**
- Interactive Database - Model data structure concepts

### API Structure (Inferred from CHM Navigation)

The CHM contains detailed documentation for:

1. **Helper Classes**
   - ETABSv1.Helper methods
   - Version-specific instantiation

2. **Core Objects** 
   - SapObject (main application)
   - SapModel (model interface)
   - File operations
   - View management

3. **Model Building**
   - Coordinate systems
   - Materials and sections  
   - Structural elements
   - Load definitions

4. **Analysis & Results**
   - Analysis execution
   - Results extraction
   - Report generation

### Implementation Strategy

The table of contents reveals a comprehensive API covering all aspects of structural modeling. The key for Gazelle integration is to:

1. **Focus on Core Methods** - Model creation, element definition, analysis execution
2. **Map Units Consistently** - ETABS eUnits to Gazelle Units system
3. **Handle COM Interop Properly** - Error codes, marshaling, resource management  
4. **Provide Type Safety** - F# wrappers around COM interfaces

### Documentation Value Assessment

**High Value (Keep):**
- Table of contents structure (hhc file)
- Keyword index (hhk file) 
- Reference summary created

**Low Value (Removed):**
- 1000+ individual HTML files with cryptic GUID names
- CSS/JavaScript styling files
- Binary CHM artifacts
- Icon and media files

The essential API structure information has been preserved while removing the presentation artifacts that don't contribute to development understanding.