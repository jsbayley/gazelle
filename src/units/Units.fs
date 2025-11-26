// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Units

open FSharp.Data.UnitSystems.SI.UnitSymbols

/// <summary>
/// Time in days.
/// </summary>
[<Measure>]
type days

/// <summary>
/// Length in millimetres.
/// </summary>
[<Measure>]
type mm

/// <summary>
/// Force in Kilonewtons.
/// </summary>
[<Measure>]
type kN

/// <summary>
/// Moment or Torque in Kilonewton-Metres.
/// </summary>
[<Measure>]
type kNm = kN * m

/// <summary>
/// Number of layers.
/// </summary>
[<Measure>]
type layers

/// <summary>
/// Number of bars.
/// </summary>
[<Measure>]
type bars

/// <summary>
/// Unit of density.
/// </summary>
type Density<[<Measure>] 'TMass, [<Measure>] 'TLength> =
  | Density of float<'TMass / 'TLength^3>

/// <summary>
/// Unit of mechanical stress.
/// </summary>
type Stress<[<Measure>] 'TForce, [<Measure>] 'TLength> =
  | Stress of float<'TForce / 'TLength^2>

/// <summary>
/// Unit of mechanical pressure.
/// </summary>
type Pressure<[<Measure>] 'TForce, [<Measure>] 'TLength> =
  Stress<'TForce, 'TLength>
