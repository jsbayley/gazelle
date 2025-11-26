namespace Gazelle.IO

open System

[<RequireQualifiedAccess>]
module Messages =

  /// Formatted message to display useful info to user.
  let info (msg: string) : unit =
    Console.ForegroundColor <- ConsoleColor.Green
    printf "Info: "
    Console.ResetColor()
    printfn "%s" msg

  /// Formatted message to display when user input is required.
  let prompt (msg: string) : unit =
    Console.ForegroundColor <- ConsoleColor.DarkYellow
    printf "Input Required: "
    Console.ResetColor()
    printfn "%s" msg

  /// Formatted message to display on successful event.
  let success (msg: string) : unit =
    Console.ForegroundColor <- ConsoleColor.Green
    printf "Success: "
    Console.ResetColor()
    printfn "%s" msg

  /// Formatted message to display when process is aborted.
  let abort (msg: string) : unit =
    Console.ForegroundColor <- ConsoleColor.Magenta
    printf "Abort: "
    Console.ResetColor()
    printfn "%s" msg

  /// Formatted message to display on erroneous event.
  let error (errorType: string) (msg: string) : unit =
    Console.ForegroundColor <- ConsoleColor.Red
    printf $"{errorType}: "
    Console.ResetColor()
    printfn "%s" msg
