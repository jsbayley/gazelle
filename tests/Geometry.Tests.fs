namespace Gazelle.Geometry.Tests

open System
open Xunit

module GeometryTests =

  [<Fact>]
  let ``Mock Test`` () = Assert.True(true)

//     module CuboidTests =

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length greater than 0mm has perimeter of 800mm`` () =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 800.0<mm>

//             let actual =
//                 Geometry.Query.Solid3D.getPerimeter (Cuboid c) |> Geometry.Unwrap.perimeter

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length greater than 0mm has area of 40_000mm^2`` () =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 40_000.0<mm^2>

//             let actual =
//                 Geometry.Query.Solid3D.getCrossSectionalArea (Cuboid c)
//                 |> Geometry.Unwrap.crossSectionalArea

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length of 1000mm has volume of 40_000_000mm^3`` () =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 40_000_000.0<mm^3>
//             let actual = Geometry.Query.Solid3D.getVolume (Cuboid c) |> Geometry.Unwrap.volume
//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length of 1000mm has 2nd-moment about XX of 133_333_333.3mm^4``
//             ()
//             =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 133_333_333.3

//             let actual =
//                 Geometry.Query.Solid3D.getSecondMomentOfArea XX (Cuboid c)
//                 |> Geometry.Unwrap.secondMomentOfArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length of 1000mm has 2nd-moment about YY of 133_333_333.3mm^4``
//             ()
//             =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 133_333_333.3

//             let actual =
//                 Geometry.Query.Solid3D.getSecondMomentOfArea YY (Cuboid c)
//                 |> Geometry.Unwrap.secondMomentOfArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length of 1000mm has surface area (excl. ends) of 800_000mm^2``
//             ()
//             =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 800_000.0<mm^2>

//             let actual =
//                 Geometry.Query.Solid3D.getSurfaceArea ExcludeEndFaces (Cuboid c)
//                 |> Geometry.Unwrap.surfaceArea

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cuboid with width and depth of 200mm and length of 1000mm has surface area (incl. ends) of 880_000mm^2``
//             ()
//             =
//             let c =
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 1000.0<mm>

//             let expected = 880_000.0<mm^2>

//             let actual =
//                 Geometry.Query.Solid3D.getSurfaceArea IncludeEndFaces (Cuboid c)
//                 |> Geometry.Unwrap.surfaceArea

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cuboid with width less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } -200.0<mm> 200.0<mm> 1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cuboid with width equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 0.0<mm> 200.0<mm> 1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cuboid with depth less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> -200.0<mm> 1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cuboid with depth equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 0.0<mm> 1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cuboid with length less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> -1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cuboid with length equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cuboid { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 200.0<mm> 0.0<mm>
//                 |> ignore)

//     module CylinderTests =

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length greater than 0mm has perimeter of 200π mm`` () =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 200.0<mm> * Math.PI

//             let actual =
//                 Geometry.Query.Solid3D.getPerimeter (Cylinder c) |> Geometry.Unwrap.perimeter

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length greater than 0mm has area of 10_000π mm^2`` () =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 10_000.0<mm^2> * Math.PI

//             let actual =
//                 Geometry.Query.Solid3D.getCrossSectionalArea (Cylinder c)
//                 |> Geometry.Unwrap.crossSectionalArea

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length of 1000mm has volume of 31_415_926.5 mm^3`` () =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 31_415_926.5

//             let actual =
//                 Geometry.Query.Solid3D.getVolume (Cylinder c)
//                 |> Geometry.Unwrap.volume
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length of 1000mm has 2nd-moment about XX of 78_539_816.3 mm^4`` () =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 78_539_816.3

//             let actual =
//                 Geometry.Query.Solid3D.getSecondMomentOfArea XX (Cylinder c)
//                 |> Geometry.Unwrap.secondMomentOfArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length of 1000mm has 2nd-moment about YY of 78_539_816.3 mm^4`` () =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 78_539_816.3

//             let actual =
//                 Geometry.Query.Solid3D.getSecondMomentOfArea YY (Cylinder c)
//                 |> Geometry.Unwrap.secondMomentOfArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length of 1000mm has surface area (excl. ends) of 628_318.5 mm^2``
//             ()
//             =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 628_318.5

//             let actual =
//                 Geometry.Query.Solid3D.getSurfaceArea ExcludeEndFaces (Cylinder c)
//                 |> Geometry.Unwrap.surfaceArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cylinder with diameter of 200mm and length of 1000mm has surface area (incl. ends) of 691_150.4 mm^2``
//             ()
//             =
//             let c = Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 1000.0<mm>
//             let expected = 691_150.4

//             let actual =
//                 Geometry.Query.Solid3D.getSurfaceArea IncludeEndFaces (Cylinder c)
//                 |> Geometry.Unwrap.surfaceArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Cylinder with diameter less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } -200.0<mm> 1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cylinder with diameter equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 0.0<mm> 1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cylinder with length less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> -1000.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Cylinder with length equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.cylinder { X = 0.0<mm>; Y = 0.0<mm> } 200.0<mm> 0.0<mm>
//                 |> ignore)

//     module RectangularHoopTests =

//         [<Fact>]
//         let ``Rectangular Hoop with width and depth of 200mm has length of 800mm`` () =
//             let rh =
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     200.0<mm>
//                     10.0<mm>

//             let expected = 800.0<mm>

//             let actual =
//                 Geometry.Query.Hoop3D.getLength (RectangularHoop rh) |> Geometry.Unwrap.length

//             Assert.Equal(expected, actual)

//         [<Fact>]
//         let ``Rectangular Hoop with leg diameter of 10mm has sectional area of 78.5mm^2`` () =
//             let rh =
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     200.0<mm>
//                     10.0<mm>

//             let expected = 78.5

//             let actual =
//                 Geometry.Query.Hoop3D.getCrossSectionalAreaOfLeg (RectangularHoop rh)
//                 |> Geometry.Unwrap.crossSectionalArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Rectangular Hoop with width and depth of 200mm and leg diameter of 10mm has volume 62832mm^3`` () =
//             let rh =
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     200.0<mm>
//                     10.0<mm>

//             let expected = 62831.9

//             let actual =
//                 Geometry.Query.Hoop3D.getVolume (RectangularHoop rh)
//                 |> Geometry.Unwrap.volume
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Rectangular Hoop with width less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     -200.0<mm>
//                     200.0<mm>
//                     10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Rectangular Hoop with width equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     0.0<mm>
//                     200.0<mm>
//                     10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Rectangular Hoop with depth less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     -200.0<mm>
//                     10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Rectangular Hoop with depth equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     0.0<mm>
//                     10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Rectangular Hoop with leg diameter less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     200.0<mm>
//                     -10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Rectangular Hoop with leg diameter equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.rectangularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     200.0<mm>
//                     0.0<mm>
//                 |> ignore)

//     module CircularHoopTests =

//         [<Fact>]
//         let ``Circular Hoop with diameter of 200mm has length of 628.3mm`` () =
//             let ch =
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     10.0<mm>

//             let expected = 628.3

//             let actual =
//                 Geometry.Query.Hoop3D.getLength (CircularHoop ch)
//                 |> Geometry.Unwrap.length
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Circular Hoop with leg diameter of 10mm has sectional area of 78.5mm^2`` () =
//             let ch =
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     10.0<mm>

//             let expected = 78.5

//             let actual =
//                 Geometry.Query.Hoop3D.getCrossSectionalAreaOfLeg (CircularHoop ch)
//                 |> Geometry.Unwrap.crossSectionalArea
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Circular Hoop with diameter of 200mm and leg diameter of 10mm has volume 49348.0mm^3`` () =
//             let ch =
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     10.0<mm>

//             let expected = 49348.0

//             let actual =
//                 Geometry.Query.Hoop3D.getVolume (CircularHoop ch)
//                 |> Geometry.Unwrap.volume
//                 |> Units.Annotate.stripUnitsFromFloat

//             Assert.Equal(expected, actual, 1)

//         [<Fact>]
//         let ``Circular Hoop with diameter less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     -200.0<mm>
//                     10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Circular Hoop with diameter equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     0.0<mm>
//                     10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Circular Hoop with leg diameter less than 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     -10.0<mm>
//                 |> ignore)

//         [<Fact>]
//         let ``Circular Hoop with leg diameter equal to 0mm is invalid`` () =
//             Assert.Throws<ArgumentException>(fun () ->
//                 Geometry.Create.circularHoop
//                     { X = 0.0<mm>
//                       Y = 0.0<mm>
//                       Z = 0.0<mm> }
//                     200.0<mm>
//                     0.0<mm>
//                 |> ignore)
