// Gazelle: a cross-platform engine for structural analysis & design.
open System
open System.IO
open System.Text.Json
open System.Text.Json.Serialization
open Spectre.Console

// Types
type CliOptions =
  { Command: string
    InputFile: string option
    OutputFile: string option
    Format: string
    Verbose: bool
    Detailed: bool
    Template: string option
    Parameters: string option
    OutputDir: string option
    Progress: bool
    Workers: int
    Help: bool }

type ModelInfo =
  { Name: string
    Version: string
    NodeCount: int
    ElementCount: int
    LoadCases: int }

type AnalysisResult =
  { ModelName: string
    Status: string
    MaxDisplacement: float option
    MaxStress: float option
    Warnings: string[]
    Errors: string[] }

type ValidationResult =
  { IsValid: bool
    Errors: string[]
    Warnings: string[] }

type Template =
  { Name: string
    Description: string
    Parameters: string[] }

// JSON serialization helpers
let private jsonOptions =
  let options = JsonSerializerOptions()
  options.PropertyNamingPolicy <- JsonNamingPolicy.CamelCase
  options.WriteIndented <- true
  options.DefaultIgnoreCondition <- JsonIgnoreCondition.WhenWritingNull
  options

let serialize<'T> (value: 'T) : string =
  JsonSerializer.Serialize(value, jsonOptions)

let serializeToFile<'T> (filePath: string) (value: 'T) =
  let json = serialize value
  File.WriteAllText(filePath, json)

// Default options
let defaultOptions =
  { Command = ""
    InputFile = None
    OutputFile = None
    Format = "text"
    Verbose = false
    Detailed = false
    Template = None
    Parameters = None
    OutputDir = None
    Progress = false
    Workers = Environment.ProcessorCount
    Help = false }

// Available templates
let templates =
  [ { Name = "beam"
      Description = "Simple beam structure"
      Parameters = [| "span"; "load" |] }
    { Name = "truss"
      Description = "Truss structure"
      Parameters = [| "width"; "height"; "loads" |] }
    { Name = "frame"
      Description = "Portal frame structure"
      Parameters = [| "width"; "height"; "loads" |] } ]

let showHelp () =
  AnsiConsole.WriteLine()

  let rule =
    Rule(
      "[bold blue]🦌 Gazelle - A Fast Engine for Structural Engineering 💨[/]"
    )

  rule.Style <- Style.Parse("blue")
  AnsiConsole.Write(rule)
  AnsiConsole.WriteLine()

  let grid = Grid()
  grid.AddColumn() |> ignore
  grid.AddColumn() |> ignore

  grid.AddRow(
    "[yellow]USAGE:[/]",
    "[white]gz[/] [blue]<command>[/] [grey][[options]][/]"
  )
  |> ignore

  grid.AddEmptyRow() |> ignore

  grid.AddRow("[yellow]COMMANDS:[/]", "") |> ignore

  grid.AddRow("  [green]info[/] [cyan]<model>[/]", "Show model information")
  |> ignore

  grid.AddRow(
    "  [green]analyze[/] [cyan]<model>[/]",
    "Analyze structural model"
  )
  |> ignore

  grid.AddRow(
    "  [green]validate[/] [cyan]<model>[/]",
    "Validate model structure"
  )
  |> ignore

  grid.AddRow("  [green]create[/]", "Create new model from template") |> ignore

  grid.AddRow("  [green]templates[/] [cyan]list[/]", "List available templates")
  |> ignore

  grid.AddRow(
    "  [green]batch-analyze[/] [cyan]<pattern>[/]",
    "Analyze multiple models"
  )
  |> ignore

  grid.AddRow("  [green]help[/]", "Show this help") |> ignore
  grid.AddEmptyRow() |> ignore

  grid.AddRow("[yellow]GLOBAL OPTIONS:[/]", "") |> ignore

  grid.AddRow(
    "  [grey]--format[/] [cyan]<json|text>[/]",
    "Output format (default: text)"
  )
  |> ignore

  grid.AddRow("  [grey]--output[/] [cyan]<file>[/]", "Output file path")
  |> ignore

  grid.AddRow("  [grey]--verbose[/]", "Enable verbose output") |> ignore

  grid.AddRow("  [grey]--quiet[/]", "Suppress all output except errors")
  |> ignore

  grid.AddEmptyRow() |> ignore

  grid.AddRow("[yellow]EXAMPLES:[/]", "") |> ignore
  grid.AddRow("  [dim]gz info model.json --format json[/]", "") |> ignore

  grid.AddRow(
    "  [dim]gz analyze beam.json --output results.json --detailed[/]",
    ""
  )
  |> ignore

  grid.AddRow("  [dim]gz create --template truss --output model.json[/]", "")
  |> ignore

  AnsiConsole.Write(grid)
  AnsiConsole.WriteLine()

// Argument parsing
let rec parseArgs args options =
  match args with
  | [] -> options
  | "--help" :: _
  | "help" :: _ -> { options with Help = true }
  | "--format" :: format :: tail ->
    parseArgs tail { options with Format = format }
  | "--output" :: file :: tail ->
    parseArgs tail { options with OutputFile = Some file }
  | "--verbose" :: tail -> parseArgs tail { options with Verbose = true }
  | "--detailed" :: tail -> parseArgs tail { options with Detailed = true }
  | "--template" :: template :: tail ->
    parseArgs
      tail
      { options with
          Template = Some template }
  | "--params" :: paramsFile :: tail ->
    parseArgs
      tail
      { options with
          Parameters = Some paramsFile }
  | "--output-dir" :: dir :: tail ->
    parseArgs tail { options with OutputDir = Some dir }
  | "--progress" :: tail -> parseArgs tail { options with Progress = true }
  | "--workers" :: workers :: tail ->
    match Int32.TryParse workers with
    | (true, n) -> parseArgs tail { options with Workers = n }
    | (false, _) -> parseArgs tail options
  | cmd :: tail when not (cmd.StartsWith "--") && options.Command = "" ->
    // For commands that don't take a file argument (like 'create'), just set command
    if cmd = "create" || cmd = "templates" then
      parseArgs tail { options with Command = cmd }
    else
      // For commands that take a file, expect next argument to be file
      match tail with
      | file :: restTail when not (file.StartsWith "--") ->
        parseArgs
          restTail
          { options with
              Command = cmd
              InputFile = Some file }
      | _ -> parseArgs tail { options with Command = cmd }
  | unknownArg :: tail ->
    // Skip unknown arguments but continue parsing
    parseArgs tail options

let parse (args: string[]) =
  let argsList = Array.toList args
  parseArgs argsList defaultOptions

// Modern output helpers with colors
let outputResult format content =
  match format with
  | "json" ->
    let json = serialize content
    let panel = Panel(json)
    panel.Header <- PanelHeader(" JSON Output ")
    panel.Border <- BoxBorder.Rounded
    panel.BorderStyle <- Style.Parse("green")
    AnsiConsole.Write(panel)
  | _ ->
    let table = Table()
    table.AddColumn("Property") |> ignore
    table.AddColumn("Value") |> ignore
    table.Border <- TableBorder.Rounded
    table.BorderStyle <- Style.Parse("blue")

    // Format content based on type
    match box content with
    | :? ModelInfo as model ->
      table.Title <- TableTitle("Model Information")
      table.AddRow("[cyan]Name[/]", model.Name) |> ignore
      table.AddRow("[cyan]Version[/]", model.Version) |> ignore
      table.AddRow("[cyan]Nodes[/]", model.NodeCount.ToString()) |> ignore
      table.AddRow("[cyan]Elements[/]", model.ElementCount.ToString()) |> ignore
      table.AddRow("[cyan]Load Cases[/]", model.LoadCases.ToString()) |> ignore
    | :? AnalysisResult as result ->
      table.Title <- TableTitle("Analysis Results")
      table.AddRow("[cyan]Model[/]", result.ModelName) |> ignore
      table.AddRow("[cyan]Status[/]", $"[green]{result.Status}[/]") |> ignore

      match result.MaxDisplacement with
      | Some d ->
        table.AddRow("[cyan]Max Displacement[/]", $"{d:F3} m") |> ignore
      | None -> ()

      match result.MaxStress with
      | Some s -> table.AddRow("[cyan]Max Stress[/]", $"{s:F1} MPa") |> ignore
      | None -> ()
    | :? ValidationResult as validation ->
      table.Title <- TableTitle("Validation Results")
      let statusColor = if validation.IsValid then "green" else "red"
      let statusText = if validation.IsValid then "✓ Valid" else "✗ Invalid"

      table.AddRow("[cyan]Status[/]", $"[{statusColor}]{statusText}[/]")
      |> ignore

      table.AddRow("[cyan]Errors[/]", validation.Errors.Length.ToString())
      |> ignore

      table.AddRow("[cyan]Warnings[/]", validation.Warnings.Length.ToString())
      |> ignore
    | _ ->
      let contentStr = content.ToString()
      table.AddRow("[cyan]Result[/]", contentStr) |> ignore

    AnsiConsole.Write(table)

let outputToFile format (filePath: string) content =
  AnsiConsole
    .Status()
    .Start(
      $"Writing to {Path.GetFileName(filePath)}...",
      fun ctx ->
        ctx.Spinner <- Spinner.Known.Star
        ctx.SpinnerStyle <- Style.Parse("green")

        match format with
        | "json" -> serializeToFile filePath content
        | _ ->
          let text = sprintf "%A" content
          File.WriteAllText(filePath, text)

        System.Threading.Thread.Sleep(500) // Brief pause to show spinner
    )

  AnsiConsole.MarkupLine($"[green]✓[/] Results written to [cyan]{filePath}[/]")

let showSuccess message =
  AnsiConsole.MarkupLine($"[green]✓[/] {message}")

let showError message =
  AnsiConsole.MarkupLine($"[red]✗[/] {message}")

let showWarning message =
  AnsiConsole.MarkupLine($"[yellow]⚠[/] {message}")

let showInfo message =
  AnsiConsole.MarkupLine($"[blue]ℹ[/] {message}")

let withProgress<'T> (message: string) (work: unit -> 'T) =
  AnsiConsole
    .Status()
    .Start(
      message,
      fun ctx ->
        ctx.Spinner <- Spinner.Known.Dots
        ctx.SpinnerStyle <- Style.Parse("yellow")
        work ()
    )

// Commands
let infoCommand (options: CliOptions) =
  match options.InputFile with
  | None ->
    showError "No model file specified"
    1
  | Some file when not (File.Exists file) ->
    showError $"Model file not found: {file}"
    1
  | Some file ->
    try
      // Mock implementation - replace with actual model loading
      let modelInfo =
        { Name = Path.GetFileNameWithoutExtension(file)
          Version = "1.0"
          NodeCount = 10
          ElementCount = 8
          LoadCases = 2 }

      match options.OutputFile with
      | Some outputFile -> outputToFile options.Format outputFile modelInfo
      | None -> outputResult options.Format modelInfo

      0
    with ex ->
      showError $"Error reading model: {ex.Message}"
      1

let analyzeCommand (options: CliOptions) =
  match options.InputFile with
  | None ->
    showError "No model file specified"
    1
  | Some file when not (File.Exists file) ->
    eprintfn "Error: Model file not found: %s" file
    1
  | Some file ->
    try
      if options.Verbose then
        showInfo $"Analyzing model: {file}"

      // Mock analysis - replace with actual analysis
      let result =
        { ModelName = Path.GetFileNameWithoutExtension(file)
          Status = "Success"
          MaxDisplacement = Some 0.025
          MaxStress = Some 145.2
          Warnings = [||]
          Errors = [||] }

      match options.OutputFile with
      | Some outputFile -> outputToFile options.Format outputFile result
      | None -> outputResult options.Format result

      0
    with ex ->
      showError $"Error during analysis: {ex.Message}"
      1

let validateCommand (options: CliOptions) =
  match options.InputFile with
  | None ->
    eprintfn "Error: No model file specified"
    1
  | Some file when not (File.Exists file) ->
    eprintfn "Error: Model file not found: %s" file
    1
  | Some file ->
    try
      // Mock validation - replace with actual validation
      let result =
        { IsValid = true
          Errors = [||]
          Warnings = [||] }

      if options.Verbose then
        printfn "Validating model: %s" file

      match options.OutputFile with
      | Some outputFile -> outputToFile options.Format outputFile result
      | None -> outputResult options.Format result

      if result.IsValid then 0 else 1
    with ex ->
      eprintfn "Error during validation: %s" ex.Message
      1

let createCommand (options: CliOptions) =
  match options.Template with
  | None ->
    showError "No template specified. Use --template <name>"
    1
  | Some templateName ->
    let template = templates |> List.tryFind (fun t -> t.Name = templateName)

    match template with
    | None ->
      eprintfn "Error: Template '%s' not found" templateName

      eprintfn
        "Available templates: %s"
        (String.Join(", ", templates |> List.map (fun t -> t.Name)))

      1
    | Some tmpl ->
      try
        if options.Verbose then
          printfn "Creating model from template: %s" templateName

        // Mock model creation - replace with actual implementation
        let newModel =
          { Name = sprintf "Generated_%s" templateName
            Version = "1.0"
            NodeCount = 4
            ElementCount = 3
            LoadCases = 1 }

        match options.OutputFile with
        | Some outputFile ->
          serializeToFile outputFile newModel
          printfn "Model created: %s" outputFile
        | None -> outputResult options.Format newModel

        0
      with ex ->
        eprintfn "Error creating model: %s" ex.Message
        1

let templatesCommand (options: CliOptions) =
  try
    match options.Format with
    | "json" ->
      let json = serialize templates
      let panel = Panel(json)
      panel.Header <- PanelHeader(" Available Templates ")
      panel.Border <- BoxBorder.Rounded
      panel.BorderStyle <- Style.Parse("green")
      AnsiConsole.Write(panel)
    | _ ->
      let table = Table()
      table.AddColumn("Template") |> ignore
      table.AddColumn("Description") |> ignore
      table.AddColumn("Parameters") |> ignore
      table.Border <- TableBorder.Rounded
      table.BorderStyle <- Style.Parse("blue")
      table.Title <- TableTitle("Available Templates")

      for template in templates do
        let parameters = String.Join(", ", template.Parameters)

        table.AddRow(
          $"[green]{template.Name}[/]",
          template.Description,
          $"[dim]{parameters}[/]"
        )
        |> ignore

      AnsiConsole.Write(table)

    0
  with ex ->
    showError $"Error listing templates: {ex.Message}"
    1

let batchAnalyzeCommand (options: CliOptions) =
  // Mock implementation - would process multiple files
  printfn "Batch analysis not yet implemented"
  0

let executeCommand (options: CliOptions) =
  match options.Command.ToLower() with
  | "info" -> infoCommand options
  | "analyze" -> analyzeCommand options
  | "validate" -> validateCommand options
  | "create" -> createCommand options
  | "templates" -> templatesCommand options
  | "batch-analyze" -> batchAnalyzeCommand options
  | "help"
  | "" ->
    showHelp ()
    0
  | cmd ->
    showError $"Unknown command '{cmd}'"
    showInfo "Use 'gz help' for available commands"
    1

[<EntryPoint>]
let main args =
  try
    let options = parse args

    if options.Help then
      showHelp ()
      0
    else
      executeCommand options
  with
  | :? FileNotFoundException as ex ->
    eprintfn "Error: File not found - %s" ex.Message
    1
  | :? UnauthorizedAccessException as ex ->
    eprintfn "Error: Access denied - %s" ex.Message
    1
  | :? ArgumentException as ex ->
    eprintfn "Error: Invalid argument - %s" ex.Message
    1
  | ex ->
    eprintfn "Error: %s" ex.Message
    1
