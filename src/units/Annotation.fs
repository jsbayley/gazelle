// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Units

/// <summary>
/// Remove unit-of-measure type annotations from various types.
/// </summary>
[<RequireQualifiedAccess>]
module RemoveUnits =

  /// <summary>
  /// Remove unit-of-measure type annotation from 32-bit integer.
  /// </summary>
  /// <param name="value">Annotated 32-bit integer.</param>
  /// <returns>Value without unit-of-measure.</returns>
  let fromInt32 (value: int<'T>) : int = int value

  /// <summary>
  /// Remove unit-of-measure type annotation from float.
  /// </summary>
  /// <param name="value">Annotated float.</param>
  /// <returns>Value without unit-of-measure.</returns>
  let fromFloat (value: float<'T>) : float = float value

/// <summary>
/// Add unit-of-measure type annotations to various types.
/// </summary>
[<RequireQualifiedAccess>]
module AddUnits =

  /// <summary>
  /// Add unit-of-measure type annotation to 32-bit integer.
  /// </summary>
  /// <param name="value">Primitive 32-bit integer.</param>
  /// <returns>Value annotated with unit-of-measure.</returns>
  let toInt32 (value: int) : int<'T> =
    LanguagePrimitives.Int32WithMeasure value

  /// <summary>
  /// Add unit-of-measure type annotation to float.
  /// </summary>
  /// <param name="value">Primitive float.</param>
  /// <returns>Value annotated with unit-of-measure.</returns>
  let toFloat (value: float) : float<'T> =
    LanguagePrimitives.FloatWithMeasure value
