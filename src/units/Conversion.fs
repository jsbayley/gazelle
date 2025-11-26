// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Units

open FSharp.Data.UnitSystems.SI.UnitSymbols

/// <summary>
/// Functions to convert unit-of-measure type annotations.
/// </summary>
[<RequireQualifiedAccess>]
module Convert =

  /// <summary>
  /// Converts millimetres to metres.
  /// </summary>
  /// <param name="n">Value in millimetres.</param>
  /// <returns>Value in metres.</returns>
  let millimetresToMetres (n: float<mm>) : float<m> = n / 1000.0<mm / m>

  /// <summary>
  /// Converts metres to millimetres.
  /// </summary>
  /// <param name="n">Value in metres.</param>
  /// <returns>Value in millimetres.</returns>
  let metresToMillimetres (n: float<m>) : float<mm> = n * 1000.0<mm / m>

  /// <summary>
  /// Converts Newtons to Kilonewtons.
  /// </summary>
  /// <param name="n">Value in Newtons.</param>
  /// <returns>Value in Kilonewtons.</returns>
  let newtonsToKilonewtons (x: float<N>) : float<kN> = x / 1000.0<N / kN>

  /// <summary>
  /// Converts Kilonewtons to Newtons.
  /// </summary>
  /// <param name="n">Value in Kilonewtons.</param>
  /// <returns>Value in Newtons.</returns>
  let kilonewtonsToNewtons (x: float<kN>) : float<N> = x * 1000.0<N / kN>
