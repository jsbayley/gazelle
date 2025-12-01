<div align="center">
  <img src="./.github/assets/gazelle.png" width="100px" height="100px" />
  <h1>Gazelle</h1>
  <p>🦌 A Fast Engine for Structural Engineering. 💨</p>
  
  [![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/jsbayley/gazelle)
  [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](https://github.com/jsbayley/gazelle/blob/main/.github/CODE_OF_CONDUCT.md)

  [![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-ce412b)](https://choosealicense.com/licenses/agpl-3.0/)
  [![CI](https://github.com/jsbayley/gazelle/actions/workflows/ci.yml/badge.svg)](https://github.com/jsbayley/gazelle/actions/workflows/ci.yml)

  [![.NET](https://img.shields.io/badge/.NET-9.0.306-512bd4)](https://dotnet.microsoft.com/en-us/)
</div>

## Table of Contents

- [Why?](#why)
- [What Is Gazelle?](#what-is-gazelle)
- [Why Choose F#?](#why-choose-f)
- [Releases](#releases)
- [Roadmap](#roadmap)
- [Join Our Community](#join-our-community)
- [Quick Start](#quick-start)
  - [Option A: Download a binary](#option-a-download-a-binary)
  - [Option B: Build from source](#option-b-build-from-source)
- [Project Values](#project-values)
- [Documentation](#documentation)
  - [API Reference](#api-reference)
  - [CLI Commands](#cli-commands)
- [Get Involved](#get-involved)
- [Open Source](#open-source)
- [Security](#security)
- [Errata](#errata)
- [FAQ](#faq)
- [Contributors](#contributors)

## Why?

<p align="justify">
I got caught up in a lot of analysis paralysis over language selection for a while, hence the project paused for quite some time. I've since decided to take a practical approach of getting Gazelle out there, seeing what the public reaction is, and go from there.
</p>

## What Is Gazelle?

<p align="justify">
Gazelle is a library and Command-Line Interface (CLI) written in F# and running on .NET 9. It favours a batteries-included approach, with intuitive CLI commands, structured outputs, and ETABS I/O support (windows-only). Gazelle is optimised for automation, CI pipelines, and workflow orchestration using peripheral command-line tools or AI orchestrators.
</p>

<p align="justify">
We support a CLI for two reasons:
</p>

<p align="justify">
  <ol>
    <li>
      Standard input/output mechanics of compiled tools (<code>stdin</code>/<code>stdout</code>) mean they can interoperate easily with other languages and technology stacks,
    </li>
    <li>
      Eventually, we intend to layer a Daemon-style service and/or AI-orchestrator on top for advanced workflows. 
    </li>
  </ol>
</p>

## Why Choose F#?

<p align="justify">
I originally selected F# because of its <a href="https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/units-of-measure">Units-of-Measure</a> feature and the interop with existing engineering applications offered by .NET. The strictness of the F# compiler makes it ideal for maintaining correctness. For safety-critical software development, compiler strictness helps to ensure structural integrity. The advanced domain modelling and type system characteristics of F# also allow for elegant expression of complex algorithms, which ultimately improves clarity, maintainability, and readability.
</p>

<p align="justify">
The foundational aspiration for Gazelle is to be a central library (and CLI) for Structural Engineering design, which captures the design equations, and other useful utilities that can be assembled into workflows for various engineering-related tasks. 
</p>

<p align="justify">
I got caught up in a lot of analysis paralysis for a while, hence why the project paused for quite some time. So, I've decided to take a practical approach of getting it out there, seeing what the reaction is from people, and whether or not it's potentially valuable.
</p>

## Releases

<p align="justify">
Gazelle is distributed via self-contained binaries for Windows, MacOS, and Linux at <a href="https://gazelle.sh">https://gazelle.sh</a>.
</p>

## Roadmap

We are currently defining the roadmap. Please keep an eye on our [GitHub repository](https://github.com/jsbayley/gazelle) for updates.

## Quick Start

### Option A: Download a binary
- Website: https://gazelle.sh
  - gz.win-x64.exe
  - gz.osx-x64
  - gz.osx-arm64
  - gz.linux-x64
  - gz.linux-arm64

macOS/Linux: add execute bit if needed
```bash
chmod +x gz.osx-arm64  # or gz.osx-x64 / gz.linux-*
./gz.osx-arm64 --help
```

Verify checksum
```bash
sha256sum -c gz.osx-arm64.sha256  # Linux
shasum -a 256 -c gz.osx-arm64.sha256  # macOS
```

Windows (PowerShell)
```powershell
Get-FileHash .\gz.win-x64.exe -Algorithm SHA256
.\gz.win-x64.exe --help
```

### Option B: Build from source
```bash
git clone https://github.com/jsbayley/gazelle.git
cd gazelle
dotnet build
dotnet test
dotnet publish cli/Gazelle.CLI.fsproj -c Release -r linux-arm64 --self-contained -p:PublishSingleFile=true -o ./artifacts/
./artifacts/gz --help
```

## Project Values

Gazelle is opinionated by design. These **nine** principles guide every decision in the project and set clear expectations for engineers and contributors.

1. **Privacy-First:** All computation happens locally. Your data stays on your machine.
2. **Radical Transparency:** Algorithms must be readable, verifiable, and open to scrutiny.
3. **Open Collaboration:** Open code, discussion, and shared learning strengthen our profession.
4. **Reliability Matters:** Stability, correctness, and deterministic behaviour matter more than novelty.
5. **Type-Safe by Design:** F# Units-of-Measure and strong typing ensure mistakes surface at compile time.
6. **Designed for Humans:** Simplicity, clarity, and flow are core design goals to reduce cognitive load.
7. **Optimised for AI:** Clean CLI interfaces and structured data formats are ideal for AI orchestration.
8. **Small Composable Tools:** Unix-style components combine into workflows.
9. **Performance as a Feature:** Lean, fast, low-friction engineering binaries.

## Documentation

Visit our [docs](./DOCS.md) to learn more.

### API Reference

The comprehensive API reference for the Gazelle library is documented in [API.md](./docs/API.md).

### CLI Commands

The full list of available CLI commands are documented in [CLI.md](./docs/CLI.md).

## Join Our Community

Gazelle is a **community-led** project designed to:

- 🚀 Accelerate AEC innovation,
- 📚 Underpin academic research,
- 🎓 Support Structural Engineering education, and
- 🫱🏻‍🫲🏾 Connect like-minded Engineers.

Please ⭐️ and 'watch' this repository so that you can track its progress in real-time. You can also get involved in the [discussions](https://github.com/jsbayley/gazelle/discussions) where the design decisions for the future of Gazelle are conducted in the open.

## Get Involved

See our guidance on [how to get involved](.github/CONTRIBUTING.md). We encourage contributions and will happily support you to take your first steps into Open Source.

## Open Source

<p align="justify">
  Engineers accept phenomenal responsibility when dedicating their lives to improving our built environment. However, the vast majority of professional engineering software is, regrettably, closed source and proprietary. This is unfair and must change. Engineers should be offered the respect and freedom to inspect, validate and influence the algorithms used to design our buildings and bridges. 
</p>

<p align="justify">
  Gazelle is proudly <a href="./LICENSE" target="_blank">open source</a>.
</p>

## Security

Review our [security policy](./.github/SECURITY.md) for reporting vulnerabilities.

## Errata

<p align="justify">
  We consider structural engineering software to be safety critical. We strive to ensure stability, robustness and correctness throughout the source code, test suite and companion documentation. Nevertheless, we are human and mistakes are possible. Please submit error reports and suggestions for improvement via <a href="https://github.com/jsbayley/gazelle/issues" target="_blank">GitHub Issues</a>.
</p>

## FAQ

Review our [FAQ](./docs/FAQ.md) for answers to popular questions.

## Contributors

Thanks to those who have [contributed](https://github.com/jsbayley/gazelle/graphs/contributors) to Gazelle and help to keep the project moving forwards:

<div align="center">
  <a href="https://github.com/jsbayley">
    <img src="https://github.com/jsbayley.png?size=64" width="64" height="64" alt="@jsbayley" />
  </a>
  <a href="https://github.com/goswinr">
    <img src="https://github.com/goswinr.png?size=64" width="64" height="64" alt="@goswinr" />
  </a>
  <a href="https://github.com/danayet">
    <img src="https://github.com/danayet.png?size=64" width="64" height="64" alt="@danayet" />
  </a>
</div>

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
