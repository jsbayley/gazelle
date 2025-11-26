// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Structures

open Gazelle.Units
open FSharp.Data.UnitSystems.SI.UnitSymbols

type UKConcreteGrade =
  | Fck12
  | Fck16
  | Fck20
  | Fck25
  | Fck30
  | Fck35
  | Fck40
  | Fck45
  | Fck50
  | Fck55
  | Fck60
  | Fck70
  | Fck80
  | Fck90

type CylinderStrength = UK of UKConcreteGrade

type Aggregate =
  | Basalt
  | Limestone
  | Sandstone
  | Quartzite

type Cement =
  | ClassR
  | ClassN
  | ClassS

type WeightClass = NormalWeight
type Age = Age of int<days>

type Concrete =
  { Age: Age
    Aggregate: Aggregate
    Cement: Cement
    Grade: CylinderStrength
    WeightClass: WeightClass }

[<RequireQualifiedAccess>]
module Concrete =

  let private validateAge (age: int<days>) =
    match age with
    | age when age <= 0<days> ->
      invalidArg $"{nameof (age)}" "Concrete age <= 0 days."
    | age when age <= 3<days> ->
      invalidArg
        $"{nameof (age)}"
        "In-situ strength tests required for age <= 3 days."
    | _ -> Age age

  let tryCreate
    (fck: CylinderStrength)
    (agg: Aggregate)
    (cem: Cement)
    (w: WeightClass)
    (age: int<days>)
    =
    let validAge = validateAge age

    { Age = validAge
      Aggregate = agg
      Cement = cem
      Grade = fck
      WeightClass = w }

  [<RequireQualifiedAccess>]
  module BasicProperties =

    [<RequireQualifiedAccess>]
    module Density =

      let unreinforced (w: WeightClass) =
        match w with
        | NormalWeight -> Density 2400.0<kg / m^3>

      let reinforced (w: WeightClass) =
        match w with
        | NormalWeight -> Density 2500.0<kg / m^3>

    [<RequireQualifiedAccess>]
    module Strength =

      let fck (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 12.0<N / mm^2>
        | UK Fck16 -> 16.0<N / mm^2>
        | UK Fck20 -> 20.0<N / mm^2>
        | UK Fck25 -> 25.0<N / mm^2>
        | UK Fck30 -> 30.0<N / mm^2>
        | UK Fck35 -> 35.0<N / mm^2>
        | UK Fck40 -> 40.0<N / mm^2>
        | UK Fck45 -> 45.0<N / mm^2>
        | UK Fck50 -> 50.0<N / mm^2>
        | UK Fck55 -> 55.0<N / mm^2>
        | UK Fck60 -> 60.0<N / mm^2>
        | UK Fck70 -> 70.0<N / mm^2>
        | UK Fck80 -> 80.0<N / mm^2>
        | UK Fck90 -> 90.0<N / mm^2>

      let fcm (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 20.0<N / mm^2>
        | UK Fck16 -> 24.0<N / mm^2>
        | UK Fck20 -> 28.0<N / mm^2>
        | UK Fck25 -> 33.0<N / mm^2>
        | UK Fck30 -> 38.0<N / mm^2>
        | UK Fck35 -> 43.0<N / mm^2>
        | UK Fck40 -> 48.0<N / mm^2>
        | UK Fck45 -> 53.0<N / mm^2>
        | UK Fck50 -> 58.0<N / mm^2>
        | UK Fck55 -> 63.0<N / mm^2>
        | UK Fck60 -> 68.0<N / mm^2>
        | UK Fck70 -> 78.0<N / mm^2>
        | UK Fck80 -> 88.0<N / mm^2>
        | UK Fck90 -> 98.0<N / mm^2>

      let fctm (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 1.6<N / mm^2>
        | UK Fck16 -> 1.9<N / mm^2>
        | UK Fck20 -> 2.2<N / mm^2>
        | UK Fck25 -> 2.6<N / mm^2>
        | UK Fck30 -> 2.9<N / mm^2>
        | UK Fck35 -> 3.2<N / mm^2>
        | UK Fck40 -> 3.5<N / mm^2>
        | UK Fck45 -> 3.8<N / mm^2>
        | UK Fck50 -> 4.1<N / mm^2>
        | UK Fck55 -> 4.2<N / mm^2>
        | UK Fck60 -> 4.4<N / mm^2>
        | UK Fck70 -> 4.6<N / mm^2>
        | UK Fck80 -> 4.8<N / mm^2>
        | UK Fck90 -> 5.0<N / mm^2>

    [<RequireQualifiedAccess>]
    module ElasticModulus =

      let Ecm (fck: CylinderStrength) (agg: Aggregate) =
        let Ecm =
          match fck with
          | UK Fck12 -> 27_000.0<N / mm^2>
          | UK Fck16 -> 29_000.0<N / mm^2>
          | UK Fck20 -> 30_000.0<N / mm^2>
          | UK Fck25 -> 31_000.0<N / mm^2>
          | UK Fck30 -> 33_000.0<N / mm^2>
          | UK Fck35 -> 34_000.0<N / mm^2>
          | UK Fck40 -> 35_000.0<N / mm^2>
          | UK Fck45 -> 36_000.0<N / mm^2>
          | UK Fck50 -> 37_000.0<N / mm^2>
          | UK Fck55 -> 38_000.0<N / mm^2>
          | UK Fck60 -> 39_000.0<N / mm^2>
          | UK Fck70 -> 41_000.0<N / mm^2>
          | UK Fck80 -> 42_000.0<N / mm^2>
          | UK Fck90 -> 44_000.0<N / mm^2>

        match agg with
        | Quartzite -> Ecm * 1.0
        | Limestone -> Ecm * 0.9
        | Sandstone -> Ecm * 0.7
        | Basalt -> Ecm * 1.2

    [<RequireQualifiedAccess>]
    module Strain =

      let c1 (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 0.0018
        | UK Fck16 -> 0.0019
        | UK Fck20 -> 0.0020
        | UK Fck25 -> 0.0021
        | UK Fck30 -> 0.0022
        | UK Fck35 -> 0.00225
        | UK Fck40 -> 0.0023
        | UK Fck45 -> 0.0024
        | UK Fck50 -> 0.00245
        | UK Fck55 -> 0.0025
        | UK Fck60 -> 0.0026
        | UK Fck70 -> 0.0027
        | UK Fck80 -> 0.0028
        | UK Fck90 -> 0.0028

      let cu1 (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 0.0035
        | UK Fck16 -> 0.0035
        | UK Fck20 -> 0.0035
        | UK Fck25 -> 0.0035
        | UK Fck30 -> 0.0035
        | UK Fck35 -> 0.0035
        | UK Fck40 -> 0.0035
        | UK Fck45 -> 0.0035
        | UK Fck50 -> 0.0035
        | UK Fck55 -> 0.0032
        | UK Fck60 -> 0.0030
        | UK Fck70 -> 0.0028
        | UK Fck80 -> 0.0028
        | UK Fck90 -> 0.0028

      let c2 (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 0.0020
        | UK Fck16 -> 0.0020
        | UK Fck20 -> 0.0020
        | UK Fck25 -> 0.0020
        | UK Fck30 -> 0.0020
        | UK Fck35 -> 0.0020
        | UK Fck40 -> 0.0020
        | UK Fck45 -> 0.0020
        | UK Fck50 -> 0.0020
        | UK Fck55 -> 0.0022
        | UK Fck60 -> 0.0023
        | UK Fck70 -> 0.0024
        | UK Fck80 -> 0.0025
        | UK Fck90 -> 0.0026

      let cu2 (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 0.0035
        | UK Fck16 -> 0.0035
        | UK Fck20 -> 0.0035
        | UK Fck25 -> 0.0035
        | UK Fck30 -> 0.0035
        | UK Fck35 -> 0.0035
        | UK Fck40 -> 0.0035
        | UK Fck45 -> 0.0035
        | UK Fck50 -> 0.0035
        | UK Fck55 -> 0.0031
        | UK Fck60 -> 0.0029
        | UK Fck70 -> 0.0027
        | UK Fck80 -> 0.0026
        | UK Fck90 -> 0.0026

      let c3 (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 0.00175
        | UK Fck16 -> 0.00175
        | UK Fck20 -> 0.00175
        | UK Fck25 -> 0.00175
        | UK Fck30 -> 0.00175
        | UK Fck35 -> 0.00175
        | UK Fck40 -> 0.00175
        | UK Fck45 -> 0.00175
        | UK Fck50 -> 0.00175
        | UK Fck55 -> 0.0018
        | UK Fck60 -> 0.0019
        | UK Fck70 -> 0.0020
        | UK Fck80 -> 0.0022
        | UK Fck90 -> 0.0023

      let cu3 (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 0.0035
        | UK Fck16 -> 0.0035
        | UK Fck20 -> 0.0035
        | UK Fck25 -> 0.0035
        | UK Fck30 -> 0.0035
        | UK Fck35 -> 0.0035
        | UK Fck40 -> 0.0035
        | UK Fck45 -> 0.0035
        | UK Fck50 -> 0.0035
        | UK Fck55 -> 0.0031
        | UK Fck60 -> 0.0029
        | UK Fck70 -> 0.0027
        | UK Fck80 -> 0.0026
        | UK Fck90 -> 0.0026

      let n (fck: CylinderStrength) =
        match fck with
        | UK Fck12 -> 2.0
        | UK Fck16 -> 2.0
        | UK Fck20 -> 2.0
        | UK Fck25 -> 2.0
        | UK Fck30 -> 2.0
        | UK Fck35 -> 2.0
        | UK Fck40 -> 2.0
        | UK Fck45 -> 2.0
        | UK Fck50 -> 2.0
        | UK Fck55 -> 1.75
        | UK Fck60 -> 1.6
        | UK Fck70 -> 1.45
        | UK Fck80 -> 1.4
        | UK Fck90 -> 1.4

  [<RequireQualifiedAccess>]
  module TimeDependentProperties =

    let private s cement =
      match cement with
      | ClassN -> 0.25
      | ClassR -> 0.20
      | ClassS -> 0.38

    let private α age =
      match age with
      | (Age age) when age < 28<days> -> 1.0
      | _ -> (2.0 / 3.0)

    let private βcc_t cement age =
      let (Age age) = age
      s cement |> exp |> (*) (1.0 - sqrt (28.0 / float age))

    [<RequireQualifiedAccess>]
    module Strength =

      let fcm_t (fck: CylinderStrength) (cem: Cement) (age: Age) =
        let βcc_t = βcc_t cem age
        let fcm_basic = BasicProperties.Strength.fcm fck
        βcc_t * fcm_basic

      let fctm_t (fck: CylinderStrength) (cem: Cement) (age: Age) =
        let βcc_t = βcc_t cem age
        let fctm_basic = BasicProperties.Strength.fctm fck
        (βcc_t ** (α age)) * fctm_basic

    [<RequireQualifiedAccess>]
    module ElasticModulus =

      let Ecm_t
        (fck: CylinderStrength)
        (cem: Cement)
        (agg: Aggregate)
        (age: Age)
        =
        let fcm = BasicProperties.Strength.fcm fck
        let fcm_timeDep = Strength.fcm_t fck cem age
        let Ecm = BasicProperties.ElasticModulus.Ecm fck agg
        ((fcm_timeDep / fcm) ** 0.3) * Ecm
