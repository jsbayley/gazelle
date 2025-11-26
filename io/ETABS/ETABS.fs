namespace Gazelle.IO.ETABS

open System
open System.Runtime.InteropServices

open Gazelle.IO
open Gazelle.IO.IO

type Version =
  | V17
  | V19

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

type SaveOption =
  | SaveFile
  | DoNotSaveFile

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

      askForUserInput "Select ETABS Version [V17/V19]..." validateVersion

    /// Establishes the ETABS version and either launches a fresh instance or attaches to an existing process.
    let using (i: InstanceType) (v: Version) : (ETABSObject * SAPModel) =

      // Returns path to ETABS installation on local machine.
      let getInstallationPath (v: Version) : FilePath =
        match v with
        | Version.V17 ->
          FilePath
            "C:\Program Files\Computers and Structures\ETABS 17\ETABS.exe"
        | Version.V19 ->
          FilePath
            "C:\Program Files\Computers and Structures\ETABS 19\ETABS.exe"

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
          // TODO: Fix Marshal.GetActiveObject - requires COM registration
          failwith "ExistingLiveInstance not implemented yet"
        | Helper.V19 _, ExistingLiveInstance ->
          // TODO: Fix Marshal.GetActiveObject - requires COM registration  
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
