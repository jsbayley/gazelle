// SPDX-License-Identifier: AGPL-3.0-or-later
// Gazelle: a fast, cross-platform engine for structural analysis & design.
// Copyright (C) 2024 James S. Bayley

namespace Gazelle.Concrete

type ConcreteError = InvalidAge of message: string

type ReinforcedConcreteError = ConcreteError of ConcreteError
