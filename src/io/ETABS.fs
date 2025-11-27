namespace Gazelle.IO.ETABS

open System
open Gazelle.IO

// ========================================
// CROSS-PLATFORM TYPES
// ========================================

type StoreyRenderProps =
  { Name: string
    Height: float
    IsMaster: bool
    MasterRefName: string
    IsSplicedAbove: bool
    SpliceHeight: float
    Color: int }

type Version =
  | V17
  | V19

type SaveOption =
  | SaveFile
  | DoNotSaveFile

#if WINDOWS
// ========================================
// WINDOWS-ONLY ETABS INTEROP
// ========================================

open System.Runtime.InteropServices

type Helper =
  | V17 of ETABSv17.cHelper
  | V19 of ETABSv1.cHelper

type InstanceType =
  | NewAppInstance
  | ExistingLiveInstance

type ETABSObject =
  | V17 of ETABSv17.cOAPI
  | V19 of ETABSv1.cOAPI

type SAPModel =
  | V17 of ETABSv17.cSapModel
  | V19 of ETABSv1.cSapModel

[<RequireQualifiedAccess>]
module ETABS =

  /// Refreshes ETABS window, allowing any model changes to be displayed.
  let refreshView (s: SAPModel) : unit =
    match s with
    | V17 s -> s.View.RefreshView(0, false)
    | V19 s -> s.View.RefreshView(0, false)
    |> ignore

  /// Creates fresh environment within ETABS.
  let initialise (s: SAPModel) : unit =

    /// Creates clean ETABS model environment and sets global unit system.
    let initialiseNewModel (s: SAPModel) : unit =
      match s with
      | V17 s -> s.InitializeNewModel(ETABSv17.eUnits.kN_mm_C)
      | V19 s -> s.InitializeNewModel(ETABSv1.eUnits.kN_mm_C)
      |> ignore

    /// Sets blank environment canvas for ETABS model.
    let createBlankModel (s: SAPModel) : unit =
      match s with
      | V17 s -> s.File.NewBlank()
      | V19 s -> s.File.NewBlank()
      |> ignore

    initialiseNewModel s
    createBlankModel s

  let start () : Result<(ETABSObject * SAPModel), IOError> =

    // Prompts user for desired ETABS version and validates input.
    let askUserToSelectVersion () : Result<Version, IOError> =

      /// Checks that the specified ETABS version is supported.
      let validateVersion (v: string) : Result<Version, IOError> =
        match v.ToUpper().Trim() with
        | "V17"
        | "VERSION17"
        | "17" -> Ok Version.V17
        | "V19"
        | "VERSION19"
        | "19" -> Ok Version.V19
        | _ -> Error(UnsupportedVersion "Unsupported ETABS Version.")

      IO.askForUserInput "Select ETABS Version [V17/V19]..." validateVersion

    /// Establishes the ETABS version and either launches a fresh instance or attaches to an existing process.
    let using (i: InstanceType) (v: Version) : (ETABSObject * SAPModel) =

      // Returns path to ETABS installation on local machine.
      let getInstallationPath (v: Version) : FilePath =
        match v with
        | Version.V17 ->
          FilePath
            "C:\\Program Files\\Computers and Structures\\ETABS 17\\ETABS.exe"
        | Version.V19 ->
          FilePath
            "C:\\Program Files\\Computers and Structures\\ETABS 19\\ETABS.exe"

      // Returns cHelper interface object used to instantiate ETABS objects.
      let createHelper (v: Version) : Helper =
        match v with
        | Version.V17 -> Helper.V17(ETABSv17.Helper() :> ETABSv17.cHelper)
        | Version.V19 -> Helper.V19(ETABSv1.Helper() :> ETABSv1.cHelper)

      // Provides access to various ETABS API methods via the SAPModel interface.
      let getSAPModel (e: ETABSObject) : SAPModel =
        match e with
        | ETABSObject.V17 e -> e.SapModel |> V17
        | ETABSObject.V19 e -> e.SapModel |> V19

      let (FilePath p) = getInstallationPath v
      let h = createHelper v

      let etabs =
        match h, i with
        | Helper.V17 h, NewAppInstance -> ETABSObject.V17(h.CreateObject(p))
        | Helper.V19 h, NewAppInstance -> ETABSObject.V19(h.CreateObject(p))
        | Helper.V17 _, ExistingLiveInstance ->
          failwith "ExistingLiveInstance not implemented yet"
        | Helper.V19 _, ExistingLiveInstance ->
          failwith "ExistingLiveInstance not implemented yet"

      let sapModel = getSAPModel etabs
      etabs, sapModel

    /// Launches fresh connection between API and ETABS process.
    let launchETABS (e: ETABSObject) =
      match e with
      | ETABSObject.V17 e -> e.ApplicationStart()
      | ETABSObject.V19 e -> e.ApplicationStart()
      |> ignore

    let version = askUserToSelectVersion ()

    let result =
      match version with
      | Ok v ->
        Messages.info "Launching ETABS Interactive..."
        Ok(using NewAppInstance v)
      | Error e -> Error e

    match result with
    | Ok(e, s) ->
      launchETABS e
      initialise s
      Ok(e, s)
    | Error e -> Error e

  let close
    (app: Result<ETABSObject * SAPModel, IOError>)
    (saveOption: SaveOption)
    : unit =

    /// Optionally saves and then safely closes ETABS.
    let closeETABS (e: ETABSObject) (s: SaveOption) =
      match e, s with
      | ETABSObject.V17 e, DoNotSaveFile -> e.ApplicationExit(false)
      | ETABSObject.V19 e, DoNotSaveFile -> e.ApplicationExit(false)
      | ETABSObject.V17 e, SaveFile -> e.ApplicationExit(true)
      | ETABSObject.V19 e, SaveFile -> e.ApplicationExit(true)
      |> ignore

    match app with
    | Ok(e, _) ->
      Messages.abort "Closing ETABS..."

      try
        closeETABS e saveOption
      with ex ->
        printfn $"Exception safely caught during shutdown: {ex.Message}"
    | Error e -> printfn "%A" e

#else
// ========================================
// NON-WINDOWS STUB IMPLEMENTATIONS
// ========================================

[<RequireQualifiedAccess>]
module ETABS =

  let start () : Result<unit, IOError> =
    Error(
      UnsupportedVersion
        "ETABS integration is only available on Windows platforms."
    )

  let close (app: Result<unit, IOError>) (saveOption: SaveOption) : unit =
    printfn "ETABS integration not available on this platform."

#endif

// ========================================
// CROSS-PLATFORM MODEL VALIDATION
// ========================================

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
  module StoreyGroup =

    /// Verifies that both the Start and End properties of a Storey Group Range are positive.
    let rangesArePositive (s: UnvalidatedStoreyGroup) : ModelValidationResult =
      IntegerRange.startAndEndArePositive s.StoreyRange

    /// Verifies that, for a given range, the End value is greater than the Start value.
    let rangeEndGreaterThanStart
      (s: UnvalidatedStoreyGroup)
      : ModelValidationResult =
      IntegerRange.endIsGreaterThanStart s.StoreyRange

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

  /// Accepts a BuildingBlueprint and applies validation functions to each UnvalidatedStoreyGroup.
  /// If validation fails, returns sequence of validation errors.
  let validate
    (b: BuildingBlueprint)
    : Result<Building, seq<ModelValidationResult>> =

    /// Maps an UnvalidatedBuildingDefinition to a validated model.
    let create (b: BuildingBlueprint) : Building =
      // Simplified model creation - expand as needed
      { Storeys = []
        Slabs = []
        Columns = [] }

    let predicates =
      seq {
        applyCheckToAll StoreyGroup.rangesArePositive
        applyCheckToAll StoreyGroup.rangeEndGreaterThanStart
        applyCheckToAll StoreyGroup.typicalHeightIsPositive
      }

    let result = b.StoreyGroups |> applyValidationChecks predicates

    match (allElementsAreValid result) with
    | true -> Ok(create b)
    | false -> Error result
