// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Structures

open System
open Gazelle.Units

/// Orthogonal geometric cross-sectional axes.
type Axis =
  | X
  | Y

/// Orthogonal axes about which rotation occurs.
type RotationalAxis =
  | XX
  | YY

// Basic geometric properties.
type Length<[<Measure>] 'T> = Length of float<'T>
type Width<[<Measure>] 'T> = Width of float<'T>
type Depth<[<Measure>] 'T> = Depth of float<'T>
type Diameter<[<Measure>] 'T> = Diameter of float<'T>
type Perimeter<[<Measure>] 'T> = Perimeter of float<'T>
type CrossSectionDiameter<[<Measure>] 'T> = CrossSectionDiameter of float<'T>
type CrossSectionalArea<[<Measure>] 'T> = CrossSectionalArea of float<'T^2>
type SurfaceArea<[<Measure>] 'T> = SurfaceArea of float<'T^2>
type Volume<[<Measure>] 'T> = Volume of float<'T^3>
type SecondMomentOfArea<[<Measure>] 'T> = SecondMomentOfArea of float<'T^4>

// Inner/outer specific dimensions.
type InnerWidth<[<Measure>] 'T> = InnerWidth of float<'T>
type OuterWidth<[<Measure>] 'T> = OuterWidth of float<'T>
type InnerDepth<[<Measure>] 'T> = InnerDepth of float<'T>
type OuterDepth<[<Measure>] 'T> = OuterDepth of float<'T>
type InnerDiameter<[<Measure>] 'T> = InnerDiameter of float<'T>
type OuterDiameter<[<Measure>] 'T> = OuterDiameter of float<'T>

/// Defines whether 3D shape end-faces are to be included in surface area calculations.
type EndFacesFlag =
  | IncludeEndFaces
  | ExcludeEndFaces

/// 2D coordinate set.
type Point2D<[<Measure>] 'T> = { X: float<'T>; Y: float<'T> }

/// 3D coordinate set.
type Point3D<[<Measure>] 'T> =
  { X: float<'T>
    Y: float<'T>
    Z: float<'T> }

/// Cuboidal geometry with centrepoints defined at either end.
type Cuboid<[<Measure>] 'T> =
  { StartPoint: Point3D<'T>
    EndPoint: Point3D<'T>
    Width: Width<'T>
    Depth: Depth<'T>
    Length: Length<'T> }

/// Cylindrical geometry with centrepoints defined at either end.
type Cylinder<[<Measure>] 'T> =
  { StartPoint: Point3D<'T>
    EndPoint: Point3D<'T>
    Diameter: Diameter<'T>
    Length: Length<'T> }

/// Hoop geometry moulded into a rectangular shape with a circular cross-section.
type RectangularHoop<[<Measure>] 'T> =
  { Centroid: Point3D<'T>
    Width: Width<'T>
    Depth: Depth<'T>
    LegDiameter: CrossSectionDiameter<'T>
    InnerWidth: InnerWidth<'T>
    InnerDepth: InnerDepth<'T>
    OuterWidth: OuterWidth<'T>
    OuterDepth: OuterDepth<'T> }

/// Hoop geometry moulded into a circular shape with a circular cross-section.
type CircularHoop<[<Measure>] 'T> =
  { Centroid: Point3D<'T>
    Diameter: Diameter<'T>
    LegDiameter: CrossSectionDiameter<'T>
    InnerDiameter: InnerDiameter<'T>
    OuterDiameter: OuterDiameter<'T> }

/// Unified type for all 3D solid geometries.
type Solid3D<[<Measure>] 'T> =
  | Cuboid of Cuboid<'T>
  | Cylinder of Cylinder<'T>

/// Unified type for all 3D hoop geometries.
type Hoop3D<[<Measure>] 'T> =
  | RectangularHoop of RectangularHoop<'T>
  | CircularHoop of CircularHoop<'T>

/// Unified type for all 3D geometries.
type Shape3D<[<Measure>] 'T> =
  | Solid3D of Solid3D<'T>
  | Hoop3D of Hoop3D<'T>


/// Collection of modules/functions to calculate geometric properties for various shapes.
[<RequireQualifiedAccess>]
module Geometry =

  /// Collection of utility functions to create various geometrical shapes.
  module Create =

    /// <summary>Creates a validated Cuboid instance.</summary>
    /// <exception cref="ArgumentOutOfRangeException"></exception>
    let cuboid (xyCentroid: Point2D<'T>) width depth length =
      match width, depth, length with
      | width, _, _ when width <= 0.0<_> ->
        invalidArg $"{nameof (width)}" "Width <= 0."
      | _, depth, _ when depth <= 0.0<_> ->
        invalidArg $"{nameof (depth)}" "Depth <= 0."
      | _, _, length when length <= 0.0<_> ->
        invalidArg $"{nameof (length)}" "Length <= 0."
      | _ ->
        { StartPoint =
            { X = xyCentroid.X
              Y = xyCentroid.Y
              Z = 0.0<_> }
          EndPoint =
            { X = xyCentroid.X
              Y = xyCentroid.Y
              Z = 0.0<_> }
          Width = Width width
          Depth = Depth depth
          Length = Length length }

    /// <summary>Creates a validated Cylinder instance.</summary>
    /// <exception cref="ArgumentOutOfRangeException"></exception>
    let cylinder
      (xyCentroid: Point2D<'T>)
      (diameter: float<'T>)
      (length: float<'T>)
      =
      match diameter, length with
      | diameter, _ when diameter <= 0.0<_> ->
        invalidArg $"{nameof (diameter)}" "Diameter <= 0."
      | _, length when length <= 0.0<_> ->
        invalidArg $"{nameof (length)}" "Length <= 0."
      | _ ->
        { StartPoint =
            { X = xyCentroid.X
              Y = xyCentroid.Y
              Z = 0.0<_> }
          EndPoint =
            { X = xyCentroid.X
              Y = xyCentroid.Y
              Z = 0.0<_> }
          Diameter = Diameter diameter
          Length = Length length }


    /// <summary>Creates a validated RectangularHoop instance.</summary>
    /// <exception cref="ArgumentOutOfRangeException"></exception>
    let rectangularHoop (centroid: Point3D<'T>) width depth legDiameter =
      match width, depth, legDiameter with
      | width, _, _ when width <= 0.0<_> ->
        invalidArg $"{nameof (width)}" "Width <= 0."
      | _, depth, _ when depth <= 0.0<_> ->
        invalidArg $"{nameof (depth)}" "Depth <= 0."
      | _, _, legDiameter when legDiameter <= 0.0<_> ->
        invalidArg $"{nameof (legDiameter)}" "Leg Diameter <= 0."
      | _ ->
        { Centroid = centroid
          Width = Width width
          Depth = Depth depth
          LegDiameter = CrossSectionDiameter legDiameter
          InnerWidth = InnerWidth(width - legDiameter)
          InnerDepth = InnerDepth(depth - legDiameter)
          OuterWidth = OuterWidth(width + legDiameter)
          OuterDepth = OuterDepth(depth + legDiameter) }

    /// <summary>Creates a validated CircularHoop instance.</summary>
    /// <exception cref="ArgumentOutOfRangeException"></exception>
    let circularHoop (centroid: Point3D<'T>) diameter legDiameter =
      match diameter, legDiameter with
      | diameter, _ when diameter <= 0.0<_> ->
        invalidArg $"{nameof (diameter)}" "Diameter <= 0."
      | _, legDiameter when legDiameter <= 0.0<_> ->
        invalidArg $"{nameof (legDiameter)}" "Leg Diameter <= 0."
      | _ ->
        { Centroid = centroid
          Diameter = Diameter diameter
          LegDiameter = CrossSectionDiameter legDiameter
          InnerDiameter = InnerDiameter(diameter - legDiameter)
          OuterDiameter = OuterDiameter(diameter + legDiameter) }


  /// Collection of utility functions to query geometric properties of shapes.
  [<RequireQualifiedAccess>]
  module Query =

    /// Unified type for all 3D solid geometries.
    [<RequireQualifiedAccess>]
    [<CompilationRepresentation(CompilationRepresentationFlags.ModuleSuffix)>]
    module Solid3D =

      /// Returns the start coordinate of a given 3D solid geometry.
      let getStartPoint (s: Solid3D<'T>) =
        match s with
        | Cuboid c -> c.StartPoint
        | Cylinder c -> c.StartPoint

      /// Returns the end coordinate of a given 3D solid geometry.
      let getEndPoint (s: Solid3D<'T>) =
        match s with
        | Cuboid c -> c.EndPoint
        | Cylinder c -> c.EndPoint

      /// Returns the length of a given 3D solid geometry.
      let getLength (s: Solid3D<'T>) =
        match s with
        | Cuboid c -> c.Length
        | Cylinder c -> c.Length

      /// Returns the cross-sectional perimeter of a given 3D solid geometry.
      let getPerimeter (s: Solid3D<'T>) =
        match s with
        | Cuboid { Width = Width w; Depth = Depth d } ->
          Perimeter((2.0 * w) + (2.0 * d))
        | Cylinder { Diameter = Diameter d } -> Perimeter(Math.PI * d)

      /// Returns the cross-sectional area of a given 3D solid geometry.
      let getCrossSectionalArea (s: Solid3D<'T>) =
        match s with
        | Cuboid { Width = Width w; Depth = Depth d } ->
          CrossSectionalArea(w * d)
        | Cylinder { Diameter = Diameter d } ->
          CrossSectionalArea(Math.PI * (Math.pow2 d) / 4.0)

      /// Returns the volume of a given 3D solid geometry.
      let getVolume (s: Solid3D<'T>) =
        let (CrossSectionalArea a) = getCrossSectionalArea s
        let (Length l) = getLength s

        match s with
        | _ -> Volume(a * l)

      /// Returns the surface area of a given 3D solid geometry. Optionally include the end faces.
      let getSurfaceArea (f: EndFacesFlag) (s: Solid3D<'T>) =
        let (Perimeter p) = getPerimeter s
        let (Length l) = getLength s
        let (CrossSectionalArea a) = getCrossSectionalArea s

        match s with
        | _ ->
          match f with
          | IncludeEndFaces -> SurfaceArea((2.0 * a) + (p * l))
          | ExcludeEndFaces -> SurfaceArea(p * l)

      /// Returns the second moment of area of a given 3D solid geometry about either the XX or YY axes.
      let getSecondMomentOfArea (axis: RotationalAxis) (s: Solid3D<'T>) =
        match s with
        | Cuboid { Width = Width w; Depth = Depth d } ->
          match axis with
          | XX -> SecondMomentOfArea(w * Math.pow3 d / 12.0)
          | YY -> SecondMomentOfArea(d * Math.pow3 w / 12.0)
        | Cylinder { Diameter = Diameter d } ->
          match axis with
          | _ -> SecondMomentOfArea(Math.PI * Math.pow4 d / 64.0)

    /// Unified type for all 3D hoop geometries.
    [<RequireQualifiedAccess>]
    [<CompilationRepresentation(CompilationRepresentationFlags.ModuleSuffix)>]
    module Hoop3D =

      /// Returns the centroid coordinates of a given hoop geometry.
      let getCentroid (h: Hoop3D<'T>) =
        match h with
        | RectangularHoop rh -> rh.Centroid
        | CircularHoop ch -> ch.Centroid

      /// Returns the hoop length.
      let getLength (h: Hoop3D<'T>) =
        match h with
        | RectangularHoop { Width = Width w; Depth = Depth d } ->
          Length((2.0 * w) + (2.0 * d))
        | CircularHoop { Diameter = Diameter d } -> Length(Math.PI * d)

      /// Returns the cross-sectional area of the hoop leg.
      let getCrossSectionalAreaOfLeg (h: Hoop3D<'T>) =
        match h with
        | RectangularHoop { LegDiameter = CrossSectionDiameter sd }
        | CircularHoop { LegDiameter = CrossSectionDiameter sd } ->
          CrossSectionalArea(Math.PI * Math.pow2 sd / 4.0)

      /// Returns the hoop volume.
      let getVolume (h: Hoop3D<'T>) =
        let (Length l) = getLength h
        let (CrossSectionalArea a) = getCrossSectionalAreaOfLeg h

        match h with
        | _ -> Volume(l * a)

    /// Unified type for all 3D geometries.
    [<RequireQualifiedAccess>]
    [<CompilationRepresentation(CompilationRepresentationFlags.ModuleSuffix)>]
    module Shape3D =

      /// Returns the 3D shape volume.
      let getVolume (shape: Shape3D<'T>) =
        match shape with
        | Solid3D s -> Solid3D.getVolume s
        | Hoop3D h -> Hoop3D.getVolume h

      /// Returns the 3D shape length.
      let getLength (shape: Shape3D<'T>) =
        match shape with
        | Solid3D s -> Solid3D.getLength s
        | Hoop3D h -> Hoop3D.getLength h

  /// Collection of convenient utility functions to unwrap single-case union types.
  [<RequireQualifiedAccess>]
  module Unwrap =

    let width (Width w) = w
    let depth (Depth d) = d
    let diameter (Diameter d) = d
    let crossSectionDiameter (CrossSectionDiameter sd) = sd
    let length (Length l) = l
    let perimeter (Perimeter p) = p
    let crossSectionalArea (CrossSectionalArea a) = a
    let volume (Volume v) = v
    let surfaceArea (SurfaceArea sa) = sa
    let secondMomentOfArea (SecondMomentOfArea sm) = sm
    let innerWidth (InnerWidth iw) = iw
    let innerDepth (InnerDepth id) = id
    let innerDiameter (InnerDiameter id) = id
    let outerWidth (OuterWidth ow) = ow
    let outerDepth (OuterDepth od) = od
    let outerDiameter (OuterDiameter od) = od
