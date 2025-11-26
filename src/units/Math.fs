// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Units

/// <summary>
/// Collection of mathematical modules that operate on
/// values with unit-of-measure type annotations.
/// </summary>
[<RequireQualifiedAccess>]
module Math =

  /// <summary>
  /// Unit safe exponentiation function raised to the power of 2.
  /// </summary>
  /// <param name="n">Original value.</param>
  /// <returns>Value raised to the power of 2.</returns>
  let pow2 (n: float<'T>) : float<'T^2> = n * n

  /// <summary>
  /// Unit safe exponentiation function raised to the power of 3.
  /// </summary>
  /// <param name="n">Original value.</param>
  /// <returns>Value raised to the power of 3.</returns>
  let pow3 (n: float<'T>) : float<'T^3> = n * n * n

  /// <summary>
  /// Unit safe exponentiation function raised to the power of 4.
  /// </summary>
  /// <param name="n">Original value.</param>
  /// <returns>Value raised to the power of 4.</returns>
  let pow4 (n: float<'T>) : float<'T^4> = n * n * n * n
