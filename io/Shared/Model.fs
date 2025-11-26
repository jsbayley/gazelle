namespace Gazelle.IO

module Model =

  /// Accepts a sequence of validation functions to apply against a given input parameter.
  let private applyValidationChecks
    (predicates: seq<'T -> seq<ModelValidationResult>>)
    (input: 'T)
    : seq<ModelValidationResult> =
    predicates |> Seq.collect (fun validate -> validate input)

  /// Accepts a sequence of generic type and then (1) applies a given predicate function to
  /// each element in the sequence, and (2) tests whether the predicate is true for all elements.
  let private applyCheckToAll
    (predicate: 'T -> ModelValidationResult)
    (s: seq<'T>)
    : seq<ModelValidationResult> =
    s |> Seq.map predicate

  /// Cycles through ValidationResults in a given sequence to check if all are valid.
  let private allElementsAreValid (s: seq<ModelValidationResult>) : bool =
    s
    |> Seq.forall (fun s ->
      match s with
      | ValidModel -> true
      | _ -> false)

  [<RequireQualifiedAccess>]
  module PositiveInt =

    /// Attempts to initialise new <see cref="PositiveInt">.
    let tryCreate (x: int) =
      match x with
      | x when x <= 0 -> invalidArg $"{nameof (x)}" "Integer <= 0."
      | _ -> PositiveInt x

  [<RequireQualifiedAccess>]
  module PositiveFloat =

    /// Attempts to initialise new <see cref="PositiveFloat">.
    let tryCreate (x: float) =
      match x with
      | x when x <= 0.0 -> invalidArg $"{nameof (x)}" "Float <= 0."
      | _ -> PositiveFloat(x * 1.0<mm>)

  [<RequireQualifiedAccess>]
  module IntegerRange =

    /// Verifies that both the Start and End properties of the given <see cref="IntegerRange"> are positive.
    let startAndEndArePositive (r: IntegerRange) : ModelValidationResult =
      match r.Start, r.End with
      | s, _ when s <= 0 -> StoreyRangeError NonPositiveStartValue
      | _, e when e <= 0 -> StoreyRangeError NonPositiveEndValue
      | _ -> ValidModel

    /// Verifies that the End value of the <see cref="IntegerRange"> is greater than the Start value.
    let endIsGreaterThanStart (r: IntegerRange) : ModelValidationResult =
      match r.Start, r.End with
      | s, e when s >= e -> StoreyRangeError StartGreaterThanEnd
      | _ -> ValidModel

    /// Verifies that two adjacent <see cref="IntegerRange"> parameters are immediately consecutive.
    /// The Start value of the second Range should immediately follow the End value of the first.
    let rangesAreConsecutive
      (r1: IntegerRange)
      (r2: IntegerRange)
      : ModelValidationResult =
      match r2.Start, r1.End with
      | s, e when s - e = 1 -> ValidModel
      | _ -> StoreyRangeError NonConsecutiveRange

  [<RequireQualifiedAccess>]
  module PositiveIntegerRange =

    /// Creates a PositiveIntegerRange from PositiveInt start and end values.
    let create
      (rngStart: PositiveInt)
      (rngEnd: PositiveInt)
      : PositiveIntegerRange =
      { Start = rngStart; End = rngEnd }

  [<RequireQualifiedAccess>]
  module StoreyGroup =

    /// Verifies that both the Start and End properties of a Storey Group Range are positive.
    let rangesArePositive (s: UnvalidatedStoreyGroup) : ModelValidationResult =
      IntegerRange.startAndEndArePositive s.StoreyRange

    /// Verifies that, for a given range, the End value is greater than the Start value.
    let rangeEndGreaterThanStart
      (s: UnvalidatedStoreyGroup)
      : ModelValidationResult =
      IntegerRange.endIsGreaterThanStart s.StoreyRange

    /// (1) Accepts sequence of storey groups, (2) sorts the sequence by Storey Range Start value,
    /// (3) pairs up adjacent groups, and (4) verifies that adjacent groups are immediately consecutive.
    let allAdjacentGroupsAreConsecutive
      (groups: seq<UnvalidatedStoreyGroup>)
      : seq<ModelValidationResult> =
      groups
      |> Seq.sortBy (fun s -> s.StoreyRange.Start)
      |> Seq.pairwise
      |> Seq.map (fun (s1, s2) ->
        IntegerRange.rangesAreConsecutive s1.StoreyRange s2.StoreyRange)

    /// Verifies that a given typical storey height for a group is positive.
    let typicalHeightIsPositive
      (s: UnvalidatedStoreyGroup)
      : ModelValidationResult =
      let height = s.TypicalStoreyHeight

      match height with
      | h when h > 0.0 -> ValidModel
      | h when h = 0.0 -> StoreyHeightError HeightEqualToZero
      | h when h < 0.0 -> StoreyHeightError HeightLessThanZero
      | _ -> StoreyHeightError InvalidHeight

    /// Returns number of storeys in given Storey Range.
    let getLength (sg: StoreyGroup) : int =
      let s, e = Unwrap.positiveIntegerRange sg.StoreyRange
      (e - s) + 1

    /// Returns Storey Numbers for all storeys in range, corresponding to storey level.
    let getIds (sg: StoreyGroup) : seq<int> =
      let s, e = Unwrap.positiveIntegerRange sg.StoreyRange
      seq { for i in s..1..e -> i }

    /// Returns storey elevations, offset from a given baseElevation, for all storeys in group.
    let getElevations
      (baseElevation: Elevation)
      (sg: StoreyGroup)
      : seq<float<mm>> =
      let sgLength = (getLength sg - 1)
      let typHeight = Unwrap.heightAndPositiveFloat sg.TypicalStoreyHeight
      let lowerElev = Unwrap.elevation baseElevation + typHeight
      let upperElev = lowerElev + (float sgLength * typHeight)
      seq { for e in lowerElev..typHeight..upperElev -> e }

    /// Creates a single StoreyGroup from an UnvalidatedStoreyGroup.
    let create (s: UnvalidatedStoreyGroup) : StoreyGroup =
      let rngStart = PositiveInt.tryCreate s.StoreyRange.Start
      let rngEnd = PositiveInt.tryCreate s.StoreyRange.End
      let h = PositiveFloat.tryCreate s.TypicalStoreyHeight |> Height
      let range = PositiveIntegerRange.create rngStart rngEnd

      // WARNING: need to add slab validation!
      let slabVertices =
        s.SlabVertices
        |> Seq.map (fun (coord) ->
          { X = coord.[0] * 1.0<mm>
            Y = coord.[1] * 1.0<mm> })

      let slabThickness = PositiveFloat.tryCreate s.SlabThickness

      // WARNING: need to add column validation!
      let columnCoords =
        s.ColumnCoordinatePairs
        |> Seq.map (fun coords ->
          { X = coords.[0] * 1.0<mm>
            Y = coords.[1] * 1.0<mm> })

      { StoreyRange = range
        TypicalStoreyHeight = h
        SlabVertices = slabVertices
        SlabThickness = slabThickness
        ColumnCoordinatePairs = columnCoords }

    /// Creates a sequence of storey group instances from a sequence of unvalidated groups.
    let createMany (s: seq<UnvalidatedStoreyGroup>) : seq<StoreyGroup> =
      s |> Seq.map create

    [<RequireQualifiedAccess>]
    module Storeys =

      /// For a sequence of StoreyGroups, cycle through respective groups and instantiate all Master/Similar Storeys.
      let create
        (baseElevation: Elevation)
        (sg: seq<StoreyGroup>)
        : Storey list =
        let rec createAll
          (baseElevation: Elevation)
          (s: seq<StoreyGroup>)
          (created: Storey list)
          : Storey list =
          let createForSingleGroup
            (baseElevation: Elevation)
            (sg: StoreyGroup)
            : (MasterStorey * seq<SimilarStorey>) =

            let getElevations
              (baseElevation: Elevation)
              (sg: StoreyGroup)
              : (Elevation * seq<Elevation>) =
              let e = getElevations baseElevation sg |> Seq.sortDescending
              let masterElev = e |> Seq.head |> Elevation
              let similarElevs = e |> Seq.tail |> Seq.map Elevation
              masterElev, similarElevs

            let getIds (sg: StoreyGroup) : (int * seq<int>) =
              let ids = getIds sg |> Seq.sortDescending
              let masterId = ids |> Seq.head
              let similarIds = ids |> Seq.tail
              masterId, similarIds

            let createMaster
              (id: int)
              (e: Elevation)
              (h: Height)
              : MasterStorey =
              { Name = Name $"Storey{id}"
                Elevation = e
                Height = h }

            let createAllSimilar
              (ids: seq<int>)
              (e: seq<Elevation>)
              (h: Height)
              (masterRef: MasterStorey)
              : seq<SimilarStorey> =
              (ids, e)
              ||> Seq.map2 (fun id e ->
                { Name = Name $"Storey{id}"
                  Elevation = e
                  Height = h
                  SimilarTo = masterRef })

            let masterElev, similarElevs = getElevations baseElevation sg
            let masterId, similarIds = getIds sg
            let height = sg.TypicalStoreyHeight
            let masterStorey = createMaster masterId masterElev height

            let similarStoreys =
              createAllSimilar similarIds similarElevs height masterStorey

            masterStorey, similarStoreys

          let wrapSimilarStoreys (s: seq<SimilarStorey>) : Storey list =
            s |> Seq.map SimilarStorey |> Seq.toList

          let s = s |> Seq.toList

          match s with
          | [] -> created
          | currGroup :: remainingGroups ->
            let ms, ss = createForSingleGroup baseElevation currGroup
            let updatedElevation = ms.Elevation
            let created = (MasterStorey ms :: wrapSimilarStoreys ss) @ created
            createAll updatedElevation remainingGroups created

        createAll baseElevation sg []

    [<RequireQualifiedAccess>]
    module Slabs =

      // WARNING: This method needs to be checked/tested!
      // Validation methods required, for instance to check that provided
      // array coordinate pairs are no longer than 2 elements.
      // Also, to check shape of slab by looking for line intersections.
      let create (baseElevation: Elevation) (sg: seq<StoreyGroup>) : Slab list =
        let rec createAll
          (baseElevation: Elevation)
          (sg: seq<StoreyGroup>)
          (created: Slab list)
          : Slab list =
          let createForSingleGroup
            (baseElevation: Elevation)
            (sg: StoreyGroup)
            : seq<Slab> =

            let ids = getIds sg |> List.ofSeq

            let names =
              ids |> Seq.map (fun id -> Name $"Slab{id}") |> List.ofSeq

            // Temporarily adding storey height to each elevation
            // so that the lowest storey is 1, not base.
            let elevations =
              getElevations baseElevation sg
              |> List.ofSeq
              |> List.map (fun e ->
                e + (Unwrap.heightAndPositiveFloat sg.TypicalStoreyHeight))

            let vertices =
              [ for e in elevations ->
                  sg.SlabVertices
                  |> List.ofSeq
                  |> List.map (fun p2D -> { X = p2D.X; Y = p2D.Y; Z = e }) ]

            let slabs =
              (names, elevations, vertices)
              |||> Seq.map3 (fun n e v ->
                { Name = n
                  Elevation = Elevation e
                  Thickness = sg.SlabThickness
                  Vertices = v })

            slabs

          let sg = sg |> Seq.toList

          match sg with
          | [] -> created
          | currGroup :: remainingGroups ->
            let slabs =
              createForSingleGroup baseElevation currGroup |> Seq.toList

            let updatedElevation =
              slabs |> Seq.map (fun s -> s.Elevation) |> Seq.max

            let created = slabs @ created
            createAll updatedElevation remainingGroups created

        createAll baseElevation sg []

    [<RequireQualifiedAccess>]
    module Columns =

      // WARNING: This method needs to be checked/tested!
      let create
        (baseElevation: Elevation)
        (sg: seq<StoreyGroup>)
        : Column list =
        let rec createAll
          (baseElevation: Elevation)
          (sg: seq<StoreyGroup>)
          (created: Column list)
          : Column list =
          let createForSingleGroup
            (baseElevation: Elevation)
            (sg: StoreyGroup)
            : seq<Column> =

            let elevations =
              getElevations baseElevation sg |> List.ofSeq |> List.pairwise

            let columnStartAndEndPoints =
              [ for (s, e) in elevations ->
                  sg.ColumnCoordinatePairs
                  |> Seq.map (fun c ->
                    {| Start = { X = c.X; Y = c.Y; Z = s }
                       End = { X = c.X; Y = c.Y; Z = e } |}) ]
              |> Seq.concat
              |> List.ofSeq

            let names =
              [ for i in 1..1 .. columnStartAndEndPoints.Length ->
                  Name $"Column{i}" ]

            let columns =
              (names, columnStartAndEndPoints)
              ||> List.map2 (fun n p ->
                { Name = n
                  Start = p.Start
                  End = p.End })
              |> List.map Column
              |> Seq.ofList

            columns

          let sg = sg |> Seq.toList

          match sg with
          | [] -> created
          | currGroup :: remainingGroups ->
            let cols =
              createForSingleGroup baseElevation currGroup |> Seq.toList

            let updatedElevation =
              cols
              |> List.map (fun (Column c) -> c.Start.Z)
              |> List.max
              |> Elevation

            let created = cols @ created
            createAll updatedElevation remainingGroups created

        createAll baseElevation sg []

  /// Accepts an <see cref="BuildingBlueprint"> and applies a sequence of predicate functions
  /// to each <see cref="UnvalidatedStoreyGroup"> in the building definition. If validation fails,
  /// function returns sequence of results containing the validation errors.
  let validate
    (b: BuildingBlueprint)
    : Result<Building, seq<ModelValidationResult>> =

    /// Maps a given UnvalidatedBuildingDefinition to a validated model.
    let create (b: BuildingBlueprint) : Building =
      let elev = (b.BaseElevation * 1.0<mm>) |> Elevation
      let sg = StoreyGroup.createMany b.StoreyGroups
      let storeys = StoreyGroup.Storeys.create elev sg
      let slabs = StoreyGroup.Slabs.create elev sg
      let columns = StoreyGroup.Columns.create elev sg

      { Storeys = storeys |> List.rev // reversing storey order, as was upside-down.
        Slabs = slabs
        Columns = columns }

    // WARNING: add validation checks for slabs!
    let predicates =
      seq {
        applyCheckToAll StoreyGroup.rangesArePositive
        applyCheckToAll StoreyGroup.rangeEndGreaterThanStart
        applyCheckToAll StoreyGroup.typicalHeightIsPositive
        StoreyGroup.allAdjacentGroupsAreConsecutive
      }

    let result = b.StoreyGroups |> applyValidationChecks predicates

    match (allElementsAreValid result) with
    | true -> Ok(create b)
    | false -> Error result
