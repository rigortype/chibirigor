---
title: Appendix a4 — Reference & ADR correspondence table
description: A reference appendix consolidating the 『しくみ』 / TAPL correspondences scattered beside each chapter, plus the real Rigor ADR pointers, into one table. An optional signpost, not required reading.
sidebar:
  order: 24
---

# Appendix a4 — Reference & ADR correspondence table

> [!TIP]
> **This appendix is an "optional signpost." It is not required reading.** This book (both
> volumes) stands on its own without opening a single reference book. This is a **reference
> table** that gathers, into one place, the correspondence information scattered beside each
> chapter — "this topic corresponds to which chapter of 『しくみ』 / TAPL," "which ADR of Rigor
> resolves this tension in the text" — so you can look it up at a glance. Pick up only the parts
> you need.

---

## a4-1. Bibliography

### Reference books (two)

- **TAPL** — Benjamin C. Pierce, *Types and Programming Languages* (MIT Press). The full
  textbook, with proper proofs for each topic. **For English readers, the original is the
  reference.**
- **『しくみ』** (*The Mechanics of Type Systems*) — Yusuke Endoh, *Learning Types and
  Programming Languages by Building Them in TypeScript* (Lambda Note). A gentle distillation of
  TAPL's essence that builds a *checker* for an annotated mini-language in TypeScript. In
  presupposing annotations, it is exactly the mirror image of chibirigor, which builds on
  inference. **Published in Japanese only** — where it is the sole correspondence and no English
  equivalent exists, the text says so plainly.

### Different weight of signposts (the two volumes differ)

- **The Little chibirigor** — references are **optional.** Each chapter is complete with zero
  references. 『しくみ』 is the closest, as a *gentle mirror image*, with corresponding chapters
  noted in the margin (Japanese readers only).
- **The Seasoned chibirigor** — sets **TAPL as the primary signpost** (variance, recursive types,
  System F, soundness proceed along TAPL's chapter structure). 『しくみ』 runs alongside at key
  points, but the areas the Seasoned volume steps into (real type inference, the FactStore) have
  no corresponding chapter in it.

### Rigor's ADRs / spec

- **ADR** (Architecture Decision Record) — Rigor's design-decision records, in the repository's
  `docs/adr/`. Judgments like "why allow open classes," "why fork rather than Ractor" remain as a
  *narrative*.
- **Internal spec** — `inference-engine.md` and others. If an ADR is the "why," the spec writes
  the "how."

> [!NOTE]
> Correspondence chapters for the type system itself are in 『しくみ』 / TAPL; judgments of Rigor's
> own *engineering* are in the ADRs. Seasoned Part 8 gathers a doorway to the ADRs as the bridge
> from "minimal version → practical tool," and a4-4 draws on that table.

---

## a4-2. The Little volume — chapter × 『しくみ』 ch. × TAPL ch.

Laid out by the v1 chapter structure (Little Part 0–9). Both references are optional. The closer
primary is the 『しくみ』 side.

| v1 ch. | Theme | 『しくみ』 | TAPL | Relation (in a phrase) |
|---|---|---|---|---|
| 0 | Introduction (an inference-driven type checker) | ch. 1 | ch. 1, ch. 8 §8.3 | contrast (sound vs. false-positive-averse) |
| 1 | Literals and arithmetic | ch. 2 | ch. 8 | near match (precision exceeds via `Const` / `type_of` = `typecheck`) |
| 2 | Method sends and dispatch | ch. 3 | ch. 9 | reread (function → method, `{params, retType}`) |
| 3 | Local variables and an immutable Scope | ch. 3, 4 | ch. 9, 11 §11.5 | match (`tyEnv` → immutable `Scope`, let-bindings) |
| 4 | Union — a type doesn't settle on one | (no corresponding ch.) | ch. 11 §11.10 | own terrain (untagged union; both books have only tagged) |
| 5 | Narrowing — splitting by case | (no corresponding ch.) | (no corresponding ch.) | own terrain (gradual / flow-sensitive) |
| 6 | Hash and array types | ch. 5 | ch. 11 §11.8, §11.7 | match (but **reversed** for open vs. exact match) |
| 7 | Acceptance checks and three-valued logic | ch. 7 | ch. 15 | match + three-valued / consistency extension |
| 8 | RBS and type signatures | ch. 9, ch. 9 exercises, afterword | ch. 23, 22 | partial match (distant relative of type substitution → return-type synthesis; fills 『しくみ』's gap) |
| 9 | The philosophy of gradual typing | afterword | *beyond* ch. 8 §8.3 | connection (soundness in full → gradual beyond it) |

> [!NOTE]
> The areas 『しくみ』 / TAPL didn't treat directly (untagged Union, three-valued logic, gradual,
> flow-sensitive narrowing) are precisely where chibirigor "pushed past." That's why Little Part 4
> and 5 have no corresponding chapter.

---

## a4-3. The Seasoned volume — chapter × 『しくみ』 ch. × TAPL ch.

Laid out by the v1 **new order** (Seasoned Part 1–8). The Seasoned volume sets TAPL as the primary
signpost.

| v1 ch. | Theme | TAPL | 『しくみ』 | Relation (in a phrase) |
|---|---|---|---|---|
| 1 | What bidirectional typing really is | ch. 9 | ch. 3 | foundation (typing rules of simply-typed lambda; `⇒`/`⇐`) |
| 2 | Subtyping and variance | ch. 15, 16 | ch. 7 | match (width/depth, variance, algorithmic subtyping) |
| 3 | Generics and type substitution | ch. 23, 22, 26, 29 | ch. 9 | match (System F type application, α-conversion, bounded quantification) |
| 4 | Recursive types: μ and coinduction | ch. 20, 21 (HKT in ch. 29) | ch. 8 | match (equirecursive, coinduction = greatest fixed point; the HKT aside has separate grounds) |
| 5 | Real type inference: filling in arguments | ch. 22 | ch. 9 exercises, afterword | frontier (type reconstruction, constraints, unification; where 『しくみ』 gave up the answer) |
| 6 | The complete FactStore | (no corresponding ch.) | (no corresponding ch.) | own terrain (general dataflow analysis; no chapter in a type-theory textbook) |
| 7 | Soundness, normalization, unsound on purpose | ch. 8 §8.3, ch. 12 | afterword | sets the central theorem (progress + preservation / normalization), tells *why unsound* |
| 8 | Toward real Rigor | (no corresponding ch.) | (no corresponding ch.) | bridge (learning minimal version → practical tool; the correspondence is the ADRs and spec) |

> [!NOTE]
> Seasoned Part 6 (FactStore) and Part 8 (the bridge to Rigor) are **own terrain** with no
> corresponding chapter in a type-theory textbook. This is the engineering layer where the
> Seasoned volume steps beyond TAPL.

---

## a4-4. Quick reference to the main ADRs the text cites

Seasoned Part 8 gathers a doorway to the ADRs. This is a list drawn from that table. Just a number
and a phrase (for detailed context see Seasoned Part 8 §8-2 / §8-3; the chapter numbers in the
text use the v1 structure).

### Two to grasp the big picture first

| ADR | In a phrase |
|---|---|
| ADR-0 | The starting point of the foundations and design principles (a map for all later ADRs) |
| ADR-4 | How the type inference engine works |

### ADRs that correspond to the text's tensions (Seasoned Part 8 §8-3)

| ADR | In a phrase | Where in the text |
|---|---|---|
| ADR-5 | The robustness principle (how to avoid false positives) | Little Part 4, 7; Seasoned Part 7 (unsound on purpose) |
| ADR-22 | baseline + onboarding | Little Part 9 (baseline proper) |
| ADR-20 | Lightweight HKT (stop recursion with reduction fuel; implemented) | Seasoned Part 4 (fuel), Part 7 (budget) |
| ADR-41 | Design of an inference budget (Status: Proposed, not implemented) | Seasoned Part 7 (budget) |
| ADR-14 | sig-gen (signature generation) | Little Part 8, Seasoned Part 3 (erasure) |
| ADR-25 | plugin RBS | Little Part 8 |
| ADR-32 | inline RBS | Little Part 8 |
| ADR-46 | incremental analysis (a cross-file dependency graph) | Seasoned Part 8 §8-2 (engineering) |

### Engineering ADRs the practical tool adds

The more *engineering-leaning* ADRs — extension API, interface separation, macro expansion,
`Data`/`Struct` folding, persistent cache, bundled LSP, CI output formats, concurrent analysis,
performance gates, and so on — are listed in **Seasoned Part 8 §8-2** (the Little volume's text
doesn't name them).

> [!NOTE]
> The fastest reading order is to **open one ADR for a row that interests you** — each is written
> as a narrative of "why it was designed this way." The type inference engine's internal spec is
> `inference-engine.md`.

---

## a4-5. How to use this appendix

- **Want to peek one level deeper into type theory** → look up the corresponding chapter in
  a4-2 / a4-3 and open your copy of 『しくみ』 / TAPL. The book is complete even if you have
  neither.
- **Want to read real Rigor** → from a4-4, open one ADR for the row that interests you most.
  ADR-0 → ADR-4 is the map of the whole.
- **Back to the text** → if you came here from a one-line pointer at the end of a chapter, check
  your position in the table and return to the chapter. The quick-reference table is a signpost,
  not the main line.
