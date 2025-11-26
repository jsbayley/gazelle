// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Structures

//open System

///// Centre-to-centre spacing between adjacent bars/links.
//type BarSpacing = BarSpacing of float<mm>

///// Space between bars, measured between the outermost bar dimensions.
//type ClearSpace = ClearSpace of float<mm>

///// Total number of reinforcing bars.
//type NumberOfBars = NumberOfBars of int<bars>

//// Lightweight records specifying rebar configuration.
//type NumberOfBarsAndBarSize = { NumberOfBars: NumberOfBars; BarSize: BarSize }
//type LinkSpacingAndLinkSize = { LinkSpacing: BarSpacing; LinkSize: BarSize }

//module Detailing =

//    /// Returns no. of intermediate bars along a given length at the prescribed spacing.
//    /// This does NOT consider bars at the start or end (i.e. at x = 0 and x = L).
//    /// For a 1000mm length at 250mm spacing, the equations returns 3 intermediate bars.
//    let getNumberOfIntermediateBars (BarSpacing sp) (Length l) =
//        int (Math.Ceiling (l / sp)) - 1
//        |> AddUnitsToInteger32
//        |> NumberOfBars

//    /// Calculates bar spacing for intermediate bars along a given length.
//    /// This does NOT consider bars at the start or end (i.e. at x = 0 and x = L).
//    /// For a 1000mm length with 3 inner bars and 2 end bars (5 bars in total),
//    /// the equation ignores the end bars and returns a spacing of 250mm.
//    let getIntermediateBarSpacing (NumberOfBars n) (Length l) = BarSpacing (l / (float n + 1.0))

//    /// Returns the minimum permissible clear spacing between adjacent bars.
//    let getMinimumClearBarSpacing (barSize: BarSize) = ClearSpace barSize.diameter

//    /// Calculates clear spacing between two straight bars by comparing the pythagorean
//    /// distance between their respective cross-section centroids in 3D space.
//    let clearSpaceBetweenBars (bar1: StraightBar) (bar2: StraightBar) =
//        let p1 = bar1.Geometry.StartPoint
//        let p2 = bar2.Geometry.StartPoint
//        let halfΦ1 = (bar1.BarSize.diameter / 2.0) |> UnitAnnotation.stripUnitsFromFloat
//        let halfΦ2 = (bar2.BarSize.diameter / 2.0) |> UnitAnnotation.stripUnitsFromFloat
//        (distanceBetweenPoints p1 p2) - (halfΦ1 + halfΦ2)
//        |> AddUnitsToFloat

//    // Union type helper methods.
//    let internal unwrapBarSpacing (BarSpacing sp) = sp
//    let internal unwrapClearSpace (ClearSpace sp) = sp
