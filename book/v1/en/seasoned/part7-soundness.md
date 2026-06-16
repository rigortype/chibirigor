---
title: Part 7 — Soundness, normalization, and "unsound on purpose"
description: "Set the central theorem of type theory (progress + preservation), and restate formally why chibirigor is deliberately unsound. Consolidate gradual's two disciplines and the termination engineering (coinduction, fuel, budget) here."
sidebar:
  order: 17
---

# The Seasoned chibirigor Part 7 — Soundness, normalization, and "unsound on purpose"

> References (optional): TAPL ch. 8 "Typed arithmetic expressions" §8.3 (safety = progress +
> preservation), ch. 12 "Normalization" / 『しくみ』 afterword. This chapter sets the central theorem
> of type theory and states outright, in formal language, *why chibirigor is deliberately unsound.*

This chapter's goal: **define head-on "what a type system is" and "what soundness is," and put into
words the meaning of chibirigor (and Rigor) *intentionally* letting go of that soundness.**

In Part 6 we built the tools all the way to the real **FactStore** (six kinds of facts,
invalidation, closure capture) that supports flow. Finally, stepping back a notch, we state outright,
in the language of theory, what the thing we've built guarantees as a "type system," and what it
*deliberately* doesn't.

---

## 7-1. The narrow and broad senses of "type system"

Borrowing 『しくみ』's afterword's organization:

- **Narrow:** a type system = a collection of **typing rules** (how to type which expression).
- **Broad:** that plus the *theorems provable* about the rules — in particular **type safety
  (soundness)** and **normalizability.**

And a **type checker** is a program that mechanically decides "can the input program be explained by
the typing rules." What we built in the Little volume was exactly this "program that made the rules
mechanically decidable." Lining up TAPL ch. 9 Figure 9-1's variable rule T-Var with the
`LocalVariableRead` clause of the Little volume's `type_of`, you see the rule and the code correspond
one to one.

---

## 7-2. Soundness = progress + preservation

The standard formulation of **type safety (soundness)** is a pair of two theorems (TAPL 8.3):

- **Progress:** a typed term is either a value or *a next evaluation step definitely exists* (= it
  doesn't get stuck = it doesn't fall into undefined behavior).
- **Preservation (subject reduction):** evaluating a typed term one step *keeps the type.*

Together: "**a typed program doesn't get stuck at any point of evaluation.**" This is the substance of
the guarantee "if it types, it won't fall into undefined behavior." "**Well-typed programs cannot go
wrong**" (Milner).

---

## 7-3. Normalization — if it types, it halts

Another theorem is **normalizability** (TAPL ch. 12). In the simply-typed lambda calculus, **every
typed term necessarily halts.** Something like `(λx. x x)(λx. x x)` (the divergent term Ω) doesn't
type. The key is the self-application `x x` — to use `x` as both "a function taking `x`" and "its
argument," you'd need `x : A -> B` and `x : A`, i.e. a *recursive* type `A = A -> B`. Simple types
don't have that, so Ω is untypeable. So "if it types, it halts" doesn't break (add recursive types
and the story changes — Seasoned Part 4).

Normalization (if it types it always halts) is a core property of the simply-typed setting, the
subject of TAPL ch. 12 — add recursive types (『しくみ』 ch. 8) or general recursion and it doesn't
hold in general. And — **chibirigor doesn't have this property.** Because its counterpart is real
Ruby, which doesn't halt.

---

## 7-4. Why is chibirigor deliberately unsound

This is the most important section of the Seasoned volume. What we built in the Little volume is **not
sound.** On purpose.

A sound checker guarantees "types ⇒ definitely safe," but in exchange **rejects programs that are safe
but unprovable** (a conservative false positive). Real dynamic-language code is that lump of "safe but
hard to prove." Insist on soundness here and **working code goes bright red.**

So chibirigor (and Rigor) **intentionally let go** of soundness.

**The "four places we deliberately miss," raised as a slogan in Little Part 9 §9-2, we restate here in
§7-2's language (progress + preservation).**

A sound type system guarantees "types ⇒ execution doesn't get stuck (progress)." chibirigor's four
holes are each places that **deliberately let go of this progress guarantee** — that is, they allow
"even if it types, it *can* get stuck with a `NoMethodError`, etc." The "stuck state" each hole admits
is this:

| Little §9-2's slogan | In progress/preservation words | The "stuck state" it admits |
|---|---|---|
| ① `untyped` accepts anything | abandons progress (accepts `untyped` against any expected type = passes subsumption straight through) | calling a method an `untyped` value doesn't answer → `NoMethodError` |
| ② open hash · unknown key → `nil` | abandons progress (loosens width subtyping on arguments) | a key that's actually absent is `nil` → halts on a call to `nil` |
| ③ doesn't punish `:maybe` | abandons progress (doesn't promote the unprovable to `:no`) | a `:maybe` is actually a mismatch → a runtime type error |
| ④ conservative narrowing | abandons the *detection* of progress (doesn't narrow disjoint / `Dynamic`) | doesn't report the real error of a branch it couldn't narrow and missed |

On the preservation side (the type is kept under evaluation), chibirigor doesn't *strongly assert* it
to begin with — when things get doubtful it widens to `untyped`, so preservation is trivially kept in
the sense of consistency (`~`) (when types stop fitting, just fall back to `untyped`). **What it lets go
of is mainly progress** — that's the true form of these four places.

Formally: **chibirigor gives up soundness — progress in particular — in a limited way, in exchange for
fewer false positives ("never frighten working code").** This is not a defect but **a design choice.**
Soundness means "miss not a single bug," but the cost (frightening working code) is judged, for a
practical checker, to be higher.

> **The volume's bookend.** The promise *foreshadowed* in Seasoned §1-3 ("never frighten working code /
> quietly pass what you don't know"), the Seasoned volume *recovers head-on* here, in the language of
> progress/preservation. Seasoned Part 1 (the entrance), which spread the bidirectional map, and this
> chapter, which states the design that "deliberately lets go" of soundness, answer each other.

---

## 7-5. Gradual's two disciplines — consistency and guarantee

"So is it then completely anything-goes?" No. Soundness is let go, but gradual typing has **other
disciplines.** What chibirigor keeps is these two.

### Discipline 1 — gradual consistency `~` (the discipline on the type side)

Once `untyped` (`Dynamic`) is involved, subtyping `<:` becomes a special relation that **passes both
ways.** This is called **gradual consistency** `~` (a point sent from Seasoned Part 2 §2-5):

- both `untyped ~ T` and `T ~ untyped` hold (**symmetric**).
- but `~` is **not transitive**: even with `Integer ~ untyped` and `untyped ~ String`, not `Integer ~
  String`. So while making `untyped` "an escape hatch that becomes anything," the mix-up of `Integer`
  and `String` can still be made `:no`.

> `~` is the academic-convention symbol of gradual typing (Siek). Real Rigor, avoiding a symbol clash
> in its docs, spells this relation `consistent(A, B)` (`~T` is reserved for negation / complement).
> The meaning (symmetric, non-transitive) is the same.

The Little volume's three-valued `accepts` was exactly for **folding this `<:` (provable / not) and `~`
(untyped involved) into one judgment**:

| Situation | Result |
|---|---|
| `S <: T` is provable | `:yes` |
| `untyped` is involved (`~` but `<:` unknown) | `:maybe` |
| `S` and `T` are definitely unrelated | `:no` |

It's precisely because `~` is **non-transitive** that `untyped` doesn't become an unlimited escape
hatch — this is the first meaning of "not anarchic."

### Discipline 2 — the gradual guarantee (the discipline on the annotation-amount side)

The other is the **gradual guarantee** (Siek et al.). A discipline not on the type side but on *the
relation between annotation amount and behavior*:

> **Adding or removing** annotations, the program's meaning (passes / doesn't, behavior) changes
> *continuously.* Reducing type annotations just makes checking looser; it doesn't suddenly break.

chibirigor's design of "fall back to `untyped`" and "make no checking position where there's no
annotation" is a naive expression of this gradual guarantee. Adding one annotation doesn't suddenly
turn an unrelated spot bright red — this is the second meaning of **soundness is let go, but it isn't
anarchic.**

`~` (type consistency) and the guarantee (continuity in annotation amount) — these **two disciplines**
distinguish "unsound on purpose" from anarchy.

> **Reference note.** Static `<:` is the world of TAPL ch. 15, 16. The `~` with `untyped` added is
> *beyond* TAPL — the realm of gradual typing (Siek & Taha 2006), the core of "unsound on purpose"
> treated in Little Part 9 and this chapter.

> **Column: with the `assert:` directive, "promise having confirmed it"**
>
> Even though Rigor is unsound, the back-door for regaining soundness at specific spots is the **`assert`
> directive** (an RBS extension annotation). It's put on methods that check / normalize:
>
> ```ruby
> # write on the RBS side (can't be written inline in the .rb body)
> # %a{rigor:v1:assert x is non-empty-string}
> def ensure_present!(x) = (raise if x.nil? || x.empty?)
> ```
>
> Its meaning is "**after this method returns, the *caller's* `x` is `non-empty-string`**" — a **gate**
> that adds a fact not inside the body but to the *caller's* scope. Even after dynamic dispatch or
> `eval` that inference can't follow, passing through a guard method once lets you declare "from here on
> I guarantee it." Write a lie and the diagnostics beyond it become lies (`accepts` turns into `:yes`).
>
> | Mechanism | Grounds for the type |
> |---|---|
> | `is_a?(String)` | a fact Rigor inferred |
> | `%a{rigor:v1:param: x is String}` | declaration → checking at the call site + binding in the body |
> | `%a{rigor:v1:assert x is String}` | after the guard passes, forces the *caller's* `x` |
>
> A role close to TypeScript's *user-defined type guard* (a predicate function returning `x is T`). In
> Rigor it's treated not as an "unsound type cast" but as "an operation that adds a fact having
> confirmed it."

---

## 7-6. How to keep termination in engineering — coinduction, fuel, budget

chibirigor can't have normalization (if it types, it halts) either. Where theory guarantees "if it
types, it halts" by *proof*, a practical checker keeps "**the analysis itself halts**" by
*engineering.* The termination tools that appeared up to this chapter line up as a **self-similar
shape** of the same contrast, at three sizes.

### Coinduction — "stop it correctly" (Seasoned Part 4)

The type equivalence test of equi-recursive doesn't halt if left alone (it keeps unfolding
`μX.{foo:X}`). Seasoned Part 4 cut this with **coinduction = the assumption set `seen`** — "if asked
the pair currently being compared again, *assume* equal and cut off." This is a solution that **stops
it correctly** (subtyping as the greatest fixed point, TAPL ch. 21).

### HKT reduction fuel — "stop when it gets dangerous" (Seasoned Part 4, ADR-20)

Rigor doesn't implement recursive types directly with μ + coinduction; it makes a lightweight HKT
(`Type::App`) + a **fuel budget** the alternative (a point sent from Seasoned Part 4 §4-5). It consumes
a counter each time it unfolds a higher-order type, cutting off at the ceiling (default 64 steps). This
is a solution that **stops when it gets dangerous** — when fuel runs out it drops the result to
`:maybe` and escapes to `untyped`, landing on the safe side.

| Way to stop | Character | Where |
|---|---|---|
| coinduction (`seen`) | stop it correctly (greatest fixed point) | the teaching equivalence test (Seasoned Part 4) |
| HKT reduction fuel (ADR-20) | stop when it gets dangerous | implemented in Rigor |
| inference budget (ADR-41) | stop when it gets dangerous (broad sense) | designed, unimplemented |

### Inference budget (ADR-41) — a larger self-similar shape

Where fuel keeps the *local* termination of HKT reduction, a broader **inference budget** (ADR-41) is
also designed — cutting off the whole analysis with a budget, a large self-similar shape of fuel. But
this one is *currently unimplemented* (ADR-41 is Status: Proposed).

The *beauty of theory* "a type system guarantees normalization" is replaced, in a practical checker,
by the *engineering* of "**cut off the analysis with a budget.**" Coinduction (stop correctly) → fuel →
budget (stop when dangerous) is the same answer, at varying scales, to the same question of "how to stop
the unfolding." That a practical checker can choose "stop when dangerous" was because of gradual's
settling: "fall back to `untyped` when you can't tell."

---

## 7-7. Summary

- A type system = the narrow sense (typing rules) + the broad sense (the theorems of soundness and
  normalization). A checker = a decision procedure for the rules.
- **Soundness = progress (doesn't get stuck) + preservation (the type is kept under evaluation)**
  (TAPL 8.3).
- **Normalization** = if it types, it halts (TAPL 12). Add recursive types or general recursion and it
  doesn't hold in general.
- **chibirigor is deliberately unsound:** it gives up soundness (progress in particular) in a limited
  way, in exchange for fewer false positives. Little §9-2's "four places it misses" are the true form
  of those holes. We recover §1-3's foreshadowing here.
- But it isn't anarchic — **gradual's two disciplines:** consistency `~` (symmetric, non-transitive)
  and the gradual guarantee (behavior continuous in annotation amount).
- Instead of normalization, it keeps termination by **engineering** — coinduction (stop correctly) →
  HKT fuel (ADR-20, implemented) → inference budget (ADR-41, designed, unimplemented), a self-similar
  shape at varying scales.

## Exercises

1. **Restate a hole in progress:** pick one of the four in §7-4's table and make one pair of concrete
   *working Ruby and an input that gets stuck on it* the hole admits (e.g. `untyped` acceptance →
   `x.foo` is `NoMethodError`).
2. **Progress and preservation:** is "even if it types, it can be a `NoMethodError`" an abandonment of
   progress or of preservation? State it along with why chibirigor can mostly keep preservation (widen
   to `untyped`).
3. **Ω is untypeable:** show where you get stuck trying to give `(λx. x x)` a simple type (`A = A -> B`
   is needed), and in a phrase what changes when you add recursive types (Seasoned Part 4).
4. **The two disciplines:** explain, in one sentence each, that gradual consistency `~` is
   *non-transitive* and that the gradual guarantee works on *annotation amount*, each by "what would
   break if this discipline didn't exist."
5. **Line up the ways to stop:** classify the three — coinduction (`seen`), HKT fuel, inference budget
   — into "stop correctly / stop when dangerous," and state in a phrase why Rigor can choose the latter
   (gradual's settling).

**Next chapter (Part 8, finale):** from chibirigor's minimal version to real Rigor's build-out
(plugins, cache, LSP, performance, baseline) — we build the bridge and close the Seasoned volume.
