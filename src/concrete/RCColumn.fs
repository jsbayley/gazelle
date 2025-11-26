// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Concrete

//open System

//type RebarCageSpec =
//    | WallCageSpec of WallCage.Specification
//    | ColumnCageSpec of ColumnCage.Specification

//type RebarCage =
//    | WallCage of WallCage.Cage
//    | ColumnCage of ColumnCage.Cage

//type RCSection =
//    { Geometry: Solid3D<mm>
//      Concrete: Concrete
//      Cover: ConcreteCover
//      RebarCage: RebarCage }

//[<RequireQualifiedAccess>]
//module RCSection =

//    /// Returns a fully configured RC Section with concrete/steel properties and
//    /// reinforcement cage. Method leverages complex union types to switch between
//    /// different categories of section (e.g. column vs. wall) and to allow
//    /// for alternative cross-sectional geometries.
//    let create concrete steel sectionGeometry cover (cageSpecification: RebarCageSpec)  =
//        let cage =
//            match cageSpecification with
//            | WallCageSpec wallCageSpec ->
//                match sectionGeometry, cover with
//                | Cuboid (Centreline cl, Width w, Depth d),
//                  RectangularSectionCover (Cover top, Cover left, Cover right, Cover bottom, Cover ends) ->
//                        let horizBarSize = wallCageSpec.HorizontalBarSpec.BarSize.diameter
//                        let cageWidth = w - left - right - horizBarSize
//                        let cageDepth = d - top - bottom
//                        let cageLength = (AddUnitsToFloat cl.Length) - (2.0 * ends)
//                        RectangularCage (Width cageWidth, Depth cageDepth, Length cageLength)
//                        |> WallCage.create wallCageSpec steel |> WallCage
//                | Cylinder _, _ | _, CircularSectionCover _ ->
//                    invalidArg $"{nameof(sectionGeometry)}" "Walls do not support cylindrical geometries."

//            | ColumnCageSpec columnCageSpec ->
//                match sectionGeometry, cover with
//                | Cuboid (Centreline cl, Width w, Depth d),
//                  RectangularSectionCover (Cover top, Cover left, Cover right, Cover bottom, Cover ends) ->
//                        match columnCageSpec with
//                        | ColumnCage.RectangularCageSpec rectColCageSpec ->
//                            let linkSize = rectColCageSpec.Links.LinkSize.diameter
//                            let cageWidth = w - left - right - linkSize
//                            let cageDepth = d - top - bottom - linkSize
//                            let cageLength = (AddUnitsToFloat cl.Length) - (2.0 * ends)
//                            RectangularCage (Width cageWidth, Depth cageDepth, Length cageLength)
//                            |> ColumnCage.create columnCageSpec steel |> ColumnCage
//                | Cylinder _, _ | _, CircularSectionCover _ ->
//                    raise (NotImplementedException "Circular geometries not yet supported.")

//        { Geometry = sectionGeometry
//          Concrete = concrete
//          Cover = cover
//          RebarCage = cage }
