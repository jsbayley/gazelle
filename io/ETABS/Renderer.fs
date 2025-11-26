namespace Gazelle.IO.ETABS

open Gazelle.IO

module Renderer =

  let private renderStoreys (sap: SAPModel) (storeys: Storey list) : unit =

    let getMasterStoreyRenderProps (ms: MasterStorey) =
      { Name = Unwrap.name ms.Name
        Height = float (Unwrap.heightAndPositiveFloat ms.Height)
        IsMaster = true
        MasterRefName = "None"
        IsSplicedAbove = false
        SpliceHeight = 0.0
        Color = 0 }

    let getSimilarStoreyRenderProps (ss: SimilarStorey) =
      { Name = Unwrap.name ss.Name
        Height = float (Unwrap.heightAndPositiveFloat ss.Height)
        IsMaster = false
        MasterRefName = $"{Unwrap.name ss.SimilarTo.Name}"
        IsSplicedAbove = false
        SpliceHeight = 0.0
        Color = 0 }

    let getStoreyProps (s: Storey list) : StoreyRenderProps[] =
      s
      |> List.map (fun s ->
        match s with
        | MasterStorey ms -> getMasterStoreyRenderProps ms
        | SimilarStorey ss -> getSimilarStoreyRenderProps ss)
      |> Array.ofList

    let getElevation (s: Storey) : float =
      match s with
      | MasterStorey ms -> float (Unwrap.elevation ms.Elevation)
      | SimilarStorey ss -> float (Unwrap.elevation ss.Elevation)

    let storeyProps = storeys |> getStoreyProps
    let names = ref (storeyProps |> Array.map (fun s -> s.Name))
    let heights = ref (storeyProps |> Array.map (fun s -> s.Height))
    let isMaster = ref (storeyProps |> Array.map (fun s -> s.IsMaster))

    let masterRefName =
      ref (storeyProps |> Array.map (fun s -> s.MasterRefName))

    let isSplicedAbove =
      ref (storeyProps |> Array.map (fun s -> s.IsSplicedAbove))

    let spliceHeights = ref (storeyProps |> Array.map (fun s -> s.SpliceHeight))
    let colors = ref (storeyProps |> Array.map (fun s -> s.Color))
    let baseElevation = (storeys |> List.map getElevation |> List.min)
    let numberOfStoreys = storeys.Length

    (match sap with
     | V17 s ->
       s.Story.SetStories_2(
         baseElevation,
         numberOfStoreys,
         names,
         heights,
         isMaster,
         masterRefName,
         isSplicedAbove,
         spliceHeights,
         colors
       )
     | V19 s ->
       s.Story.SetStories_2(
         baseElevation,
         numberOfStoreys,
         names,
         heights,
         isMaster,
         masterRefName,
         isSplicedAbove,
         spliceHeights,
         colors
       ))
    |> ignore

  // WARNING: Code to be refactored.
  let private renderSlabs (sap: SAPModel) (slabs: Slab list) : unit =

    let slabItems =
      slabs
      |> List.map (fun slab ->
        {| NumberOfPoints = slab.Vertices.Length
           X =
            ref (
              slab.Vertices |> List.map (fun s -> float s.X) |> Array.ofList
            )
           Y =
            ref (
              slab.Vertices |> List.map (fun s -> float s.Y) |> Array.ofList
            )
           Z =
            ref (
              slab.Vertices |> List.map (fun s -> float s.Z) |> Array.ofList
            )
           Name = ref (slab.Name |> Unwrap.name) |})

    match sap with
    | V17 sap ->
      slabItems
      |> List.iter (fun s ->
        sap.AreaObj.AddByCoord(s.NumberOfPoints, s.X, s.Y, s.Z, s.Name)
        |> ignore)
    | V19 sap ->
      slabItems
      |> List.iter (fun s ->
        sap.AreaObj.AddByCoord(s.NumberOfPoints, s.X, s.Y, s.Z, s.Name)
        |> ignore)

  let private renderColumns (sap: SAPModel) (columns: Column list) =

    let columnItems =
      columns
      |> List.map (fun (Column c) ->
        {| Name = ref (Unwrap.name c.Name)
           Xi = float c.Start.X
           Yi = float c.Start.Y
           Zi = float c.Start.Z
           Xj = float c.End.X
           Yj = float c.End.Y
           Zj = float c.End.Z |})

    match sap with
    | V17 sap ->
      columnItems
      |> List.iter (fun c ->
        sap.FrameObj.AddByCoord(c.Xi, c.Yi, c.Zi, c.Xj, c.Yj, c.Zj, c.Name)
        |> ignore)
    | V19 sap ->
      columnItems
      |> List.iter (fun c ->
        sap.FrameObj.AddByCoord(c.Xi, c.Yi, c.Zi, c.Xj, c.Yj, c.Zj, c.Name)
        |> ignore)


  // WARNING: Consider refactoring method.
  let render
    (app: Result<ETABSObject * SAPModel, IOError>)
    (b: Building)
    : unit =
    match app with
    | Ok(_, sap) ->
      Messages.success "Rendering ETABS model..."
      ETABS.initialise sap
      renderStoreys sap b.Storeys
      renderSlabs sap b.Slabs
      renderColumns sap b.Columns
      ETABS.refreshView sap
    | Error e -> IOError.print e
