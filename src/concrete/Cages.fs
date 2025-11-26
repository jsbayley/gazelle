// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Concrete

//open System

///// 3D rebar cage dimensions measured between centrelines of outermost reinforcement.
//type CageDimensions<[<Measure>] 'T> =
//    | RectangularCage of Width<'T> * Depth<'T> * Length<'T>

//    member this.Centreline =
//        match this with
//        | RectangularCage (_, _, Length l) ->
//            (Point3D (0.0, 0.0, 0.0),
//                Point3D (0.0, 0.0, stripUnitsFromFloat l))
//            ||> createCentreline

//    member this.Length =
//        match this with
//        | RectangularCage (_, _, l) -> l


//[<RequireQualifiedAccess>]
//module ColumnCage =

//    /// Rectangular column cage face.
//    type Face = Top | Left | Right | Bottom

//    /// Rectangular column cage corners.
//    type Corner = TopRight | TopLeft | BottomRight | BottomLeft

//    /// Straight reinforcing bars positioned at cage corners.
//    type CornerBars =
//        { TopRight: StraightBar
//          TopLeft: StraightBar
//          BottomRight: StraightBar
//          BottomLeft: StraightBar }

//    /// Collection of straight reinforcing bars for respective cage faces.
//    type SideBars =
//        { TopFace: StraightBar list
//          LeftFace: StraightBar list
//          RightFace: StraightBar list
//          BottomFace: StraightBar list }

//    /// Defines rectangular column cage rebar configuration.
//    type RectangularCageSpecification =
//        { CornerBarSize: BarSize
//          TopFaceBars: NumberOfBarsAndBarSize
//          BottomFaceBars: NumberOfBarsAndBarSize
//          LeftFaceBars: NumberOfBarsAndBarSize
//          RightFaceBars: NumberOfBarsAndBarSize
//          Links: LinkSpacingAndLinkSize }

//    /// Rebar specifications for alternative column cage geometries.
//    type Specification =
//        | RectangularCageSpec of RectangularCageSpecification

//    /// Fully configured column cage.
//    type Cage =
//        { Dimensions: CageDimensions<mm>
//          CornerBars: CornerBars
//          SideBars: SideBars
//          Links: Link list }


//    [<RequireQualifiedAccess>]
//    module Links =

//        /// Calculates no. of links within given length at the prescribed spacing.
//        let getNumberOfLinks (BarSpacing sp) (Length l) =
//            Math.Ceiling (l / sp)
//            |> int
//            |> (+) 1
//            |> AddUnitsToInteger32
//            |> NumberOfBars

//        /// Calculates link spacing for a discrete no. of links over a given length.
//        let getLinkSpacing (NumberOfBars n) (Length l) =
//            l / (float n - 1.0)
//            |> BarSpacing

//        /// For a given spacing, returns all link centroids along a centreline.
//        let getLinkCentroidsAlongCentreline (Centreline l) (BarSpacing spacing) =
//            let spacing = stripUnitsFromFloat spacing
//            let p1, p2 = l.StartPoint, l.EndPoint
//            [ for z in p1.Z .. spacing .. p2.Z ->
//                Point3D (p1.X, p1.Y, z)
//                |> Centroid ]

//        /// Returns shear links for full column cage length.
//        let create steel linkSize spacing (cageDims: CageDimensions<mm>) =
//            let cl = cageDims.Centreline
//            let length = cageDims.Length
//            let quantity = getNumberOfLinks spacing length
//            let updatedSpacing = getLinkSpacing quantity length
//            let centroids = getLinkCentroidsAlongCentreline cl updatedSpacing
//            [for ct in centroids ->
//                match cageDims with
//                | RectangularCage (Width w, Depth d, _) ->
//                    RectangularLink.create
//                        ct
//                        (Width w)
//                        (Depth d)
//                        linkSize
//                        steel]


//    [<RequireQualifiedAccess>]
//    module CornerBars =

//        /// Locates the geometric bar centroid, for a selected corner and given bar size.
//        let findCorner link (barSize: BarSize) corner =
//            let b = link.Geometry.InnerWidth / 2.0
//            let d = link.Geometry.InnerDepth / 2.0
//            let ctX, ctY = link.Geometry.CentroidCoords
//            let Φ = barSize.diameter / 2.0

//            match corner with
//            | TopRight ->
//                let x = ctX + b - Φ
//                let y = ctY + d - Φ
//                x, y
//            | TopLeft ->
//                let x = ctX - b + Φ
//                let y = ctY + d - Φ
//                x, y
//            | BottomRight ->
//                let x = ctX + b - Φ
//                let y = ctY - d + Φ
//                x, y
//            | BottomLeft ->
//                let x = ctX - b + Φ
//                let y = ctY - d + Φ
//                x, y

//        /// Returns collection of straight steel bars for rectangular cage corners.
//        let create barSize steel link (Length len) =
//            let createAt corner =
//                corner
//                |> findCorner link barSize
//                |> createSymmetricEndPoints len
//                ||> StraightBar.create barSize steel

//            { TopRight = createAt TopRight
//              TopLeft = createAt TopLeft
//              BottomRight = createAt BottomRight
//              BottomLeft = createAt BottomLeft }


//    [<RequireQualifiedAccess>]
//    module SideBars =

//        /// Returns the spacing interval for intermediate side bars.
//        let getSideBarSpacing cornerBars side numberOfBars =
//            match side with
//            | Top | Bottom ->
//                distanceBetweenPoints
//                    cornerBars.TopRight.Geometry.StartPoint
//                    cornerBars.TopLeft.Geometry.StartPoint
//            | Left | Right ->
//                distanceBetweenPoints
//                    cornerBars.TopLeft.Geometry.StartPoint
//                    cornerBars.BottomLeft.Geometry.StartPoint
//            |> AddUnitsToFloat
//            |> Length
//            |> getIntermediateBarSpacing numberOfBars

//        /// Returns the 'constant' x or y coordinate for a given side.
//        let getSideBarConstantCoord (barSize: BarSize) side (link: Link) =
//            let Φ = barSize.diameter |> stripUnitsFromFloat
//            let ctX, ctY, w, d, sd =
//                match link.Geometry with
//                | RectangularHoop (Centroid ct, Width w, Depth d, SectionDiameter sd) ->
//                    ct.X, ct.Y, w, d, sd
//                | CircularHoop (Centroid ct, Diameter d, SectionDiameter sd) ->
//                    ct.X, ct.Y, d, d, sd
//            match side with
//            | Top ->    ctY + (d / 2.0<_>) - (sd / 2.0<_>) - (Φ / 2.0)
//            | Bottom -> ctY - (d / 2.0<_>) + (sd / 2.0<_>) + (Φ / 2.0)
//            | Left ->   ctX - (w / 2.0<_>) + (sd / 2.0<_>) + (Φ / 2.0)
//            | Right ->  ctX + (w / 2.0<_>) - (sd / 2.0<_>) - (Φ / 2.0)

//        /// Locates the side bar centroids.
//        let getSideBar2DCoords side numberOfBars cornerBars link barSize =
//            let spacing =
//                getSideBarSpacing cornerBars side numberOfBars
//                |> unwrapBarSpacing
//                |> stripUnitsFromFloat
//            let constantCoord = getSideBarConstantCoord barSize side link
//            match side with
//            | Top | Bottom ->
//                let xStart = cornerBars.TopLeft.Geometry.StartPoint.X + spacing
//                let xEnd = cornerBars.TopRight.Geometry.StartPoint.X - spacing
//                [for x in xStart .. spacing .. xEnd -> (x, constantCoord)]
//            | Left | Right ->
//                let yStart = cornerBars.BottomLeft.Geometry.StartPoint.Y + spacing
//                let yEnd = cornerBars.TopLeft.Geometry.StartPoint.Y - spacing
//                [for y in yStart .. spacing .. yEnd -> (constantCoord, y)]

//        /// Returns list of straight steel bars for a single cage side.
//        let create side cornerBars link barSize numberOfBars steel =
//            let xyCoords = getSideBar2DCoords side numberOfBars cornerBars link barSize
//            let zStart = cornerBars.TopRight.Geometry.StartPoint.Z
//            let zEnd = cornerBars.TopRight.Geometry.EndPoint.Z
//            [ for x, y in xyCoords ->
//                StraightBar.create
//                    barSize
//                    steel
//                    (Point3D (x, y, zStart))
//                    (Point3D (x, y, zEnd)) ]


//    /// Returns a fully configured column rebar cage.
//    let create (rebarSpec: Specification) steel (dimensions: CageDimensions<mm>) =
//        let length = match dimensions with RectangularCage (_, _, l) -> l

//        match rebarSpec with
//        | RectangularCageSpec spec ->

//            let links =
//                let linkSize = spec.Links.LinkSize
//                let linkSpacing = spec.Links.LinkSpacing
//                Links.create steel linkSize linkSpacing dimensions

//            let cornerBars =
//                let barSize = spec.CornerBarSize
//                CornerBars.create barSize steel links.[0] length

//            let sideBars =

//                let topFaceBars =
//                    let barSize = spec.TopFaceBars.BarSize
//                    let numOfBars = spec.TopFaceBars.NumberOfBars
//                    SideBars.create Top cornerBars links.[0] barSize numOfBars steel

//                let leftFaceBars =
//                    let barSize = spec.LeftFaceBars.BarSize
//                    let numOfBars = spec.LeftFaceBars.NumberOfBars
//                    SideBars.create Left cornerBars links.[0] barSize numOfBars steel

//                let rightFaceBars =
//                    let barSize = spec.RightFaceBars.BarSize
//                    let numOfBars = spec.RightFaceBars.NumberOfBars
//                    SideBars.create Right cornerBars links.[0] barSize numOfBars steel

//                let bottomFaceBars =
//                    let barSize = spec.BottomFaceBars.BarSize
//                    let numOfBars = spec.BottomFaceBars.NumberOfBars
//                    SideBars.create Bottom cornerBars links.[0] barSize numOfBars steel

//                { TopFace = topFaceBars
//                  LeftFace = leftFaceBars
//                  RightFace = rightFaceBars
//                  BottomFace = bottomFaceBars }

//            { Dimensions = dimensions
//              CornerBars = cornerBars
//              SideBars = sideBars
//              Links = links }


//[<RequireQualifiedAccess>]
//module WallCage =

//    /// Respective wall face.
//    type Face = LeftFace | RightFace

//    /// Respective wall end zone.
//    type EndZone = Top | Bottom

//    /// Number of pairwise bar layers in each end zone.
//    type NumberOfBarLayersInEndZone = NumberOfBarLayersInEndZone of int<layers>

//    /// Defines bar configuration for wall end zones.
//    type EndZoneBarSpecification =
//        { NumberOfLayers: NumberOfBarLayersInEndZone
//          BarSize: BarSize }

//    /// Defines bar configuration for wall central zone.
//    type CentralZoneBarSpecification = { BarSize: BarSize; Spacing: BarSpacing }

//    /// Defines bar configuration for horizontal wall bars.
//    type HorizontalBarSpecification = { BarSize: BarSize; Spacing: BarSpacing }

//    /// Defines wall cage rebar configurations for all zones.
//    type Specification =
//        { HorizontalBarSpec: HorizontalBarSpecification
//          EndZoneBarSpec: EndZoneBarSpecification
//          CentralZoneBarSpec: CentralZoneBarSpecification }

//    /// Fully configured wall cage.
//    type Cage =
//        { Dimensions: CageDimensions<mm>
//          HorizontalBars: StraightBar list
//          EndZoneBars: StraightBar list
//          CentralZoneBars: StraightBar list }


//    [<RequireQualifiedAccess>]
//    module HorizontalBars =

//        /// Returns x- and y-coordinates for horizontal wall bars.
//        let findHorizontalBarXYCoords cageDims =
//            let (Width w), (Depth d), _ = cageDims
//            let xLeft = -w / 2.0
//            let xRight = w / 2.0
//            let yTop = d / 2.0
//            let yBottom = -d / 2.0
//            xLeft, xRight, yTop, yBottom

//        /// Returns z-coordinates for horizontal wall bars.
//        let findHorizontalBarZCoords cageDims horizBarSpacing =
//            let _, _, l = cageDims
//            let numOfBars = getNumberOfIntermediateBars horizBarSpacing l
//            let updatedSpacing =
//                getIntermediateBarSpacing numOfBars l
//                |> unwrapBarSpacing
//                |> stripUnitsFromFloat
//            let length =
//                let unwrapLength (Length l) = l
//                l |> unwrapLength |> stripUnitsFromFloat
//            let zStart, zEnd = 0.0, length
//            [for z in zStart .. updatedSpacing .. zEnd -> z]

//        /// Groups horizontal wall bar x-, y- and z-coordinates for a given face.
//        let combineHorizontalXYZBarCoords xyCoords zCoords face =
//            let xLeft, xRight, yTop, yBottom = xyCoords
//            zCoords
//            |> match face with
//                | LeftFace ->
//                    List.map (fun z ->
//                        Point3D (stripUnitsFromFloat xLeft, stripUnitsFromFloat yBottom, z),
//                        Point3D (stripUnitsFromFloat xLeft, stripUnitsFromFloat yTop, z))
//                | RightFace ->
//                    List.map (fun z ->
//                        Point3D (stripUnitsFromFloat xRight, stripUnitsFromFloat yBottom, z),
//                        Point3D (stripUnitsFromFloat xRight, stripUnitsFromFloat yTop, z))

//        /// Returns all steel horizontal wall bars.
//        let create (cageDims: CageDimensions<mm>) horizBarSpacing barSize steel =
//            let cageDims =
//                match cageDims with
//                | RectangularCage (w, d, l) -> (w, d, l)

//            let xyCoords = findHorizontalBarXYCoords cageDims
//            let zCoords = findHorizontalBarZCoords cageDims horizBarSpacing
//            let barCoordsLeft = combineHorizontalXYZBarCoords xyCoords zCoords LeftFace
//            let barCoordsRight = combineHorizontalXYZBarCoords xyCoords zCoords RightFace
//            barCoordsLeft @ barCoordsRight
//            |> List.map (fun (p1, p2) ->
//                StraightBar.create barSize steel p1 p2)

//        /// Returns clear space between horizontal wall bars.
//        let getClearSpaceBetweenHorizontalBars cageDims (horizontalBar: BarSize) =
//            let (Width cageWidth), _, _ = cageDims
//            cageWidth - horizontalBar.diameter
//            |> ClearSpace


//    [<RequireQualifiedAccess>]
//    module VerticalBars =

//        /// Returns x-coordinate of vertical wall bar, measured from the centreline of the cage.
//        let getVerticalBarXCoordinate face (verticalBarSize: BarSize) spaceBetweenHorizontalBars =
//            let halfWidth = unwrapClearSpace spaceBetweenHorizontalBars / 2.0
//            match face with
//            | LeftFace -> (- halfWidth) + (verticalBarSize.diameter / 2.0)
//            | RightFace -> halfWidth - (verticalBarSize.diameter / 2.0)


//        [<RequireQualifiedAccess>]
//        module EndZone =

//            /// Returns x- and y-coordinates for end zone bars, for a single end zone and wall face.
//            let findEndZoneBarCoords zone face barSize (OuterDepth endZoneDepth) cageDims horizBarSize =
//                let _, (Depth cageDepth), _ = cageDims
//                let ezDepth = endZoneDepth
//                let ezWidth = HorizontalBars.getClearSpaceBetweenHorizontalBars cageDims horizBarSize
//                let xCoord = getVerticalBarXCoordinate face barSize ezWidth
//                let Φ = barSize.diameter
//                let spacingBetweenBarCentres =
//                    2.0 * unwrapClearSpace (getMinimumClearBarSpacing barSize)
//                match zone with
//                | Top ->
//                    let yStart = (cageDepth - Φ) / 2.0
//                    let yEnd = yStart - ezDepth + (Φ / 2.0)
//                    [for y in yStart .. -spacingBetweenBarCentres .. yEnd -> (xCoord, y)]
//                | Bottom ->
//                    let yStart = (Φ - cageDepth) / 2.0
//                    let yEnd = yStart + ezDepth - (Φ / 2.0)
//                    [for y in yStart .. spacingBetweenBarCentres .. yEnd -> (xCoord, y)]

//            /// Returns depth of single end zone, including clear space between bars.
//            /// Measured to the outermost bar dimensions.
//            let getEndZoneDepth (NumberOfBarLayersInEndZone layers) endZoneBarSize =
//                let numOfLayers = layers |> float |> AddUnitsToFloat
//                let clearBarSpace =
//                    getMinimumClearBarSpacing endZoneBarSize
//                    |> unwrapClearSpace
//                    |> (*) (numOfLayers - 1.0)
//                endZoneBarSize.diameter
//                |> (*) numOfLayers
//                |> (+) clearBarSpace
//                |> OuterDepth

//            /// Returns list of straight steel bars for both end zones and wall faces.
//            let create barSize endZoneDepth cageDims horizBarSize steel =
//                let _, _, (Length cageLength) = cageDims
//                [ Top, LeftFace
//                  Top, RightFace
//                  Bottom, LeftFace
//                  Bottom, RightFace ]
//                |> List.collect (fun (zone, face) ->
//                    findEndZoneBarCoords zone face barSize endZoneDepth cageDims horizBarSize)
//                |> List.map (fun (x, y) -> stripUnitsFromFloat x, stripUnitsFromFloat y)
//                |> List.map (fun (x, y) ->
//                    StraightBar.create
//                        barSize
//                        steel
//                        (Point3D (x, y, 0.0))
//                        (Point3D (x, y, stripUnitsFromFloat cageLength)))


//        [<RequireQualifiedAccess>]
//        module CentralZone =

//            /// Returns x- and y-coordinates for central zone bars, for a single end zone and wall face.
//            let findCentralZoneBarCoords face barSize spacing (OuterDepth centralZoneDepth) cageDims horizBarSize =
//                let czWidth = HorizontalBars.getClearSpaceBetweenHorizontalBars cageDims horizBarSize
//                let czDepth = centralZoneDepth
//                let xCoord = getVerticalBarXCoordinate face barSize czWidth
//                let Φ = barSize.diameter
//                let yStart = (-czDepth / 2.0) + (Φ / 2.0)
//                let yEnd = (czDepth / 2.0) - (Φ / 2.0)
//                let length = (yEnd - yStart) |> Length
//                let numberOfBars = getNumberOfIntermediateBars spacing length
//                let updatedSpacing = getIntermediateBarSpacing numberOfBars length |> unwrapBarSpacing
//                [ for y in yStart .. updatedSpacing .. yEnd -> xCoord, y ]

//            /// Returns depth of central wall zone. Calculated using the outermost depth
//            /// of both end zones; adds additional clear spacing between the end zone edges
//            /// and the outermost bars in the central zone.
//            let getCentralZoneDepth cageDims endZoneOuterDepth endZoneClearBarSpacing =
//                let _, (Depth cageDepth), _ = cageDims
//                let depthOfBothEndZones =
//                    match endZoneOuterDepth with OuterDepth d -> d
//                    |> (*) 2.0
//                let clearSpaceForBothEndZones =
//                    endZoneClearBarSpacing
//                    |> unwrapClearSpace
//                    |> (*) 2.0
//                (cageDepth - depthOfBothEndZones - clearSpaceForBothEndZones)
//                |> OuterDepth

//            /// Returns list of straight steel bars for both faces of central zone.
//            let create barSize spacing centralZoneDepth horizBarSize cageDims steel =
//                let _, _, (Length cageLength) = cageDims
//                [ LeftFace; RightFace ]
//                |> List.collect (fun face ->
//                    findCentralZoneBarCoords face barSize spacing centralZoneDepth cageDims horizBarSize)
//                |> List.map (fun (x, y) -> stripUnitsFromFloat x, stripUnitsFromFloat y)
//                |> List.map (fun (x, y) ->
//                    StraightBar.create
//                        barSize
//                        steel
//                        (Point3D (x, y, 0.0))
//                        (Point3D (x, y, stripUnitsFromFloat cageLength)))


//    /// Returns fully configured wall cage.
//    let create (spec: Specification) steel (dimensions: CageDimensions<mm>) =
//        let dimsAsTuple =
//            match dimensions with
//            | RectangularCage (w, d, l) -> (w, d, l)

//        let horizBarSize = spec.HorizontalBarSpec.BarSize
//        let ezLayers = spec.EndZoneBarSpec.NumberOfLayers
//        let ezBarSize = spec.EndZoneBarSpec.BarSize
//        let ezClearBarSpacing = getMinimumClearBarSpacing ezBarSize
//        let ezDepth = VerticalBars.EndZone.getEndZoneDepth ezLayers ezBarSize
//        let czBarSize = spec.CentralZoneBarSpec.BarSize
//        let czDepth = VerticalBars.CentralZone.getCentralZoneDepth dimsAsTuple ezDepth ezClearBarSpacing
//        let czSpacing = spec.CentralZoneBarSpec.Spacing
//        let horizBarSpacing = spec.HorizontalBarSpec.Spacing

//        let ezBars = VerticalBars.EndZone.create ezBarSize ezDepth dimsAsTuple horizBarSize steel
//        let czBars = VerticalBars.CentralZone.create czBarSize czSpacing czDepth horizBarSize dimsAsTuple steel
//        let horizBars = HorizontalBars.create dimensions horizBarSpacing horizBarSize steel

//        { Dimensions = dimensions
//          HorizontalBars = horizBars
//          EndZoneBars = ezBars
//          CentralZoneBars = czBars }
