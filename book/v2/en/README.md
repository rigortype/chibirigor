---
title: chibirigor — learn type checking by building a tiny Ruby type checker
description: A two-volume online book. The Little volume builds a working minimal checker; the Seasoned volume reads the machinery behind it.
sidebar:
  order: 0
---

# chibirigor

An online tutorial where you **learn by building the smallest Ruby type checker,
`chibirigor`, with your own hands.** Just as [chibivue](https://github.com/chibivue-land/chibivue)
teaches Vue by rebuilding it in miniature, we learn the real
[Rigor](https://github.com/rigortype/rigor) — a gradual type checker for Ruby — by rebuilding
it small.

> [!NOTE]
> This directory is the **English edition (v2)**, transcreated from the Japanese original
> under [`../ja/`](../ja/README.md). Contributor notes on how it is written live in
> [STYLE.md](STYLE.md).

## What you get from this book

By writing a few dozen to a few hundred lines of Ruby, you'll be able to answer these
questions with your own hands:

- How does a type checker **infer** types straight from source code?
- Why does Rigor **refuse to reject** untyped code, and still report only genuine
  contradictions?
- What does **"never frighten working code"** actually mean as a design stance?

When you're done, you'll have a small, working type checker with two commands — `check` (type
diagnostics) and `annotate` (show the inferred types) — and a head start on reading real
Rigor's source.

## Who this is for, and what's assumed

- **Intermediate Rubyists** — comfortable reading and writing classes, modules,
  `case`/`when`, and blocks.
- **No type-system theory required** (the Little volume). Every concept the Little volume
  leans on is introduced gently, right where it's needed.
- The only prerequisite is Ruby. There is **no math or proof in the Little volume** — the
  Seasoned volume takes those on.

## Two volumes

Following Scheme's classics (*The Little Schemer* / *The Seasoned Schemer*), the book is split
in two.

- **The Little chibirigor** ([`little/`](little/README.md)) — the gentle introduction. Build a
  minimal implementation where `check` and `annotate` work, across Part 0 through Part 9.
  **The code runs to completion within the Little volume**; the formal theory is handed to the
  Seasoned volume.
- **The Seasoned chibirigor** ([`seasoned/`](seasoned/README.md)) — the advanced volume. Here
  we **read the machinery behind what the Little volume built, in the language of formal
  theory** (bidirectional typing, subtyping and variance, generics, recursive types, real
  inference, the FactStore, soundness). It takes the notation and vocabulary head-on.

> [!NOTE]
> The Little volume is the **"build it"** volume; the Seasoned volume is the **"read it"**
> volume. In the Seasoned volume, concepts — not code — take the lead.

Either order works, but if this is your first time, read Little → Seasoned.

## The three perspectives in every chapter

Each chapter is written from a small set of **three perspectives** — "**① type theory ↔
② Ruby/RBS ↔ ③ Rigor's implementation**," laid over one another
([Part 0](little/part0-introduction.md) explains it in full).

Every chapter ends with **exercises** you run by hand. At the back of the book you'll find a
**glossary** ([`glossary.md`](glossary.md)) and a set of cross-volume **appendices**
([`appendix/`](appendix/)).

## Further reading (optional)

For readers who want to look one level deeper into type theory, each chapter carries a short
**reference note** in the margin. **Neither book below is required** — chibirigor stands on
its own; reach for them only where you want more.

- **TAPL** — Benjamin C. Pierce, *Types and Programming Languages* (MIT Press). The full
  textbook, with real proofs for each topic. This is the shared reference for English readers.
- **『しくみ』** (*The Mechanics of Type Systems*) — Yusuke Endoh, *Building a Type System in
  TypeScript* (Lambda Note). A gentle distillation of TAPL's essence that builds a *checker*
  for a typed mini-language — almost exactly chibirigor's mirror image. **It is published in
  Japanese only**; where it is the sole correspondence and no English equivalent exists, the
  text says so plainly.

A correspondence table lives in appendix
[`appendix/a4-bibliography.md`](appendix/a4-bibliography.md).

## Setup

Anything with Ruby 3.4+ (which bundles the **Prism** parser) will run it. No test framework
needed.

```console
$ git clone <this repository>
$ cd chibirigor
$ ruby exe/chibirigor check    path/to/file.rb   # type diagnostics
$ ruby exe/chibirigor annotate path/to/file.rb   # show inferred types
```

If you want to follow along by typing the code out, a `lib/` directory in your own working
folder, grown file by file per chapter, is all you need. Every chapter's code has been checked
to run on real Prism/Ruby.

## Table of contents

### The Little chibirigor (build it)

| Part | Theme |
|---|---|
| [0](little/part0-introduction.md) | Introduction — an inference-driven type checker |
| [1](little/part1-literals-and-arithmetic.md) | Literals and arithmetic |
| [2](little/part2-method-dispatch.md) | Method sends and dispatch |
| [3](little/part3-scope-and-statements.md) | Local variables and an immutable Scope |
| [4](little/part4-union.md) | Union — when a type doesn't settle on one |
| [5](little/part5-narrowing.md) | Narrowing — splitting by case |
| [6](little/part6-hash-and-tuple.md) | Hash and array types |
| [7](little/part7-accepts-and-trinary.md) | Acceptance checks and three-valued logic |
| [8](little/part8-rbs-and-signatures.md) | RBS and type signatures |
| [9](little/part9-gradual-philosophy.md) | The philosophy of gradual typing (finale) |

### The Seasoned chibirigor (read it)

| Part | Theme |
|---|---|
| [1](seasoned/part1-bidirectional-typing.md) | What bidirectional typing really is |
| [2](seasoned/part2-subtyping-and-variance.md) | Subtyping and variance |
| [3](seasoned/part3-generics-and-substitution.md) | Generics and type substitution |
| [4](seasoned/part4-recursive-types.md) | Recursive types — μ and coinduction |
| [5](seasoned/part5-real-inference.md) | Real type inference — filling in arguments |
| [6](seasoned/part6-fact-store.md) | The complete FactStore |
| [7](seasoned/part7-soundness.md) | Soundness, normalization, and "unsound on purpose" |
| [8](seasoned/part8-toward-rigor.md) | Toward real Rigor (finale) |

### Appendices

- [a1 — Catalog of special types](appendix/a1-special-types.md) — `untyped` / `void` /
  `never` / `Top` / `Bot`
- [a2 — Narrowing patterns](appendix/a2-narrowing-patterns.md) (a warm-up for Seasoned Part 6;
  skippable if you're reading only the Little volume)
- [a3 — Tooling](appendix/a3-tooling.md) — `--explain` / the two-layer type display / `trace`
  / the dispatch cascade
- [a4 — Reference & ADR correspondence table](appendix/a4-bibliography.md)
- [a5 — Bridges from other languages](appendix/a5-other-languages.md) — null safety /
  nominal vs. structural subtyping / the HashShape lineage / untagged unions

Now let's begin with Part 0.
