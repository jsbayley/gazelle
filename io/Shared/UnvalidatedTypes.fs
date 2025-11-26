namespace Gazelle.IO

// DTO types specified in user config. file prior to model validation.
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
