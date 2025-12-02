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

- [What Is Gazelle?](#what-is-gazelle)
- [Why Build Gazelle?](#why-build-gazelle)
- [A Note From Our Creator](#a-note-from-our-creator)
- [Project Values](#project-values)
- [Why F#?](#why-f)
- [Releases](#releases)
- [Roadmap](#roadmap)
- [Join Our Community](#join-our-community)
- [Quick Start](#quick-start)
  - [Option A: Download a binary](#option-a-download-a-binary)
  - [Option B: Build from source](#option-b-build-from-source)
- [Documentation](#documentation)
  - [API Reference](#api-reference)
  - [CLI Commands](#cli-commands)
- [Get Involved](#get-involved)
- [Open Source](#open-source)
- [Security](#security)
- [Errata](#errata)
- [FAQ](#faq)
- [Contributors](#contributors)

## What Is Gazelle?

<p align="justify">
Gazelle is a library and Command-Line Interface (CLI) written in F# and running on .NET 9. It favours a batteries-included approach, with intuitive CLI commands, structured outputs, and ETABS I/O support (windows-only). Gazelle is optimised for automation, CI pipelines, and workflow orchestration using peripheral command-line tools or AI orchestrators. The standard input/output mechanics of CLI tools (<code>stdin</code>/<code>stdout</code>) allows them to interoperate easily with other languages and technology stacks, which opens up the ecosystem for collaboration.
</p>

## Why Build Gazelle?

Gazelle is, first and foremost, the tool we wish existed:

- Fast,
- Composable,
- Transparent, 
- Minimal,
- Local,
- Type-safe, and
- Crafted with intention.

The project is the purest expression of our love for:

- Clarity,
- Elegance,
- Order,
- Purpose, 
- Precision, and
- Experience Design.

Frustrated by the friction and opacity of traditional tools, and an admiration for the Unix Philosophy of small, focussed, composable tools, we set out on a journey to build a modern equivalent for the Structural Engineer. Gazelle aims to play nicely with the wider ecosystem, offer a stable and reliable platform for others to build on top of, and ultimately be a tool of choice for Engineers everywhere. We want to build something that Engineers love to use.

## A Note From Our Creator

<p align="justify">
Gazelle is the clearest, most honest expression of my craft, my philosophy, and my desire to create a better way for engineers to think and work. It sits at the exact intersection of who I am, how I think, and what I wish existed in the world. It's ultimately a form of self-expression. Gazelle is my way of showing that AEC tools can be more humane.  We don’t need bloated GUIs to do serious engineering. We can centre experience design in a field that ignores it. We can open-source safety-critical tooling and still maintain trust.
</p>

<p align="justify">
I am humbled and energised to see so many people take an interest in the project and follow its evolution. I welcome and encourage as many of you as possible to <a href="#get-involved">get involved</a> and <a href="#join-our-community">join our community</a>. If you love Gazelle and wish to support, you can also <a href="https://github.com/sponsors/jsbayley">sponsor</a> its development here on GitHub.
</p>

<p align="justify">
I appreciate each and everyone of you. Your advice and suggestions help to shape the strategic roadmap for Gazelle. If you'd like to chat, you can always reach me on <a href="https://www.linkedin.com/in/jsbayley">LinkedIn</a>. Please use our <a href="https://github.com/jsbayley/gazelle/discussions">discussions</a> forum to meet others in our community.
</p>

<p align="justify">
🦌 James 💨
</p>

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

## Why F#?

<p align="justify">
We primarily selected F# because of its <a href="https://learn.microsoft.com/en-us/dotnet/fsharp/language-reference/units-of-measure">Units-of-Measure</a> feature, the language's succinctness for expressing mathematical formulae, and the interop with existing engineering applications offered by .NET. The strictness of the F# compiler makes it ideal for maintaining correctness. For safety-critical software development, compiler strictness helps to ensure structural integrity. The advanced domain modelling and type system characteristics of F# also allow for elegant expression of complex algorithms, which ultimately improves clarity, maintainability, and readability.
</p>

## Releases

<p align="justify">
Gazelle is distributed via self-contained binaries for Windows, MacOS, and Linux at <a href="https://gazelle.sh">https://gazelle.sh</a>.
</p>

## Roadmap

We are currently defining the roadmap. Please keep an eye on our [GitHub repository](https://github.com/jsbayley/gazelle) for updates.

## Quick Start

### Option A: Download a binary
1. Visit <a href="https://gazelle.sh">Gazelle.sh</a> and <a href="https://gazelle.sh#download">download</a> one of the following platform-specific binaries:
  - gz.win-x64.exe
  - gz.osx-x64
  - gz.osx-arm64
  - gz.linux-x64
  - gz.linux-arm64

2. For MacOS and Linux Users, enable binary execution permissions:

```bash
# Linux (x64)
chmod +x gz.linux-x64
```

```bash
# MacOS (ARM64)
chmod +x gz.osx-arm64
```

3. For security reasons to ensure file integrity and no tampering has occurred, verify the checksum:

```bash
# Linux
sha256sum -c gz.osx-arm64.sha256
```

```bash
# MacOS
shasum -a 256 -c gz.osx-arm64.sha256
```

```powershell
# Windows (PowerShell)
Get-FileHash .\gz.win-x64.exe -Algorithm SHA256
```

4. Fire up the help docs:

```bash
# Linux (x64)
./gz.linux-x64 --help
```

```bash
# MacOS (ARM64)
./gz.osx-arm64 --help
```

```powershell
# Windows (PowerShell)
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
