namespace Gazelle.IO

[<RequireQualifiedAccess>]
module Unwrap =

  // Helper functions to unwrap single-case union types.
  let positiveInt (PositiveInt i) = i
  let positiveFloat (PositiveFloat i) = i
  let elevation (Elevation e) = e
  let height (Height h) = h
  let heightAndPositiveFloat = height >> positiveFloat
  let name (Name n) = n
  let filepath (FilePath p) = p

  let positiveIntegerRange (r: PositiveIntegerRange) : (int * int) =
    let s = positiveInt r.Start
    let e = positiveInt r.End
    s, e
