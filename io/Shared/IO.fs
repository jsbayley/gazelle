namespace Gazelle.IO

open System
open System.IO
open System.Text.Json

module IO =

  /// Generic HOF that prompts for user input and validates it against a given validation function.
  let askForUserInput<'T>
    (msgPrompt: string)
    (validateInput: string -> Result<'T, IOError>)
    : Result<'T, IOError> =
    Console.Clear()
    let mutable validInput = false
    let mutable result: (Result<'T, IOError>) = Error BadUserInput

    while not validInput do
      Messages.prompt msgPrompt
      let input = Console.ReadLine()
      let validation = validateInput input

      match validation with
      | Ok value ->
        validInput <- true
        result <- Ok value
      | Error e ->
        IOError.print e
        result <- Error BadUserInput

    Console.Clear()
    result

  /// Reads in FilePath and deserializes to 'T using specified 'deserializer' function.
  let readFileAndDeserialize<'T>
    (deserialize: string -> 'T)
    (f: FilePath)
    : Result<'T, IOError> =
    try
      Unwrap.filepath f |> File.ReadAllText |> deserialize |> Ok
    with :? JsonException ->
      Error(DeserializationError "Malformed JSON file")

  /// Attempts to deserialize given string to Json.
  let deserializeJson<'T> (s: string) : 'T = JsonSerializer.Deserialize<'T> s

  /// Validates that a given directory path is neither null, empty, or not found.
  let validateDirectoryPath (path: string) : Result<DirectoryPath, IOError> =
    match path with
    | p when String.IsNullOrEmpty p -> Error(PathError "Provided empty path")
    | p when not (Directory.Exists p) -> Error(PathError "Directory not found")
    | _ -> Ok(DirectoryPath path)

  /// Checks given file extension against list of permitted extensions.
  let checkFileExtension
    (f: FilePath)
    (validExtensions: string list)
    : Result<string, IOError> =
    let (FilePath f) = f

    match Path.GetExtension f with
    | ext when List.contains (Path.GetExtension ext) validExtensions -> Ok ext
    | ext when String.IsNullOrEmpty ext ->
      Error(FileExtensionError "Provided file has no valid extension")
    | _ -> Error(FileExtensionError "No valid file extension found")

  [<RequireQualifiedAccess>]
  module FileWatcher =

    /// Prompts user for their 'working directory' path and validates input.
    let queryWorkingDirectory () : Result<DirectoryPath, IOError> =
      askForUserInput "Enter path to working directory..." validateDirectoryPath

    /// Configures and returns a valid FileSystemWatcher that monitors the 'working directory'.
    /// Accepts a 'handler' callback function that is invoked whenever a 'change' event fires.
    let create
      (watchedDirectory: Result<DirectoryPath, IOError>)
      (onChangeHandler: FileSystemEventArgs -> unit)
      : unit =
      match watchedDirectory with
      | Ok dir ->
        let (DirectoryPath watchedDirectory) = dir
        let fileWatcher = new FileSystemWatcher()
        fileWatcher.Path <- watchedDirectory
        fileWatcher.EnableRaisingEvents <- true
        fileWatcher.IncludeSubdirectories <- true

        fileWatcher.NotifyFilter <-
          (fileWatcher.NotifyFilter ||| NotifyFilters.LastWrite)

        fileWatcher.Changed.Add(onChangeHandler)
        Messages.info $"Watching for file changes in {watchedDirectory}...\n"
      | Error e -> IOError.print e

    /// When a 'change' event is fired by the FileWatcher, (1) parse the changed .json
    /// or .config file, (2) deserialize the contents and bind against generic type 'T,
    /// and (3) update the ETABS model to reflect the changes.
    // let onChangeUpdateModel<'T, 'U> (deserialize: string -> Result<'T, IOError>) (validate: 'T -> Result<'U, seq<ValidationResult>>) (render: 'U -> unit) =
    let onChangeUpdateModel<'T, 'U>
      (deserialize: string -> 'T)
      (validate: 'T -> Result<'U, seq<ModelValidationResult>>)
      (render: 'U -> unit)
      =

      /// When a 'change' event is fired by the FileWatcher, (1) get the fullpath
      /// of the changed item, (2) check that the file extension is either .json,
      /// .config, or .yaml (3) read in the file, and (4) deserialize it.
      let onChangeParseConfigFileAndDeserialize
        (deserialize: string -> 'T)
        (e: FileSystemEventArgs)
        : Result<'T, IOError> =
        let path = e.FullPath |> FilePath

        match checkFileExtension path [ ".json"; ".yaml" ] with
        | Ok _ ->
          let result = readFileAndDeserialize<'T> deserialize path

          match result with
          | Ok r -> Ok r
          | Error e -> Error e
        | Error e -> Error e

      /// Validates and renders a given model. The function accepts a higher-order 'render' function to allow
      /// for extensible and flexible integration with various 3rd party applications, e.g. ETABS, Revit, Ram Concept etc.
      let updateModel
        (validate: 'T -> Result<'U, seq<ModelValidationResult>>)
        (render: 'U -> unit)
        (model: Result<'T, IOError>)
        : unit =
        match model with
        | Ok serializedModel ->
          match (validate serializedModel) with
          | Ok model -> render model
          | Error e -> ModelValidationResult.printErrors e
        | Error e -> IOError.print e

      (onChangeParseConfigFileAndDeserialize deserialize)
      >> (updateModel validate render)
