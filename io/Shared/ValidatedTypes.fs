namespace Gazelle.IO

// Units.
[<Measure>]
type mm

// IO.
type FilePath = FilePath of string
type DirectoryPath = DirectoryPath of string

// Domain Model.
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
