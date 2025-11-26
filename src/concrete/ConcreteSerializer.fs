// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Concrete

// module ConcreteSerializer =

//     module CylinderStrength =

//         let deserialize (region: string) (fck: string) =
//             match region.ToUpperInvariant() with
//             | "UK" ->
//                 match fck.ToUpperInvariant() with
//                 | "C12/15" -> UKConcrete Fck12
//                 | "C16/20" -> UKConcrete Fck16
//                 | "C20/25" -> UKConcrete Fck20
//                 | "C25/30" -> UKConcrete Fck25
//                 | "C30/37" -> UKConcrete Fck30
//                 | "C35/45" -> UKConcrete Fck35
//                 | "C40/50" -> UKConcrete Fck40
//                 | "C45/55" -> UKConcrete Fck45
//                 | "C50/60" -> UKConcrete Fck50
//                 | "C55/67" -> UKConcrete Fck55
//                 | "C60/75" -> UKConcrete Fck60
//                 | "C70/85" -> UKConcrete Fck70
//                 | "C80/95" -> UKConcrete Fck80
//                 | "C90/105" -> UKConcrete Fck90
//                 | _ -> invalidArg $"{nameof(fck)}" "No matching cylinder strength."
//             | _ -> invalidArg $"{nameof(region)}" "No matching Eurocode region."

//         let serialize (fck: CylinderStrength) =
//             match fck with
//             | UKConcrete Fck12 -> "C12/15"
//             | UKConcrete Fck16 -> "C16/20"
//             | UKConcrete Fck20 -> "C20/25"
//             | UKConcrete Fck25 -> "C25/30"
//             | UKConcrete Fck30 -> "C30/37"
//             | UKConcrete Fck35 -> "C35/45"
//             | UKConcrete Fck40 -> "C40/50"
//             | UKConcrete Fck45 -> "C45/55"
//             | UKConcrete Fck50 -> "C50/60"
//             | UKConcrete Fck55 -> "C55/67"
//             | UKConcrete Fck60 -> "C60/75"
//             | UKConcrete Fck70 -> "C70/85"
//             | UKConcrete Fck80 -> "C80/95"
//             | UKConcrete Fck90 -> "C90/105"

//     module Aggregate =

//         let deserialize (agg: string) =
//             match agg.ToLowerInvariant() with
//             | "basalt" -> Basalt
//             | "limestone" -> Limestone
//             | "quartzite" -> Quartzite
//             | "sandstone" -> Sandstone
//             | _ -> invalidArg $"{nameof(agg)}" "No matching aggregate type."

//         let serialize (aggregate: Aggregate) =
//             match aggregate with
//             | Basalt -> "Basalt"
//             | Limestone -> "Limestone"
//             | Quartzite -> "Quartzite"
//             | Sandstone -> "Sandstone"

//     module Cement =

//         let deserialize (cem: string) =
//             match cem.ToLowerInvariant() with
//             | "classn" -> ClassN
//             | "classs" -> ClassS
//             | "classr" -> ClassR
//             | _ -> invalidArg $"{nameof(cem)}" "No matching cement type."

//         let serialize (cem: Cement) =
//             match cem with
//             | ClassN -> "ClassN"
//             | ClassR -> "ClassR"
//             | ClassS -> "ClassS"

//     module MixProperties =

//         type MixPropertiesDTO =
//             { Aggregate: string
//               Cement: string
//               Grade: string
//               Region: string }

//         let deserialize (concreteMix: MixPropertiesDTO) : MixProperties =
//             match concreteMix with
//             | { Aggregate = agg
//                 Cement = cem
//                 Grade = fck
//                 Region = region } ->
//                     { Aggregate = Aggregate.deserialize agg
//                       Cement = Cement.deserialize cem
//                       Grade = CylinderStrength.deserialize region fck }

//         let serialize (mix: MixProperties) : MixPropertiesDTO =
//             let region =
//                 match mix.Grade with
//                 | UKConcrete _ -> "UK"
//             { Aggregate = Aggregate.serialize mix.Aggregate
//               Cement = Cement.serialize mix.Cement
//               Grade = CylinderStrength.serialize mix.Grade
//               Region = region }

//     module Strengths =

//         type StrengthsDTO =
//             { fck: float
//               fcm: float
//               fctm: float
//               fcm_t: float
//               fctm_t: float }

//         let deserialize (strengths: StrengthsDTO) : Strengths =
//             match strengths with
//             | { fck = _fck
//                 fcm = _fcm
//                 fctm = _fctm
//                 fcm_t = _fcm_t
//                 fctm_t = _fctm_t } ->
//                     { fck = AddUnitsToFloat _fck
//                       fcm = AddUnitsToFloat _fcm
//                       fctm = AddUnitsToFloat _fctm
//                       fcm_t = AddUnitsToFloat _fcm_t
//                       fctm_t = AddUnitsToFloat _fctm_t }

//         let serialize (strengths: Strengths) : StrengthsDTO =
//             { fck = stripUnitsFromFloat strengths.fck
//               fcm = stripUnitsFromFloat strengths.fcm
//               fctm = stripUnitsFromFloat strengths.fctm
//               fcm_t = stripUnitsFromFloat strengths.fcm_t
//               fctm_t = stripUnitsFromFloat strengths.fctm_t }

//     module ElasticModuli =

//         type ElasticModuliDTO =
//             { Ecm: float
//               Ecm_t: float }

//         let deserialize (moduli: ElasticModuliDTO) : ElasticModuli =
//             { Ecm = AddUnitsToFloat moduli.Ecm
//               Ecm_t = AddUnitsToFloat moduli.Ecm_t }

//         let serialize (moduli: ElasticModuli) : ElasticModuliDTO =
//             { Ecm = stripUnitsFromFloat moduli.Ecm
//               Ecm_t = stripUnitsFromFloat moduli.Ecm_t }

//     module MechanicalProperties =

//         open Strengths
//         open ElasticModuli

//         type MechanicalPropertiesDTO =
//             { Strengths: StrengthsDTO
//               Strains: Strains
//               ElasticModuli: ElasticModuliDTO }

//         let deserialize (props: MechanicalPropertiesDTO) : MechanicalProperties =
//             match props with
//             | { Strengths = strengths
//                 Strains = strains
//                 ElasticModuli = moduli } ->
//                     { Strengths = Strengths.deserialize strengths
//                       Strains = strains
//                       ElasticModuli = ElasticModuli.deserialize moduli }

//         let serialize (props: MechanicalProperties) : MechanicalPropertiesDTO =
//             { Strengths = Strengths.serialize props.Strengths
//               Strains = props.Strains
//               ElasticModuli = ElasticModuli.serialize props.ElasticModuli }

//     module Concrete =

//         open MixProperties
//         open MechanicalProperties

//         type ConcreteDTO =
//             { Age: int
//               Density: float
//               MixProperties: MixPropertiesDTO
//               MechanicalProperties: MechanicalPropertiesDTO }

//         let deserialize (concrete: ConcreteDTO) : Concrete =
//             match concrete with
//             | { Age = age
//                 Density = density
//                 MixProperties = mix
//                 MechanicalProperties = props } ->
//                     { Age = CurrentAge (AddUnitsToInteger32 age)
//                       Density = AddUnitsToFloat density
//                       MixProperties = MixProperties.deserialize mix
//                       MechanicalProperties = MechanicalProperties.deserialize props }

//         let serialize (concrete: Concrete) : ConcreteDTO =
//             let unwrapAge (CurrentAge age) = age
//             { Age = stripUnitsFromInteger (unwrapAge concrete.Age)
//               Density = stripUnitsFromFloat concrete.Density
//               MixProperties = MixProperties.serialize concrete.MixProperties
//               MechanicalProperties = MechanicalProperties.serialize concrete.MechanicalProperties }
