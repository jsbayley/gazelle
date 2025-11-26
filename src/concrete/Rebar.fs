namespace Gazelle.Concrete

open Gazelle.Units
open FSharp.Data.UnitSystems.SI.UnitSymbols

type UKSteelGrade =
  | B400
  | B500
  | B600
  | C400
  | C500
  | C600

type SteelGrade = UKSteelGrade of UKSteelGrade

type UKBarSize =
  | H6
  | H8
  | H10
  | H12
  | H16
  | H20
  | H25
  | H32
  | H40
  | H50

type BarSize = UKBarSize of UKBarSize

type StraightBar =
  { SteelGrade: SteelGrade
    BarSize: BarSize
    Geometry: Cylinder<mm> }

type RectangularLink =
  { SteelGrade: SteelGrade
    BarSize: BarSize
    Geometry: RectangularHoop<mm> }

[<RequireQualifiedAccess>]
module Rebar =

  let getDiameter (b: BarSize) =
    match b with
    | UKBarSize size ->
      match size with
      | H6 -> 6.0<mm>
      | H8 -> 8.0<mm>
      | H10 -> 10.0<mm>
      | H12 -> 12.0<mm>
      | H16 -> 16.0<mm>
      | H20 -> 20.0<mm>
      | H25 -> 25.0<mm>
      | H32 -> 32.0<mm>
      | H40 -> 40.0<mm>
      | H50 -> 50.0<mm>

  [<RequireQualifiedAccess>]
  module Steel =

    let getYieldStrength (s: SteelGrade) =
      match s with
      | UKSteelGrade grade ->
        match grade with
        | B400 -> 400.0<N / mm^2>
        | B500 -> 500.0<N / mm^2>
        | B600 -> 600.0<N / mm^2>
        | C400 -> 400.0<N / mm^2>
        | C500 -> 500.0<N / mm^2>
        | C600 -> 600.0<N / mm^2>

    let getDensity (s: SteelGrade) =
      match s with
      | UKSteelGrade _ -> Density 7850.0<kg / m^3>

    let getElasticModulus (s: SteelGrade) =
      match s with
      | UKSteelGrade _ -> 200_000.0<N / mm^2>

  [<RequireQualifiedAccess>]
  module StraightBar =

    let tryCreate
      (s: SteelGrade)
      (b: BarSize)
      (centroid: Point2D<mm>)
      (l: Length<mm>)
      : StraightBar =
      let (Length length) = l
      let diameter = getDiameter b

      { SteelGrade = s
        BarSize = b
        Geometry = Geometry.Create.cylinder centroid diameter length }

  [<RequireQualifiedAccess>]
  module RectangularLink =

    let tryCreate
      (s: SteelGrade)
      (b: BarSize)
      (centroid: Point3D<mm>)
      (w: Width<mm>)
      (d: Depth<mm>)
      =
      let (Width width) = w
      let (Depth depth) = d
      let legDiameter = getDiameter b

      { SteelGrade = s
        BarSize = b
        Geometry =
          Geometry.Create.rectangularHoop centroid width depth legDiameter }
