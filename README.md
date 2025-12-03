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
- [Why Build It?](#why-build-it)
- [Get Started](#get-started)
- [Releases](#releases)
- [Documentation](#documentation)
  - [Quick Start](./docs/DOCS.md#quick-start)
  - [Deep-Dive](./docs/DOCS.md)
  - [API Reference](./docs/API.md)
  - [CLI Commands](./docs/CLI.md)
  - [FAQ](./docs/FAQ.md)
- [Design Philosophy](#design-philosophy)
- [Project Values](#project-values)
- [Why F#?](#why-f)
- [A Note From Our Creator](#a-note-from-our-creator)
- [Join Our Community](#join-our-community)
- [Roadmap](#roadmap)
- [Get Involved](#get-involved)
- [Open Source](#open-source)
- [Security](#security)
- [Errata](#errata)
- [Help and Support](#help-and-support)
- [Our Contributors](#our-contributors)

## What Is Gazelle?

<p align="justify">
Gazelle is a .NET library and Command-Line Interface (CLI) written in F#. It favours a batteries-included approach with intuitive CLI commands, structured outputs, Eurocode design equations, and ETABS I/O support. Gazelle is optimised for automation, CI pipelines, and workflow orchestration using peripheral command-line tools or AI orchestrators. The standard I/O mechanics of CLI tools allow them to interoperate easily with other languages and technology stacks, which opens up the ecosystem for collaboration and innovation.
</p>

## Why Build It?

<p align="justify">
Frustrated by the friction and opacity of traditional engineering software, and an admiration for the Unix Philosophy of small, focussed, composable tools, we set out on a journey to build a modern equivalent for the Structural Engineer. Gazelle aims to play nicely with the wider ecosystem, offer a stable and reliable platform for others to build on top of, and ultimately be a tool of choice for Engineers everywhere. We want to build something that Engineers love to use.
</p>

## Get Started

<div align="justify">
  <p>New to Gazelle? Choose your pathway:</p>
  <ul>
    <li><strong>Structural Engineer?</strong> Read our <a href="./docs/DOCS.md#quick-start">Quick Start</a> guide.</li>
    <li><strong>Researcher?</strong> <a href="./.github/CONTRIBUTING.md">Help us improve Gazelle</a>.</li>
    <li><strong>Academic, Teacher or Student?</strong> <a href="#join-our-community">Join our community</a>.</li>
    <li><strong>Professional Institution?</strong> Open a <a href="https://github.com/jsbayley/gazelle/discussions">discussion</a> or message us on <a href="https://www.linkedin.com/in/jsbayley">LinkedIn</a>.</li>
    <li><strong>F# Programmer?</strong> Review our <a href="https://github.com/jsbayley/gazelle/issues">Issues</a> and <a href=".github/CONTRIBUTING.md">get involved</a>.</li>
    <li><strong>C# Programmer?</strong> Join this discussion: <a href="https://github.com/jsbayley/gazelle/discussions/161">Create C# wrapper...</a></li>
    <li><strong>Python Programmer?</strong> Join this discussion: <a href="https://github.com/jsbayley/gazelle/discussions/162">Wire up Fable transpilation...</a></li>
    <li><strong>JavaScript Programmer?</strong> Join this discussion: <a href="https://github.com/jsbayley/gazelle/discussions/162">Wire up Fable transpilation...</a></li>
    <li><strong>PowerShell User?</strong> Join this discussion: <a href="https://github.com/jsbayley/gazelle/discussions/160">Implement PowerShell module...</a></li>
    <li><strong>Keen to Sponsor?</strong> <a href="https://github.com/sponsors/jsbayley">Sponsor</a> development right here on GitHub.</li>
    <li><strong>Need Enterprise Support?</strong> Message us on <a href="https://www.linkedin.com/in/jsbayley">LinkedIn</a>.</li>
    <li><strong>Discovered a Vulnerability?</strong> Review our <a href="./.github/SECURITY.md">Security Policy</a>.</li>
    <li><strong>Found a Bug?</strong> Please file an <a href="https://github.com/jsbayley/gazelle/issues">Issue</a>.</li>
  </ul>
</div>

## Releases

<p align="justify">
Gazelle is distributed via self-contained binaries for Windows, MacOS, and Linux at <a href="https://gazelle.sh">https://gazelle.sh</a>.
</p>

## Documentation

<div align="justify">
  <p>Check out these resources to learn Gazelle.</p>
  <ul>
    <li><strong><a href="./docs/DOCS.md#quick-start">Quick Start Guide</a></strong>: Get started, fast! 💨</li>
    <li><strong><a href="./docs/DOCS.md">Documentation</a></strong>: A comprehensive deep-dive.</li>
    <li><strong><a href="./docs/CLI.md">CLI Commands</a></strong>: Explore all <code>gz</code> commands.</li>
    <li><strong><a href="./docs/API.md">API Reference</a></strong>: For .NET Developers.</li>
    <li><strong><a href="https://github.com/jsbayley/gazelle/discussions">Discussions</a></strong>: Where our community learns together.</li>
    <li><strong><a href="./docs/FAQ.md">FAQ</a></strong>: Answers to popular questions.</li>
  </ul>
</div>

## Design Philosophy

<p align="justify">
Gazelle is, first and foremost, the tool we wish existed:
</p>

- Fast,
- Local,
- Minimal,
- Type-safe,
- Composable,
- Transparent, and 
- Crafted with intention.

<p align="justify">
The project is the purest expression of our love for:
</p>

- Order,
- Purpose, 
- Clarity,
- Elegance,
- Precision, and
- Experience Design.

## Project Values

<p align="justify">
Gazelle is opinionated by design. These <strong>nine</strong> principles guide every decision in the project and set clear expectations for engineers and contributors.
</p>

1. **Privacy-First:** All computation happens locally. Your data stays with you.
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

## A Note From Our Creator

<p align="justify">
Gazelle is the clearest, most honest expression of my craft, my philosophy, and my desire to create a better way for engineers to think and work. It sits at the exact intersection of who I am, how I think, and what I wish existed in the world. It's ultimately a form of self-expression. Gazelle is my way of showing that AEC tools can be more humane.  We don’t need bloated GUIs to do serious engineering. We can centre experience design in a field that ignores it. We can open-source safety-critical tooling and still maintain trust.
</p>

<p align="justify">
I am humbled and energised to see so many people take an interest in the project and follow its evolution. I welcome and encourage as many of you as possible to <a href="#get-involved">get involved</a> and <a href="#join-our-community">join our community</a>. If you love Gazelle and wish to support, you can also <a href="https://github.com/sponsors/jsbayley">sponsor</a> its development here on GitHub.
</p>

<p align="justify">
I appreciate each and every one of you. Your advice and suggestions help to shape the strategic roadmap for Gazelle. If you'd like to chat, you can always reach me on <a href="https://www.linkedin.com/in/jsbayley">LinkedIn</a>. Please use our <a href="https://github.com/jsbayley/gazelle/discussions">discussions</a> forum to meet others in our community.
</p>

<p align="justify">
🦌 James 💨
</p>

## Join Our Community

Gazelle is a **community-led** project designed to:

- 🚀 Accelerate AEC innovation,
- 📚 Underpin academic research,
- 🎓 Support Structural Engineering education, and
- 🫱🏻‍🫲🏾 Connect like-minded Engineers.

Please ⭐️ and 'watch' this repository so that you can track its progress in real-time. You can also get involved in the [discussions](https://github.com/jsbayley/gazelle/discussions) where the design decisions for the future of Gazelle are conducted in the open.

## Roadmap

<p align="justify">
We are currently defining the roadmap. Please keep an eye on our <a href="https://github.com/jsbayley/gazelle">GitHub repository</a> for updates.
</p>

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

## Help and Support

<div align="justify">
<p>Stuck and need support? Here are a few suggested next steps:</p>
  <ul>
    <li>Review our <a href="./docs/DOCS.md">documentation</a> to see if we've answered your question.</li>
    <li>Check out <a href="https://github.com/jsbayley/gazelle/issues">Issues</a> to see if others are experiencing similar problems.</li>
    <li>Review <a href="https://github.com/jsbayley/gazelle/discussions">Discussions</a> to discover conversations our community is having.</li>
    <li>File a new <a href="https://github.com/jsbayley/gazelle/issues">Issue</a> and help us improve Gazelle.</li>
    <li>Reach out on <a href="https://www.linkedin.com/in/jsbayley">LinkedIn</a> and we'll do our best to help you.</li>
  </ul>
</div>

## Our Contributors

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
