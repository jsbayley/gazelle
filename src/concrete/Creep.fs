// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Structures

// open type System.Math
// open FSharp.Data.UnitSystems.SI.UnitSymbols

// module Creep =

//type RelativeHumidity = RelativeHumidity of float
//type LoadingAge = LoadingAge of int<days>
//type NotionalSize = NotionalSize of h0:float<mm>

/// Convenient collection of Creep and Shrinkage related parameters.
//type CreepShrinkage =
//    { RelativeHumidity: RelativeHumidity
//      LoadingAge: LoadingAge
//      NotionalSize: NotionalSize
//      CreepCoefficient: float
//      ShrinkageStrain: float }

//     // Partially applied functions to calculate Creep α factors.
//     let private α (power: float) (fcm: float<N/mm^2>) = (35.0<N/mm^2> / fcm) ** power

//     let α1 = α 0.7
//     let α2 = α 0.2
//     let α3 = α 0.5

//     /// β_fcm is a factor to allow for the effect of concrete strength on the
//     /// notional creep coefficient.
//     let βfcm (fcm: float<N/mm^2>) =
//         let fcm = float fcm
//         16.8 / sqrt fcm

//     /// β_t0 is a factor to allow for the effect of concrete age at loading
//     /// on the notional creep coefficient.
//     let βt0 (LoadingAge loadingAge) =
//         1.0 / (0.1 + (float loadingAge ** 0.2))

//     /// βH is a coefficient depending on the relative humidity (RH in %).
//     let βH (RelativeHumidity rh) (NotionalSize h0) (fcm: float<N/mm^2>) =
//         let h0 = float h0
//         if fcm <= 35.0<N/mm^2> then
//             let βh =
//                 0.012 * (rh ** 18.0)
//                 |> (+) 1.0
//                 |> (*) 1.5
//                 |> (*) h0
//                 |> (+) 250.0
//             if βh <= 1500.0 then βh else 1500.0
//         else
//             let βh =
//                 0.012 * (rh ** 18.0)
//                 |> (+) 1.0
//                 |> (*) 1.5
//                 |> (*) h0
//                 |> (+) (250.0 * α3 fcm)
//             if βh <= (1500.0 * α3 fcm) then βh else (1500.0 * α3 fcm)

//     /// β_ctt0 is a coefficient to describe the development of creep with time after loading.
//     let βctt0 (LoadingAge loadingAge) (CurrentAge currentAge) (βH: float) =
//         let t = (float)currentAge
//         let t0 = (float)loadingAge
//         let numerator = t - t0
//         let denominator = βH + t - t0
//         let quotient = numerator / denominator
//         quotient ** 0.3

//     /// Returns the Notional Creep Coefficient (RH) to EC2.
//     let ψRH (RelativeHumidity rh) (NotionalSize h0) (fcm: float<N/mm^2>) =
//         let h0 = (float)h0
//         if fcm <= 35.0<N/mm^2> then
//             let numerator = 1.0 - rh
//             let denominator = 0.1 * Cbrt(h0)
//             let quotient = numerator / denominator
//             1.0 + quotient
//         else
//             let numerator = 1.0 - rh
//             let denominator = 0.1 * Cbrt(h0) * (α1 fcm)
//             let quotient = numerator / denominator
//             (1.0 + quotient) * (α2 fcm)

//     /// Returns the Creep Coefficient to EC2.
//     let ψtt0 (βctt0: float) (βfcm: float) (βt0: float) (ψRH: float) =
//         ψRH * βfcm * βt0 * βctt0

//     /// Calculates notional geometric size of member.
//     let private getNotionalSize (section: Shape3D<mm, m>) =
//         let area = getSectionalArea section
//         let perimeter = getPerimeter section
//         2.0 * area / perimeter |> NotionalSize

//     let private validateLoadingAge (loadingAge: int<days>) (CurrentAge concreteAge) =
//         if loadingAge <= 0<days> || loadingAge > concreteAge then
//             invalidArg $"nameof{loadingAge}" "Illegal concrete loading age."
//         else LoadingAge loadingAge

//     let private validateRelativeHumidity (rh: float) =
//         if (rh <= 0.0 || rh >= 1.0) then
//             invalidArg $"{nameof(rh)}" "Relative Humidity outside allowable range."
//         else RelativeHumidity rh

//     /// Calculates the Creep Coefficient, ψtt0, to EC2.
//     let getCreepCoefficient (section: Shape3D<mm, m>) (loadingAge: int<days>) (relativeHumidity: float) concrete  =
//         let h0 = getNotionalSize section
//         let rh = validateRelativeHumidity relativeHumidity
//         let loadingAge = validateLoadingAge loadingAge concrete.Age
//         let fcm = concrete.MechanicalProperties.Strengths.fcm
//         let currentAge = concrete.Age
//         let βfcm = βfcm fcm
//         let βt0 = βt0 loadingAge
//         let βH = βH rh h0 fcm
//         let βctt0 = βctt0 loadingAge currentAge βH
//         let ψRH = ψRH rh h0 fcm
//         ψtt0 βctt0 βfcm βt0 ψRH
