---
title: The Seasoned chibirigor
description: The advanced volume — read the machinery behind the Little volume in the language of formal theory. Part 1 through 8.
sidebar:
  order: 2
---

# The Seasoned chibirigor

Once the [Little chibirigor](../little/README.md) has built a *working* minimal checker
(`check` + `annotate`), this volume **reads the machinery behind it in the language of formal
theory**. Everything the Little volume **deliberately avoided** — the formalization of
bidirectional typing, variance, generics, recursive types, real type inference, the FactStore,
soundness — is collected here. Concepts lead, not code: each chapter is a *commentary plus
design sketches* anchored to the Little volume's implementation (the runnable sketches live in
[`examples/`](examples/README.md), where `ruby <file>` turns its own self-check green).

> **The Seasoned volume is the "read it" volume, not the "build it" one.** The Little volume
> was "very gentle, jargon deferred"; the Seasoned volume takes *vocabulary and formality*
> head-on. The reference correspondence (『しくみ』 / TAPL) is consolidated in
> [appendix a4](../appendix/a4-bibliography.md) (**neither is required reading**).

## Chapters (one long climb: structure → inference & flow → the peak of theory → the bridge)

| Part | Theme | Starting point (Little) |
|---|---|---|
| [1](part1-bidirectional-typing.md) | What bidirectional typing really is (`type_of` = synthesis `⇒` / `accepts` = checking `⇐`) | Little P7 · P9 |
| [2](part2-subtyping-and-variance.md) | Subtyping and variance (width/depth · covariant returns · contravariant parameters) | Little P6 · P7 |
| [3](part3-generics-and-substitution.md) | Generics and type substitution (`subst` · α-equivalence · variable capture · erasure) | Little P8 |
| [4](part4-recursive-types.md) | Recursive types — μ and coinduction (+ a note: HKT / `App` + fuel) | Little P6 |
| [5](part5-real-inference.md) | Real type inference — filling in arguments (capability gathering / constraints + unification) | Little P8 |
| [6](part6-fact-store.md) | The complete FactStore (six buckets · stability · closure capture · join) | Little P3 · P5 |
| [7](part7-soundness.md) | Soundness, normalization, and "unsound on purpose" (+ the two disciplines of gradual typing) | Little P9 |
| [8](part8-toward-rigor.md) | Toward real Rigor (plugins · cache · LSP · ADRs · performance) | the whole Little volume |

## Runnable design sketches

The core algorithms run on their own as minimal Ruby under [`examples/`](examples/README.md):
`subtype.rb` (Part 2) / `subst.rb` (Part 3) / `mu_typeeq.rb` (Part 4) / `unification.rb`
(Part 5) / `fact_invalidation.rb` (Part 6). `check_docs.rb` detects drift between the prose
and the code.

## Reading order

Come here after finishing the Little chibirigor. Each Seasoned chapter proceeds as "that
implementation back in the Little volume was really the theory of ◯◯" and "the ◯◯ the Little
volume avoided, read here."
