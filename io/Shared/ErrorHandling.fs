namespace Gazelle.IO

type IOError =
  | BadUserInput
  | UnsupportedVersion of string
  | PathError of string
  | FileExtensionError of string
  | DeserializationError of string

type RangeError =
  | NonPositiveStartValue
  | NonPositiveEndValue
  | StartGreaterThanEnd
  | NonConsecutiveRange

type HeightError =
  | HeightLessThanZero
  | HeightEqualToZero
  | InvalidHeight

type ModelValidationResult =
  | ValidModel
  | StoreyHeightError of HeightError
  | StoreyRangeError of RangeError

[<RequireQualifiedAccess>]
module IOError =

  let getAsString (e: IOError) : string =
    match e with
    | BadUserInput -> "Bad User Input."
    | UnsupportedVersion msg -> $"Unsupported Version: {msg}."
    | PathError msg -> $"Path Error: {msg}."
    | FileExtensionError msg -> $"File Extension Error: {msg}."
    | DeserializationError msg -> $"Deserialization Error: {msg}."

  let print (error: IOError) =
    error |> getAsString |> Messages.error "IOError"

[<RequireQualifiedAccess>]
module ModelValidationResult =

  module RangeError =

    let getAsString (e: RangeError) : string =
      match e with
      | NonPositiveStartValue ->
        "Non-positive start value given for storey range."
      | NonPositiveEndValue -> "Non-positive end value given for storey range."
      | StartGreaterThanEnd -> "Range start value is greater than end value."
      | NonConsecutiveRange -> "Adjacent storey ranges are non-consecutive."

  module HeightError =

    let getAsString (e: HeightError) : string =
      match e with
      | HeightLessThanZero -> "Storey height is less than zero."
      | HeightEqualToZero -> "Storey height is equal to zero."
      | InvalidHeight -> "Invalid storey height."

  let getAsString (e: ModelValidationResult) : string =
    match e with
    | ValidModel -> "Valid Model."
    | StoreyRangeError e -> RangeError.getAsString e
    | StoreyHeightError e -> HeightError.getAsString e

  let printErrors (errors: seq<ModelValidationResult>) : unit =
    errors
    |> Seq.map getAsString
    |> Seq.filter (fun e -> e <> "Valid Model.")
    |> Seq.iter (Messages.error "Model Validation Error")
