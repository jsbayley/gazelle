// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Geometry

/// <summary>
/// Orthogonal geometric axis.
/// </summary>
type Axis =
  | X
  | Y

/// <summary>
/// Orthogonal axis about which rotation occurs.
/// </summary>
type RotationalAxis =
  | XX
  | YY

/// <summary>
/// Length dimension annotated with unit-of-measure.
/// </summary>
type Length<[<Measure>] 'TUnit> = Length of float<'TUnit>

/// <summary>
/// Width dimension annotated with unit-of-measure.
/// </summary>
type Width<[<Measure>] 'TUnit> = Width of float<'TUnit>

/// <summary>
/// Depth dimension annotated with unit-of-measure.
/// </summary>
type Depth<[<Measure>] 'TUnit> = Depth of float<'TUnit>

/// <summary>
/// Diameter dimension annotated with unit-of-measure.
/// </summary>
type Diameter<[<Measure>] 'TUnit> = Diameter of float<'TUnit>

/// <summary>
/// Perimeter dimension annotated with unit-of-measure.
/// </summary>
type Perimeter<[<Measure>] 'TUnit> = Perimeter of float<'TUnit>

/// <summary>
/// Area dimension annotated with unit-of-measure.
/// Union type represents possible area semantics.
/// </summary>
type Area<[<Measure>] 'TUnit> =
  | CrossSectionalArea of float<'TUnit^2>
  | SurfaceArea of float<'TUnit^2>

/// <summary>
/// Volume dimension annotated with unit-of-measure.
/// </summary>
type Volume<[<Measure>] 'TUnit> = Volume of float<'TUnit^3>

/// <summary>
/// Second moment of area dimension annotated with unit-of-measure.
/// </summary>
type SecondMomentOfArea<[<Measure>] 'TUnit> =
  | SecondMomentOfArea of float<'TUnit^4>
