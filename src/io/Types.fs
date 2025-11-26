namespace Gazelle.IO

// ========================================
// TYPES
// ========================================

// Units
[<Measure>]
type mm

// IO Types
type FilePath = FilePath of string
type DirectoryPath = DirectoryPath of string

// Unvalidated Input Types (from user config before validation)
type IntegerRange = { Start: int; End: int }

type UnvalidatedStoreyGroup =
  { StoreyRange: IntegerRange
    TypicalStoreyHeight: float
    SlabThickness: float
    SlabVertices: seq<float[]>
    ColumnCoordinatePairs: seq<float[]> }

type BuildingBlueprint =
  { BaseElevation: float
    StoreyGroups: seq<UnvalidatedStoreyGroup> }

// Validated Domain Types
type Name = Name of string
type PositiveInt = PositiveInt of int
type PositiveFloat = PositiveFloat of float<mm>
type Height = Height of PositiveFloat
type Elevation = Elevation of float<mm>

type Point2D = { X: float<mm>; Y: float<mm> }

type Point3D =
  { X: float<mm>
    Y: float<mm>
    Z: float<mm> }

type PositiveIntegerRange =
  { Start: PositiveInt; End: PositiveInt }

type StoreyGroup =
  { StoreyRange: PositiveIntegerRange
    TypicalStoreyHeight: Height
    SlabThickness: PositiveFloat
    SlabVertices: seq<Point2D>
    ColumnCoordinatePairs: seq<Point2D> }

type MasterStorey =
  { Name: Name
    Elevation: Elevation
    Height: Height }

type SimilarStorey =
  { Name: Name
    Elevation: Elevation
    Height: Height
    SimilarTo: MasterStorey }

type Storey =
  | MasterStorey of MasterStorey
  | SimilarStorey of SimilarStorey

type Slab =
  { Name: Name
    Elevation: Elevation
    Thickness: PositiveFloat
    Vertices: Point3D list }

type Frame =
  { Name: Name
    Start: Point3D
    End: Point3D }

type Column = Column of Frame

type Building =
  { Storeys: Storey list
    Slabs: Slab list
    Columns: Column list }

// Error Types
type IOError =
  | BadUserInput
  | UnsupportedVersion of string
  | PathError of string
  | FileExtensionError of string
  | DeserializationError of string

type RangeError =
  | NonPositiveStartValue
  | NonPositiveEndValue
  | StartGreaterThanEnd
  | NonConsecutiveRange

type HeightError =
  | HeightLessThanZero
  | HeightEqualToZero
  | InvalidHeight

type ModelValidationResult =
  | ValidModel
  | StoreyHeightError of HeightError
  | StoreyRangeError of RangeError

// ========================================
// ERROR HANDLING
// ========================================

[<RequireQualifiedAccess>]
module IOError =

  let getAsString (e: IOError) : string =
    match e with
    | BadUserInput -> "Bad User Input."
    | UnsupportedVersion msg -> $"Unsupported Version: {msg}."
    | PathError msg -> $"Path Error: {msg}."
    | FileExtensionError msg -> $"File Extension Error: {msg}."
    | DeserializationError msg -> $"Deserialization Error: {msg}."

  let print (error: IOError) =
    let msg = error |> getAsString
    System.Console.ForegroundColor <- System.ConsoleColor.Red
    System.Console.WriteLine($"❌ IOError: {msg}")
    System.Console.ResetColor()

[<RequireQualifiedAccess>]
module ModelValidationResult =

  module RangeError =

    let getAsString (e: RangeError) : string =
      match e with
      | NonPositiveStartValue ->
        "Non-positive start value given for storey range."
      | NonPositiveEndValue -> "Non-positive end value given for storey range."
      | StartGreaterThanEnd -> "Range start value is greater than end value."
      | NonConsecutiveRange -> "Adjacent storey ranges are non-consecutive."

  module HeightError =

    let getAsString (e: HeightError) : string =
      match e with
      | HeightLessThanZero -> "Storey height is less than zero."
      | HeightEqualToZero -> "Storey height is equal to zero."
      | InvalidHeight -> "Invalid storey height."

  let getAsString (e: ModelValidationResult) : string =
    match e with
    | ValidModel -> "Valid Model."
    | StoreyRangeError e -> RangeError.getAsString e
    | StoreyHeightError e -> HeightError.getAsString e

  let printErrors (errors: seq<ModelValidationResult>) : unit =
    errors
    |> Seq.map getAsString
    |> Seq.filter (fun e -> e <> "Valid Model.")
    |> Seq.iter (fun msg ->
      System.Console.ForegroundColor <- System.ConsoleColor.Red
      System.Console.WriteLine($"❌ Model Validation Error: {msg}")
      System.Console.ResetColor())

// ========================================
// UTILITY FUNCTIONS
// ========================================

[<RequireQualifiedAccess>]
module Unwrap =

  let name (Name s) : string = s
  let filePath (FilePath s) : string = s
  let directoryPath (DirectoryPath s) : string = s
  let positiveInt (PositiveInt i) : int = i
  let positiveFloat (PositiveFloat f) : float<mm> = f
  let height (Height h) : PositiveFloat = h
  let elevation (Elevation e) : float<mm> = e