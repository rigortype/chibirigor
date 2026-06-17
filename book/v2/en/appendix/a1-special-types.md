---
title: "Appendix a1 — Catalog of special types"
description: "`untyped` / `void` / `never`, `Top` / `Bot` — a reference catalog consolidating the cross-volume \"special types\" in one place."
sidebar:
  order: 21
---

# Appendix a1 — Catalog of special types

This appendix is a **reference catalog** gathering, in one place for cross-volume lookup, the
"special types" introduced here and there in the main volume — `untyped` (`Dynamic[Top]`), `void`,
`never` (`Bot`), and the two ends of the lattice `Top` / `Bot`. It assumes readers who came here
via a "for details see appendix a1" pointer from Little Part 1 (`untyped`'s first appearance),
Part 4 (Union and the bottom type), Part 5 (a branch going empty under narrowing), Part 8 (the
return type `void`), and Part 9 (the sum-up box of the three special types). **Look up only the
items you need, and return to the main volume.** Where each type is dug into deeper is signposted
in "Going further" at the end. The main line sticks to organizing concepts and produces no new code
(only the chapter-end notes occasionally attach a tiny-version demo).

---

## a1-1. `untyped` (= `Dynamic[Top]`) — the heart of gradual

`untyped` is the type representing "**unknown / no way to check — keep quiet.**" In the chibirigor
body it's the `Dynamic` type carrier, and `to_s` prints `untyped`. It's what `type_of` returns when
it loses the type (unknown syntax, unknown method), and **where `untyped` appears = where Rigor
lost the type.**

### Decompose into two axes

What's often confused about `untyped` (and TypeScript's `any`) is that two originally separate axes
look like one.

- **Axis A: position in the subtyping lattice** — the top of the type lattice (`Top` / ⊤) or the
  bottom.
- **Axis B: checked or not** — a sound static type, or gradual's "stay quiet" type.

Laid out on these two axes, the differences among similar faces become clear:

- TypeScript's `unknown` is a **plain top type** (axis A only). Anything can be assigned to it, but
  **you can do nothing with it until you narrow it.** A one-way street, "can be put on top but not
  taken out from below," and therefore sound.
- TypeScript's `any` is not just "anything can be assigned to it" but "it can be assigned to
  anything." It behaves as if it's **at the top and bottom of the lattice at once** — a
  contradiction as a subtyping relation, so `any` is **a switch that gives up soundness and turns
  checking off** (axis B). This is the true form of gradual typing's dynamic type `?`.
- `Dynamic[Top]` is the expression that **deliberately decomposes these two and shows them by
  name.** `Top` is the part (value set) of a sound top type "anything could be here," and `Dynamic`
  is the **gradual `?` marker** laid on top of it (the mark for "the type was lost. stay quiet,
  don't frighten"). Its behavior is close to `any`, but it isn't collapsed into one word — "where it
  stayed quiet" remains as structure. That Little Part 9's `rigor check --explain` can map every
  fail-soft site is because this `Dynamic` marker doesn't vanish.

> [!NOTE]
> The main volume's minimal `Dynamic` (`Dynamic.new`) and the internal notation `Dynamic[Top]` are
> **the same thing.** `[Top]` is internal notation making "the contents could be anything" explicit;
> the main text just doesn't attach it. In real Rigor's code, top/bottom are lowercase `top`/`bot`,
> but this book unifies them as uppercase `Top`/`Bot` for readability.

In a phrase — **`unknown` presses you with "you don't know, so narrow it," while `any` /
`Dynamic[Top]` say "you don't know, so stay quiet."** Same "don't know," but the attitude is exactly
opposite — take soundness, or take not-stopping the working code.

### Cross-language / cross-tool correspondence

`untyped` isn't a Ruby-only concept; any language with a type checker necessarily has a counterpart.
Only the name differs; the role is always "unknown / no way to check — keep quiet."

| Language / tool | Name | In a phrase |
|---|---|---|
| TypeScript | `any` / `unknown` | `any` turns all checking off; `unknown` requires narrowing |
| Python (mypy) | `Any` | flows in both argument and return directions |
| Go | `interface{}` / `any` | the empty set of an interface = anything fits |
| PHP (PHPStan) | `mixed` | a root type and also the mark of "unknown" |
| C# | `dynamic` | turns compile-time checking off |
| Elixir (Dialyzer) | `dynamic()` | the "universe" of set-theoretic types |
| Rigor / RBS | `untyped` (internal: `Dynamic[Top]`) | `Top` (the supremum of all types) with `Dynamic` laid on it |

> [!NOTE]
> The story of `untyped` becoming a special relation that "passes both ways" in the three-valued
> acceptance check (gradual consistency `~`, symmetric and non-transitive) is in Seasoned Part 2
> §2-5.

---

## a1-2. `void` — "a value is returned but don't look at it," akin to ⊤

`void` is a type exclusive to RBS's **return position**, meaning "**a value is returned, but don't
rely on (don't look at) that value.**" It's put on methods "called for a side effect," like
`Array#each`.

### Behavior akin to ⊤ (the top type)

In RBS's type system, `void`'s *lattice behavior* is nearly the same as the top type (⊤)
(top-like) — because "it accepts any value / you can extract nothing from that value." But `void` is
a **return-position-only marker**, and when it appears in a value position it's treated as folded
back to `top` (it's not perfectly identical to the top type). What differs is only the **nuance (the
message put into it)**:

- The top type / `unknown` says "there's a meaningful value here. narrow it before you use it."
- `void` says "there's a value here but it has no meaning. **don't use it at all.**"

From the same lattice position, it sends the reader the opposite signal — a fine example of the
type system being identical but the *intent* differing.

### The BC-break contract

A Ruby method's **last evaluated expression is its implicit return value**, so a method tagged
`void` does return *something* at run time, and rewriting the implementation can change that value.
`void` is the declaration that honestly admits this reality as a type, tantamount to saying "this
method **could return any value.**" That's exactly why it works on the **contract** side:

- Return type `void` makes the caller promise "don't depend on the return value" ⇒ **changing the
  return value later isn't a breaking change (BC break).**
- Conversely, declaring `-> nil` means promising "I return nil," and later changing it to return
  another value breaks the contract.

Herein lies the practical benefit of choosing `void`.

### `void` works on the checking side `⇐` (a bridge to Seasoned Part 1)

chibirigor is on the side that **synthesizes return types from the body** (it *builds* types, not
*verifies* annotations), so `void` doesn't appear — it always emits a concrete type, "the type of
the last expression." That is, `void` **doesn't appear on the synthesis side `⇒`** that assembles a
type; it **works on the checking side `⇐`** that checks an RBS-*declared* return type against the
body. The true form of that `⇐` (subsumption / the three-valued acceptance check) is treated in
Seasoned Part 1.

> [!NOTE]
> TypeScript's `void`, and C/Java's `void` (these lean toward "there is no value"), are family at
> the same position.

---

## a1-3. `never` (= `Bot`) — the type of an unreachable branch

If the top type is the lattice's ceiling "anything could be here," its **exact opposite** is the
**bottom type** (`Bot`, ⊥). "**The smallest box, into which not a single value goes**" — the type
representing *unreachable / can't happen*.

### Where it appears

- *Non-returning* expressions: a path that only `raise`s and returns no value; an infinite loop.
- *A branch narrowed dry*: the dead branch of Little Part 5. In the then-branch of applying
  `if x.is_a?(String)` to `x : Integer`, if `x` had a type it would be "Integer and String" — no
  such value **exists**, so the type is the empty set = bottom. You could also call it the type of
  "narrowed until zero candidates remain."

### The dual property — `Bot <: T`

Its property, too, is **dual** to top.

- A top-type value "could be anything, so **you can do nothing until you narrow it.**"
- A bottom-type value "**can't be obtained in the first place,** so if you assume it were obtained,
  **you can use it for anything**" — this is **`Bot <: T`: it's a subtype of every type.** "A
  function that returns `never`" = "a function that never returns" is this restatement.

### Cross-language / cross-tool correspondence

| Language / tool | Name | Where it appears |
|---|---|---|
| TypeScript | `never` | gaps in exhaustiveness checks; the return of `throw`/infinite loops; a branch narrowed dry |
| Rust | `!` (never type) | `panic!`, `return`, `loop {}` and other "non-returning" expressions |
| Scala / Kotlin | `Nothing` | an expression that only throws; the element type of an empty collection |
| Haskell | `Void` | a type with no value (zero inhabitants) |
| Rigor / RBS | `bot` (internal: `Bot`) | unreachable; a path that always stops at `raise` |

> [!NOTE]
> chibirigor itself doesn't build the bottom type *as a type*; it stops at treating "an
> unreachable branch" as a **diagnostic** (real Rigor's unreachable arm, ADR-47). Under a "don't
> frighten" policy, telling you "this won't be taken" is more useful in practice than dutifully
> propagating the empty set.

### a1-3x. A note: chibirigor has an unreachable-arm diagnostic too (opt-in)

In line with the policy above, we added **unreachable detection** to chibirigor too. Without
building a `Bot` type carrier, it directly judges "the branch's subject becomes the empty set (=
`Bot`)" and emits an `:info` diagnostic (`unreachable_branch?` in `lib/chibirigor/narrowing.rb`).
You opt in with `check --unreachable`:

```console
$ printf 'x = 1\nif x.is_a?(String)\n  y = x + 1\nelse\n  z = x * 2\nend\n' > demo.rb
$ ruby exe/chibirigor check --unreachable demo.rb
demo.rb:3:3: info: this branch is unreachable (the condition is always false)
    y = x + 1
    ^^^^^^^^^
```

The then-branch of applying `if x.is_a?(String)` to `x : Integer` is "Integer and String" = zero
inhabitants = `Bot`, so it's unreachable. **By default (no flag) it stays quiet** — so as not to
break Little Part 4/5's promise of "don't narrow, don't touch a dead branch (zero false
positives)." Telling you "this won't be taken" is reserved for **when the reader explicitly asks**
(`--unreachable`).

For soundness, it asserts **only when it can prove**:

- only when the subject is a **closed known type** (containing no `untyped` at all). If even a
  little `Dynamic` mixes in, it stays quiet (gradual).
- `is_a?` asserts "can't happen" only between **concrete classes (leaves) it can be sure are
  mutually disjoint.** An **ancestor relation** like `is_a?(Numeric)` or `is_a?(Object)` it doesn't
  assert (it holds no ancestor table) — because even with `x : Integer`, `x.is_a?(Numeric)` can be
  true (avoiding a false positive).

This is a miniature of real Rigor's ADR-47 (`flow.unreachable-clause`). The real thing judges a
clause where narrowing narrowed the subject to `bot` as dead, and has an FP envelope (a fence of
exclusion conditions to prevent false positives) excluding loops, blocks, and gradual. chibirigor
minimizes the same idea down to three points: "leaf-class disjointness," "closed types only," and
"opt-in."

> [!NOTE]
> Note it's **the reverse of exhaustiveness (missing arm)** (appendix [a5-5](a5-other-languages.md)).
> Java's/C#'s exhaustiveness checks blame a "**missing** branch"; an unreachable arm points to a
> "**superfluous (never taken)** branch." The former presses you to *write* it; the latter just tells
> you it *can be removed* — it doesn't stop working code.

---

## a1-4. `Top` / `Bot` — the two ends of the lattice

The subtyping `<:` lattice has two ends. **`Bot <: T <: Top`** — every type `T` necessarily fits
between `Bot` as the lower end and `Top` as the upper end.

```text
                 Top (⊤)           ← every type is a subtype of this (the biggest box)
              ╱    │    ╲
     {name:}   Integer   String     ← concrete types
              ╲    │    ╱
                 Bot (⊥)           ← a subtype of every type (unreachable; the smallest box)
```

> [!NOTE]
> The `{name:}` in the figure is not a maximal element on the same level — as in Seasoned §2-2,
> structural types **extend downward by width subtyping** (`{name:, age:} <: {name:}`), so it's
> placed here as "a representative of the concrete types" in a schematic.

The two ends are **dual**:

| | `Top` (⊤) | `Bot` (⊥) |
|---|---|---|
| Value set | anything fits (the biggest box) | nothing fits (the smallest box) |
| Subtyping | every type is a subtype of `Top` (`T <: Top`) | a subtype of every type (`Bot <: T`) |
| Using the value | can do nothing until you narrow it | can't be obtained in the first place = usable for anything |
| Face in the main volume | `unknown` / `void` (Little Part 1, 8) | `never` (Little Part 4, 5) |

Where subsumption is the operation of "loosening upward (toward `Top`)," `Bot`, from the lattice's
floor, "can impersonate any type" — the directions being symmetric is the lattice's beauty.

> [!NOTE]
> Note: `untyped` (`any` / `Dynamic[Top]`) is **not the top type.** As in a1-1's axis A / axis B,
> `Top` is the sound top type (axis A), and `untyped` is that with a "turn checking off" marker
> (axis B) laid on it — a different thing.

---

## a1-5. The three types compared, summed up (`untyped` / `void` / `never`)

Laying out the three beginners most often confuse makes the axes clear.

| Type | Control returns? | The value? | In a phrase | First appears |
|---|---|---|---|---|
| `untyped` (`unknown`) | returns | exists but **type unknown** | "the contents are unknown" | Little Part 1 |
| `void` | **returns** | exists but **don't look** | "don't rely on the return value" | Little Part 8 |
| `never` (`Bot`) | **doesn't return** | **none** (zero inhabitants) | "doesn't come back at all" | Little Part 4 |

- `void` is neither "the value's contents are unknown" (`untyped`) nor "doesn't come back"
  (`never`); it's **a 'be quiet' mark standing in the return position** — a relative of `untyped` in
  telling the checker "you needn't check the return value here" (but its target is not a *value* but
  *how the return value is used*).
- `untyped` and `void` are both family of "be quiet," but `untyped` **doesn't stand on its own as a
  top in the lattice** (a gradual type with a marker on `Top`), while `void` is **akin to top on the
  lattice** (but a return-position-only marker).
- Only `never` / `Bot` is at the **floor** of the lattice, opposite in direction from the other two
  (the ceiling side).

---

## a1-6. Going further (where each deeper dive lives)

This appendix is an organization for "looking up." The deeper dives into the theory live here:

- **`never` / `Bot`, the formal treatment of the lattice and duals** → Seasoned Part 2 "Subtyping
  and variance" (`Bot <: T <: Top`, S-Arrow and variance).
- **`void` / `Top`, checking a return type `⇐`** → Seasoned Part 1 "Bidirectional typing" (the map
  of synthesis `⇒` / checking `⇐`).
- **`untyped`'s gradual consistency `~` (symmetric, non-transitive)** → Seasoned Part 2 §2-5, and
  the sum-up of "unsound on purpose" is Seasoned Part 7.
- **refinement carriers (`non-empty-string`, `positive-int`, etc., types narrowed by a predicate)**
  → appendix [a2](a2-narrowing-patterns.md) "Narrowing patterns" / the glossary's "refinement
  carrier" and "`Difference` type" entries. The difference from `Const` (a pinpoint value) is there
  too.
- **The sum-up in the main volume** → Little Part 9's "three special types" sum-up box.
