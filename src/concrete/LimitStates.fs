// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Concrete

type DesignSituation =
  | Persistent
  | Transient
  | Accidental

type LimitState =
  | ULS of DesignSituation
  | SLS
