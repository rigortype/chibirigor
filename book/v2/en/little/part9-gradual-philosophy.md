---
title: Part 9 — The philosophy of gradual typing (finale)
description: "Finish off `untyped`'s propagation, set up a baseline, and sum up the four places chibirigor deliberately misses and the philosophy of gradual typing."
sidebar:
  order: 10
---

# The Little chibirigor Part 9 — The philosophy of gradual typing (finale)

This chapter's goal: **finish off `untyped`'s propagation, set up a baseline, and sum up "the
places chibirigor deliberately misses."** Then we connect to the large current of gradual typing,
and onward to Rigor. The main volume closes here.

---

## 9-1. `untyped` is contagious

One last small code change. When `untyped` (`Dynamic`) mixes into a union, we make **the whole
`untyped`**:

```ruby
module Type
  module_function

  def union(types)
    flat = types.flat_map { |t| t.is_a?(Union) ? t.members : [t] }.uniq
    return Dynamic.new if flat.empty?
    return Dynamic.new if flat.any?(Dynamic)   # ★ if untyped mixes in, the whole is untyped
    return flat.first if flat.size == 1
    Union[flat.freeze]
  end
end
```

```text
c ? 1 : foo.bar  ->  untyped
c ? 1 : "a"      ->  1 | "a"
```

The reasoning is straightforward. If we've lost track of even part of the type, we can't assert
anything about that union type. Rather than holding it half-baked as `1 | untyped`, we say
`untyped` honestly. This is gradual's way of thinking.

---

## 9-2. The four places chibirigor "deliberately misses"

chibirigor is not sound. It **deliberately** misses things. That's for "never frighten working
code." Let's make clear, at the end, where it misses:

1. **`untyped` accepts anything** (Part 7's `accepts` is `:maybe`). The hole at the boundary where
   the type was lost.
2. **A hash's unknown key is `nil`** (Part 6). Extra keys allowed too (open). Reject by exact match,
   as 『しくみ』 does, and working options hashes go bright red.
3. **`:maybe` is not punished** (Part 7's `dispatch`). When in doubt, stay quiet.
4. **Narrowing is conservative** (Part 5). A condition we can't read, disjoint, or `Dynamic` — we
   don't narrow.

Each trades "a miss = the danger of overlooking a bug" to avoid "a false positive = frightening
working code." chibirigor (and Rigor) weigh the latter cost far more heavily.

> [!NOTE]
> These four, raised here as a slogan, are re-stated in formal language in the Seasoned volume,
> **The Seasoned chibirigor** Part 7, as **holes that *deliberately* break soundness (progress +
> preservation).** "Why it's okay to be unsound" gets an answer from theory.

The three perspectives:

- **① Type theory:** the trade-off of soundness (ruling out undefined behavior) vs. fewer false
  positives.
- **② In Ruby:** types bolted onto a dynamic language. Too strict and on-the-ground code stops
  turning over.
- **③ In Rigor:** make "never frighten working code" the highest value. Deliberately miss.

---

## 9-3. Baseline — swallow the existing, watch only the new

Even so, running it on an existing codebase for the first time can produce a flood of
diagnostics. So, **baseline**: record the first diagnostics as "swallowed," and from then on show
**only new diagnostics**.

```ruby
def check(source, baseline = [])
  # … gather diagnostics …
  diagnostics.reject { |d| baseline.include?(d) }
end
```

```ruby
base = check(source)              # save the first diagnostics as the baseline
check(edited_source, base)        # only the new ones not in the baseline come out
```

"Leave the existing complaints alone for now; keep only the code you write from here clean" — this
too is a form of "don't frighten" (a miniature of Rigor's `.rigor-baseline.yml`). Not forcing you
to fix everything at once on the first introduction — another mechanism for **not frightening
existing code.**

### What to match on — don't include the column

A baseline judges "is this the same diagnostic" and subtracts the known ones. Here, **what to use
as the key** is the design's crux. chibirigor's baseline matches on **only the line and message**,
and **doesn't include the column.**

The reason is simple — so that **editing the same line and shifting the column doesn't unseat the
baseline.** We gave a diagnostic's display (Part 1's caret `^^^`) the column and length, but
don't bring them into the baseline's *identity check* — display precision and match stability are
separate things.

Real Rigor's baseline (**ADR-22**) makes this sturdier still. By default it matches on the **rule
ID** (which rule was tripped), and **doesn't include the line number** in the key. The design: a
baseline that's hard to unseat even when lines move up or down. Not including the column is the
same for both chibirigor and Rigor. To sum up — chibirigor matches on "line + message," Rigor on
"rule ID (no line)," and **neither looks at the column.** The coarser the key, the more
edit-resistant the baseline.

> [!NOTE]
> Real Rigor has an investigation command, `rigor check --explain`, that lists "the places it
> quietly missed (fail-soft)." A tool that produces a map of "why isn't this reported = where the
> type was lost," it makes visible the silence that chibirigor's "stay quiet if you don't know,"
> flipped over, turns into "quietly miss." The mechanism is in appendix
> [a3-1](../appendix/a3-tooling.md).

---

## 9-4. Summing up the three special types

Throughout the main volume, **"special types"** of a different stripe from ordinary types
(`Integer`, `String`, `Const[1]`, …) showed their faces a few times. Let's bundle them here at the
end. Each is "a mark that tells the checker *some attitude*," not a representation of "a value's
shape" like an ordinary type (a table comparing the three side by side is in appendix
[a1-5](../appendix/a1-special-types.md)).

And beneath these three lie the two ends of a **lattice** that orders types by size — **the
largest (holds anything) `Top` (⊤), and the smallest (zero inhabitants, unreachable) `Bot`
(⊥)**. `Top` is close to Java's `Object`, a "ceiling," but how it works in checking is a different
thing. Here it's enough to grasp just the symbols and their positions; the lattice itself in full
is handed to the Seasoned volume. `untyped` was `Top` with a "be quiet" marker (`Dynamic`) laid
on it; `never` was `Bot` itself; `void`, as *lattice behavior*, was next to `Top` — that was the
relationship.

- **`untyped`:** the escape hatch when the type was **lost.** Accepts anything, can be passed to
  anything (the "stay quiet" type that gave up soundness). The hardest-working special type in the
  main volume; 9-1's "contagion" is this too.
- **`void`:** a "be quiet" that stands in the return-type position. "A value is returned, but don't
  rely on it." Unlike `-> nil`, it has the **contract** benefit that changing the return value
  later isn't a breaking change.
- **`never` / `Bot`:** the zero-inhabitant type. It represents "this branch is **not reached**" —
  the dead branch of narrowing (Part 5), or the type of an expression that never returns.

> [!NOTE]
> The deeper dives are placed elsewhere. The catalog of the three (axis A: position in subtyping /
> axis B: checked or not, plus an `any` / `unknown` cross-language table) is in **appendix a1**.
> `never` / `Bot` (the bottom type and the subtyping lattice) in full is in **Seasoned Part 2
> (subtyping and variance)**; `void` / `Top` (how it works on the `⇐` side of bidirectional) in
> **Seasoned Part 1 (bidirectional typing)**. In the main volume we go just as far as bundling
> "there are three special types."

---

## 9-5. From here on — to gradual typing and Rigor

chibirigor was built, consistently from the start, on this stance:

| | A static, sound checker | chibirigor (gradual) |
|---|---|---|
| Verdict | Two-valued: fits / doesn't | Three-valued: `:yes` / `:no` / `:maybe` |
| Unknown type | None (annotations required) | `untyped` (`Dynamic`) is the lead |
| On a misfit | Stops with an exception | Pile up diagnostics and continue; stay quiet about the unknown |
| Values | Soundness | Produce no false positives (don't frighten) |

"Don't stop even when the type is lost," "stay quiet where you don't know," "never frighten working
code" — this gradual typing is a design philosophy formalized in 2000s type-system research.
chibirigor is a tracing of its doorway, *by your own hand*.

> [!TIP]
> **Reference note (optional).** If you have 『しくみ』 or TAPL — those books build a static, sound
> checker and end by pointing to gradual typing as "the next frontier." chibirigor can be read as
> starting from just past there. Reading them alongside makes "why three values" and "why fall back
> to `untyped`" settle in more easily (it stands on its own without them). Soundness in full is
> TAPL ch. 8 §8.3 "Safety = Progress + Preservation"; gradual is beyond it (post-TAPL research).

And now that we've finished building chibirigor, peeking into real Rigor, you can see that each
thing we met here (`Scope`, `accepts`, narrowing, RBS, sig inference) is built out at practical
scale. **chibirigor is a doorway to Rigor.**

---

## 9-6. Summary so far (the whole nine-chapter main volume)

| Part | What we added |
|---|---|
| 1 | `Const` / `Dynamic` / `Nominal`, `type_of`, `check` / `annotate` |
| 2 | `Dispatch` (the method table), unknown degrades, constant folding (note) |
| 3 | immutable `Scope`, `eval_statement` (thread statements) |
| 4 | `Union`, gather the branch types of an if / ternary into one |
| 5 | `narrow`, `nil?` / `is_a?` narrowing, the two laws, reassignment reset (don't narrow a dead branch) |
| 6 | `HashShape` / `Tuple`, index read (unknown key is nil · open) |
| 7 | `accepts` (three-valued), report only `:no` |
| 8 | `Rbs.load`, return-type synthesis for `def`, RBS-style signatures, making `untyped` visible |
| 9 | `untyped`'s propagation, baseline, the sum-up of deliberate misses, the three special types |

## What you can do, having read the main volume

With just a few hundred lines of Ruby, you got this far:

- **infer types** from Ruby source (literals, methods, variables, branches, hashes/arrays),
- report contradictions as **diagnostics** (without stopping, and staying quiet where you don't
  know),
- show the inferred types and **method signatures** with `annotate`,
- swallow existing code with a **baseline** and keep "only the code you write from here" clean,
- and implement "**never frighten working code**" as concrete mechanisms — `untyped`, three values,
  open shape, conservative narrowing, baseline.

This is a minimal version of real Rigor. In the Seasoned volume, we give each of these a
**theoretical backing.**

## Exercises

1. Confirm that `c ? 1 : foo.bar` becomes `untyped`, and point to which line of `union` did the
   work.
2. Pass a baseline to `check`, and confirm the existing diagnostics disappear and only the new ones
   come out.
3. Why does the baseline match on "line + message" and not include the column? If you add trailing
   whitespace to the same line and shift the column, what happens to the baseline? Explain from
   9-3's match key.
4. Of the "four deliberate misses," take one (e.g. unknown key → nil) and make it deliberately
   strict (an error); give one concrete example of what *working code* would be frightened.

---

If you find yourself wanting to watch the assembled inferencer *run* frame by frame, peek at
`trace` in appendix [a3](../appendix/a3-tooling.md) — you can follow with your eyes how the parts
built in each Part interlock on a single expression.

**On to the sequel, "The Seasoned chibirigor":** the formalization of bidirectional typing,
variance, recursive types, argument inference (type reconstruction), the complete FactStore,
soundness theory, the `erasure` / sig-gen proper — beyond *The Mechanics of Type Systems*, and into
Rigor's build-out. What you built *by hand* in the main volume gets *theory's names* in the
Seasoned volume.

---

> **This chapter's implementation (and answer key for the exercises)** → [`impls/dist/part9/lib`](https://github.com/rigortype/chibirigor/tree/master/impls/dist/part9/lib)
