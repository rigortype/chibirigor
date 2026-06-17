---
title: Part 0 — Introduction: an inference-driven type checker
description: The doorway to the book. Grasp what chibirigor is and isn't, share the implementation's premises, and the two promises that run through the whole book.
sidebar:
  order: 1
---

# The Little chibirigor Part 0 — Introduction: an inference-driven type checker

> The doorway to the book. Before we write any code, this chapter is about grasping what
> chibirigor is — and isn't. We'll share the implementation's premises (Prism, `check`,
> `annotate`) and the two promises that run through the whole book up front.

`chibirigor` is **the smallest Ruby type checker — one that infers types for itself.** Step by
step, with our own hands, we'll build a miniature version of the architecture of the real
[Rigor](https://github.com/rigortype/rigor), a gradual type checker for Ruby. Just as
[chibivue](https://github.com/chibivue-land/chibivue) teaches Vue by rebuilding it small, we
learn Rigor by rebuilding it small. This book is built the same way chibivue is — chibivue
leaves each round's milestone under `book/impls`, and from the end of each chapter you can jump
to the staged snapshot at `impls/dist/partN` (the working minimal version, built up to that
chapter).

> [!NOTE]
> **Where Rigor came from — why it was built, and why we rebuild it "small"**
>
> Rigor's author spent roughly a decade on static analysis for dynamic languages, PHP among
> them. The tooling around Ruby's types had long felt awkward to them, and for a long time they
> took that to be Ruby's fate as a language — until RubyKaigi 2026 (Hakodate), where several
> talks overturned that assumption. Feeling the need to step well beyond the frame of Ruby's
> existing type tools, they took features from TypeScript and Python together with PHPStan's
> plugin architecture and **re-integrated them on top of RBS's type system** — and Rigor was
> born.
>
> Because Rigor came together so quickly, though, almost no one holds its whole picture in
> their head. Type inference, that seemingly magical mechanism, ought to be something you can
> rebuild by hand once you've extracted just its essence — exactly as chibivue did with the
> reactivity system. chibirigor, this book, began from that hunch.

What we build, at the center of the book, is just two commands:

- **`check`** — read Ruby code and report type contradictions (diagnostics).
- **`annotate`** — display the inferred types.

(Later, an appendix adds a small extension — opt-in `--explain` / `--unreachable` flags to
`check` — but the spine of the book stays these two throughout.)

---

## 0.1 An inference-driven type checker

First, let's pin down what kind of tool chibirigor is.

Most **type checkers** out in the world assume *type annotations written into the program* and
decide whether those annotations contradict one another. Without annotations, they have no
work to do.

- A **type annotation** means *writing a type into the code*, as in "this variable is an
  `Integer`." In Ruby you don't normally write them — which is why the first time you write one
  yourself is much later, in Part 8 (RBS); until then they don't appear.

But Ruby code usually has no type annotations. So before it leans on annotations, chibirigor
first **derives types from the expressions themselves**. See `1`, it's an integer; see
`"a".upcase`, it's a string. Using the types it gets that way, it finds contradictions
(`check`) and shows types (`annotate`). In one phrase, chibirigor is **"an annotation-free type
checker, built on a foundation of inference."** Inference isn't a separate pre-pass set apart
from checking — it's the **foundation** that makes checking possible.

The "inference" we mean here is **building a type upward from an expression (synthesis)** — "`1`
is an integer," "`"a".upcase` is a string" — stacking up types from what's written. The
reverse — **working out a method's parameter types backward from how callers use it** — we do
*not* do (in the Little volume). If we can't tell a parameter's type, we collapse it to
`untyped` (just remember: when in doubt, fall back to `untyped`). The catalog of special types,
`untyped` first among them, is appendix [a1](../appendix/a1-special-types.md) — no need to read
it through; it's a reference to consult after you've read up to Part 9.

> [!NOTE]
> **Ruby's other inference tool — TypeProf.** Rigor isn't the only tool that pulls types out of
> annotation-free Ruby. [TypeProf](https://github.com/ruby/typeprof), bundled with Ruby,
> "executes the whole program at the type level" and **works backward from callers all the way
> to parameter types**, generating RBS — exactly the job chibirigor gives up on by collapsing
> to `untyped`.
>
> That chibirigor (and Rigor) deliberately *don't* do this is not a weakness but a design
> choice: whole-program execution tends to explode on large codebases, so we **narrow to local
> inference and buy scale and "silence" (no false positives)** instead. This one line we won't
> cross — not inferring arguments backward — the Little volume returns to concretely just once,
> in Part 8, and the Seasoned volume treats carefully in Part 5.

> [!TIP]
> **About the reference books (optional).** For readers who want to look one level deeper into
> type theory, each chapter carries a reference note in the margin with the corresponding
> passage. **Neither is required** (chibirigor stands on its own). Pick up only the parts you
> need.
>
> - **『しくみ』** — Yusuke Endoh, *The Mechanics of Type Systems: Learning Types and
>   Programming Languages by Building Them in TypeScript* (Lambda Note; hereafter 『しくみ』). A
>   gentle distillation of TAPL's essence that builds a *checker* for an annotated mini-language
>   — chibirigor's exact mirror image. **Published in Japanese only.**
> - **TAPL** — Benjamin C. Pierce, *Types and Programming Languages* (MIT Press). The full
>   textbook, with proper proofs for each topic. **For English readers the original is the
>   reference** (『しくみ』 is Japanese-only).
>
> This Part 0's topics (type safety, undefined behavior) correspond to **TAPL ch. 1
> "Introduction" + ch. 8 §8.3 "Safety = Progress + Preservation" / 『しくみ』 ch. 1.**

"Infer first, then check on the result." **Inference is the foundation; `check` and `annotate`
consume its output.** Fix that order in your head from the start.

---

## 0.2 chibirigor doesn't reject its input

The other promise. **chibirigor accepts any code Ruby doesn't reject as a syntax error.** It
never says "this has no types, so I won't analyze it."

- The parser is Ruby's standard **Prism**. Prism partially parses even somewhat broken syntax,
  so *the range it accepts is, if anything, wider than the Ruby runtime's*.
- But **"chibirigor passed it" does not guarantee "it runs."** All we return is inferred types
  and diagnostics.

This is a decisive difference in stance from 『しくみ』. That book's checker *throws an exception
and stops* on a type error. chibirigor **always takes the input, and its output is diagnostics
and inferred types.** It doesn't stop; where it can't tell, it stays quiet and moves on.

---

## 0.3 The heart is two functions

The heart of the type checker we're about to build is just two functions. We grow these two
across the whole book.

- **`type_of`** — **find** (synthesize) a type from an expression. `1 + 2` → `Integer`, `"a"` →
  `"a"`. If it can't tell, it just returns `untyped` — it **never fails.**
- **`accepts`** — given a type where another is expected, **check whether it fits.**

The overall flow looks like this (`type_of` is the lead; `check` / `annotate` only consume its
output):

```text
                                  ┌─ accepts at an expected type → on mismatch ─→  check (diagnostics)
source ─Prism→ AST ─type_of→ type ┤
                                  └─ show it as is ────────────────────────────→  annotate (inferred types)
```

![Figure 0-1 — chibirigor's data flow](../figures/svg/little-0-1.svg)
> ▼ Figure 0-1 — chibirigor's data flow

`type_of` builds a type, `check` verifies "does it fit the expected type" and emits
diagnostics, and `annotate` shows the built type as is — **inference is the foundation, and
both `check` and `annotate` consume its output.**

And now, before anything else, let's put the core principle that runs through the whole book
into words — no jargon needed:

> [!IMPORTANT]
> A diagnostic (a type error) only ever appears where an expected type is fixed. And once
> "don't know" (`untyped`) is involved, it never complains. So — code whose type is unknown
> but that nonetheless works is never blamed.

This is the true form of Rigor's motto **"never frighten working code."** chibirigor, too,
cares far more about **not producing false positives** (not painting working code red) than
about soundness (catching every last bug). Why choose that side — that settles in chapter by
chapter, as we run up against the reality of Ruby.

---

## 0.4 The three perspectives in every chapter

Each chapter is written from a small set of three perspectives:

1. **① Type theory** — one concept you meet in the chapter (and where 『しくみ』 covers it).
2. **② In Ruby / RBS** — how it looks in Ruby, or *how it fails to show up*.
3. **③ Rigor's implementation problem** — why the naive implementation breaks against Ruby's
   reality, and how it was reconciled.

**"Understanding Rigor" means watching the trouble in ③ arise *necessarily* from ② (Ruby's
reality), and settle *gently* under the concept in ①.** The hard material (the formalization of
bidirectional typing, variance, recursive types, real type inference…) all goes to the
Seasoned volume, **The Seasoned chibirigor.** The Little volume concentrates on building a
*working* minimal version, and finishing it with satisfaction.

### What we stack across these nine chapters (the whole picture)

Each chapter builds on the one before. What grows in a chapter is just "one hard thing":

| Ch. | What we add | Keywords |
|---|---|---|
| 1 | Represent types as data; find types from expressions | `Const` · `type_of` · `check` · `annotate` |
| 2 | Type method sends | dispatch table · unknown is `Dynamic` |
| 3 | Handle variables and statements | immutable `Scope` · threading statements |
| 4 | Types branch (Union) | `Union` |
| 5 | Narrow types by case | narrowing · two laws |
| 6 | Give hashes and arrays types | `HashShape` · `Tuple` |
| 7 | Judge "does it fit" with three values | `accepts` · `:yes` / `:no` / `:maybe` |
| 8 | Pull types from RBS; infer and show return types | `Rbs.load` · synthesizing a `def`'s sig |
| 9 | Close with the philosophy of gradual typing | `untyped` propagation · baseline |

By the time you reach Part 9, you're left with a type checker where `check` and `annotate` both
work, end to end.

> [!TIP]
> **The 『しくみ』 / TAPL correspondence table** is gathered in appendix
> [a4](../appendix/a4-bibliography.md) (optional). Each chapter also carries the matching
> reference passage at its head, but when you want the whole map at a glance, consult appendix
> [a4](../appendix/a4-bibliography.md). The areas 『しくみ』 didn't cover directly (union types,
> three-valued logic, gradual typing, flow-sensitive scope) are precisely where chibirigor
> "pushes past" it.

---

## 0.5 Setup

Anything with Ruby 3.4+ (Prism bundled) will run it. No test framework, either.

```console
$ ruby exe/chibirigor check  path/to/file.rb     # type diagnostics
$ ruby exe/chibirigor annotate path/to/file.rb   # show inferred types
```

> [!TIP]
> **On checking the exercise answers.** Each chapter ends with exercises. The "verify with
> `annotate` / `check`" kind you can check on the spot by running the commands above. Each
> chapter's "**this chapter's implementation (and answer key for the exercises)**" link
> (`impls/dist/partN`) is *the reference implementation up to that chapter*, so you can line up
> your own hand-typed version against its behavior.
>
> The "explain / state your approach" kind has no single fixed answer; the grounds are in that
> chapter's prose (the point is to put it into words by hand).

Let's start by `parse`-ing and peeking at the syntax tree (the AST). Once you can see what kind
of tree Prism turns a program into, you're ready for `type_of` to walk that tree and build up a
type.

**Next chapter (Part 1):** we write the first `type_of`. A type checker of just a few dozen
lines — literals and arithmetic only — already starts doing the job of "giving things types."

---

> [!IMPORTANT]
> **Where this chapter sits.** Part 0 introduces concepts, so it has no corresponding
> implementation file (`lib/` code begins in Part 1). The two things promised here — "build on
> a foundation of inference (but don't work arguments backward)" and "don't reject / don't
> frighten" — become the bedrock of every design decision that follows.
