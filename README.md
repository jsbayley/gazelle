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

- [Documentation](#documentation)
- [Get Involved](#get-involved)
- [Open Source](#open-source)
- [Security](#security)
- [Errata](#errata)

## Documentation

Visit our [docs](./DOCS.md) to learn more.

## Why

The foundational aspiration for Gazelle is to be a central library (and CLI) for Structural Engineering design, which captures the design equations, and other useful utilities that can be assembled into workflows for various engineering-related tasks. The reason I originally selected F# was because of the Units-of-Measure feature and its interop with existing engineering applications. 

The ETABS wrapper was originally a separate project that I started as a concept, and I decided to integrate into Gazelle because it felt like a logical thing to do: pull/push data from where Engineers are working and then run automation on top of those steps. 

I got caught up in a lot of analysis paralysis for a while, hence why the project paused for quite some time. So, I've decided to take a practical approach of getting it out there, seeing what the reaction is from people, and whether or not it's potentially valuable.

The CLI piece is for two reasons: 

1. The input/output mechanics of compiled tools mean that they can interoperate easily with other languages and stacks, and 

2. Because I'd like to eventually layer on top either a Daemon-style service, or an AI-orchestration. But relying on the strictness of the F# compiler to help ensure structural integrity for users.

## Values

Gazelle is opinionated by design. These **nine** principles guide every decision in the project and set clear expectations for engineers and contributors.

1. **Privacy-First:** All computation happens locally. Your data stays on your machine.
2. **Radical Transparency:** Algorithms must be readable, verifiable, and open to scrutiny.
3. **Open Collaboration:** Open code, discussion, and shared learning strengthen our profession.
4. **Reliability Matters:** Stability, correctness, and deterministic behaviour matter more than novelty.
5. **Type-Safe by Design:** F# Units-of-Measure and strong typing ensure mistakes surface at compile time.
6. **Designed for Humans:** Simplicity, clarity, and flow are core design goals to reduce cognitive load.
7. **Optimised for AI:** Clean CLI interfaces and structured data formats ideal for AI orchestration.
8. **Performance as a Feature:** Lean, fast, low-friction engineering binaries.
9. **Small Composable Tools:** Unix-style components combine into workflows.

## Get Involved

See our guidance on [how to get involved](./CONTRIBUTING.md).

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

---

<div align="center">
  <p><strong>Built with ❤️ for the global engineering community</strong></p>
  <p><small>Fast • Simple • Reliable • Transparent • Cross-platform</small></p>
</div>
