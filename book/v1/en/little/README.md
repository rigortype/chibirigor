---
title: The Little chibirigor
description: The gentle introduction — build a working minimal checker (check + annotate) across Part 0 through 9.
sidebar:
  order: 1
---

# The Little chibirigor

The Little volume teaches the smallest Ruby type checker, `chibirigor`, **by building it**
step by step. Across Part 0 through Part 9 we build a minimal implementation where `check`
(type diagnostics) and `annotate` (show the inferred types) work. **The code runs to
completion within this volume**; the deeper dives — the formal theory — are handed to the
Seasoned volume.

> New here? Start at the shared [README](../README.md) (motivation, audience, setup). Terms
> are in the [glossary](../glossary.md); cross-volume references live in the
> [appendices](../appendix/). Each chapter is a three-voice piece — "① type theory ↔
> ② Ruby/RBS ↔ ③ Rigor's implementation" — and ends with exercises (the full reading of the
> three voices is in [Part 0](part0-introduction.md)).

## Chapters

| Part | Theme |
|---|---|
| [0](part0-introduction.md) | Introduction — an inference-driven type checker / accepting input / two functions |
| [1](part1-literals-and-arithmetic.md) | Literals and arithmetic (`Const` / `type_of` / `check` / `annotate`) |
| [2](part2-method-dispatch.md) | Method sends and dispatch (+ a note on constant folding) |
| [3](part3-scope-and-statements.md) | Local variables and an immutable Scope |
| [4](part4-union.md) | Union — when a type doesn't settle on one |
| [5](part5-narrowing.md) | Narrowing — splitting by case |
| [6](part6-hash-and-tuple.md) | Hash and array types (`HashShape` / `Tuple`) |
| [7](part7-accepts-and-trinary.md) | Acceptance checks and three-valued logic (`accepts` = `:yes` / `:no` / `:maybe`) |
| [8](part8-rbs-and-signatures.md) | RBS and type signatures (a table from RBS → synthesize return types from `def`) |
| [9](part9-gradual-philosophy.md) | The philosophy of gradual typing (finale) |

When you're done, continue to the [Seasoned chibirigor](../seasoned/README.md) — the **"read
it"** volume.
