---
title: Part 1 — What bidirectional typing really is
description: Reveal that the `type_of` / `accepts` built in the Little volume were the skeleton of bidirectional typing — "synthesis (⇒)" and "checking (⇐)."
sidebar:
  order: 11
---

# The Seasoned chibirigor Part 1 — What bidirectional typing really is

> The first chapter of the Seasoned volume. **The Seasoned volume is the "read it" volume, not the
> "build it" one** — we give proper names to the mechanism we built *unconsciously* in the Little
> volume, and read the back of it in formal language. This chapter writes almost no new code. It's
> an overview (a map) revealing what the things we already wrote actually were — though at the end
> we peek, in just one place, at the point where checking `⇐` emits a diagnostic.

In the Little volume (The Little chibirigor), we built two functions as the heart of the
inferencer:

- `type_of(node, scope, diag)` — **find** a type from an expression.
- `accepts(expected, actual)` — **check** whether a type fits an expectation.

These two have proper names in the world of type theory. That's **bidirectional typing.** The
Seasoned volume starts here.

---

## 1-1. Two directions — synthesis and checking

Typing has two modes of opposite direction.

- **Synthesis (`⇒`):** look at an expression and *build a type from the bottom up.* The Little
  volume's `type_of` is this. See `1`, it's `Integer`; see `a + b`, it's (from both their types)
  `Integer`. **Input is just the expression**, output is the type.
- **Checking (`⇐`):** against an *already-expected type*, *check from the top down* whether the
  expression fits. The Little volume's `accepts` (and the dispatch that calls it) is this. "An
  `Integer` is wanted here. Does this expression fit?" **Input is the expression and the expected
  type**, output is a verdict (three-valued in chibirigor).

The Little volume's `type_of` was through-and-through `⇒`, and only where `dispatch` looks at
arguments was it `⇐`. That is, chibirigor was **bidirectional** from the start. As a figure, it's
two arrows of opposite direction:

```text
   expected type T (RBS signature · annotation · declared return type)
            │
            │  ⇐  check        : S <: T ?  ── mismatch → diagnostic
            ▼
   expr e  ──⇒──▶  type S
        synthesize: build a type from the expression (chibirigor never fails; unknown → untyped)
```

![Figure 1-1 — synthesize ⇒ and check ⇐](../figures/svg/seasoned-1-1.svg)
> ▼ Figure 1-1 — synthesize `⇒` (build a type from the bottom up) and check `⇐` (match against the expected type above)

> [!TIP]
> **Reference note (optional).** The form of bidirectional typing rests on the typing rules of
> TAPL ch. 9 (the simply-typed lambda calculus). 『しくみ』 ch. 3 (function types)'s `typecheck` is
> made of pure `⇒` (synthesis) alone — that book's mini-language always has type annotations on
> parameters, so it didn't need to be aware of `⇐` (checking) independently.
>
> chibirigor, dealing with annotation-free Ruby, had to clearly separate `⇒` and `⇐` — that's
> bidirectional typing.

---

## 1-2. Writing the rules formally

In the Seasoned volume we don't fear notation. A typing rule writes the "premises" above a
horizontal line and the "conclusion" below. There are two judgments:

- Synthesis: `Γ ⊢ e ⇒ T` (under environment Γ, synthesizing the type of expression e gives T)
- Checking: `Γ ⊢ e ⇐ T` (under environment Γ, expression e checks against expected type T)

(`⊢` is a type-theory symbol read "turnstile," meaning *under the premises on the left, the right
is derivable*. `⇒`/`⇐` were the synthesis/checking directions.)

`Γ` (gamma) is the Little volume's `Scope` (variable name → type). For example, the synthesis rule
for a variable reference:

```
  x : T ∈ Γ
  ───────────  (Var-Synth)
  Γ ⊢ x ⇒ T
```

"If `x : T` is in Γ, then `x`'s type is synthesized as `T`." The Little volume's
`scope.local(node.name) || Dynamic` is exactly this rule (with a chibirigor-style footnote: if it
isn't there, fall back to `untyped`).

And the most important rule, tying the two directions together — **subsumption**:

```
  Γ ⊢ e ⇒ S    S <: T
  ─────────────────────  (Sub)
  Γ ⊢ e ⇐ T
```

"Synthesizing expression e gave S. If S is a subtype of T, then e checks against T." **Checking is
'synthesize, then match by subtyping'** — this is the true form of the Little volume's "`type_of`
an argument, then `accepts`." The relation between `<:` (subtyping) and chibirigor's three-valued
`accepts` is pinned down in Part 2.

---

## 1-3. Why a diagnostic only comes out in checking (`⇐`)

In the Little volume we wrote the most important sentence thus: "a diagnostic only appears where the
expected type is fixed." In bidirectional words:

> [!IMPORTANT]
> A diagnostic is born **only when `S <: T` breaks in the subsumption of checking `⇐`.** A `⇐`
> position = **where the expected type T is declared/known**: an explicit annotation, a declared
> return type, and also **the argument position of a method call listed in RBS (or the Little
> volume's hand-written table).**

Let's be precise here. It is **not** that "annotation-free code has no `⇐` position" — a core-type
method call (the `+` in `1 + x`, etc.) is a `⇐` position even with zero annotations, because RBS
gives the argument's expected type. So *why* does working code still go unfrightened? There are two
reasons, both corresponding to the Little volume's Part 9 "deliberately miss":

1. **Synthesis `⇒` deliberately doesn't fail (chibirigor-specific).** In bidirectional typing *in
   general*, synthesis can fail on an unbound variable or unsynthesizable syntax. chibirigor
   *intentionally* synthesizes every unknown into `untyped`, totalizing it. So even when "an
   expression that lost its type" comes to a `⇐` position, the synthesis result is `untyped`.
2. **`untyped` passes straight through checking.** `accepts` is unconditionally `:maybe` (not
   punished) if either side is `Dynamic`. So even when `untyped` comes to a `⇐` position, it's not
   a diagnostic.

To sum up — working code goes unfrightened not because "there's no `⇐` position" but because
**"synthesis collapses the unknown to `untyped`, and checking doesn't punish `untyped`."** It's a
property statable only by laying chibirigor's two gradual design judgments (① totalizing synthesis,
② lenient checking) onto the bidirectional framework. Seasoned Part 7's "deliberately let go of
soundness" catches these same two points in the language of progress/preservation — we recover, late
in the volume, the foreshadowing dropped at the entrance.

---

## 1-4. The robustness principle is the direction itself

In Little Part 7 we saw the asymmetry "strict on returns, lenient on arguments." Seen
bidirectionally, this is *the direction itself*:

- **The return value is synthesis `⇒`:** build *the narrowest type* from the body (strict = high
  precision).
- **An argument is checking `⇐`:** receive *leniently* against the expected type (let `:maybe`
  through too).

The robustness principle Rigor upholds (Postel's law: "strict in what you return, liberal in what
you accept") overlaps the two directions of bidirectional typing exactly, on one sheet. In the
Little volume it appeared as a practice; in the Seasoned volume, as theory.

---

## 1-5. How it works inside Rigor

Real Rigor's architecture has these two directions as parts directly:

- **`ExpressionTyper`** (typing an expression) = synthesis `⇒`. Pure, non-destructive, builds a
  type from an expression.
- **`accepts` at a call site** (the acceptance check, three-valued + reason) = checking `⇐`. Works
  only where an RBS declaration gives the expected type T.
- **`Scope` / `FactStore`** = the environment Γ (far richer than the Little volume's; treated in
  Part 6).

chibirigor's `type_of` / `accepts` / `Scope` were the minimal version of these three. The Seasoned
volume adds flesh to this skeleton.

> [!NOTE]
> **Where the notation comes from.** Bidirectional typing is a framework popularized by Pierce &
> Turner, "Local Type Inference" (2000), with Dunfield & Krishnaswami's survey "Bidirectional
> Typing" (2021) as the modern overview. It's the apparatus for organizing where to place
> annotations so type inference runs deterministically.

---

## 1-6. What the Seasoned volume stacks on this — the volume's map

With the map of bidirectionality in hand, the rest of the Seasoned chapters are surveyable by
"which part of the map they fill in." The whole is one long climb: "① bidirectional (the map) →
② the structure of types → ③ inference and flow → ④ the peak of theory → ⑤ the bridge":

- **Part 2 Subtyping and variance:** refine subsumption's `<:`. In particular, the story where
  *a function's argument is contravariant* — the direction twists one more turn (the climax of
  『しくみ』 ch. 7).
- **Part 3 Generics and type substitution:** type variables `(X) -> X` and type substitution
  (System F). Part 5's inference references this *backward*.
- **Part 4 Recursive types: μ and coinduction:** how `⇒`/`⇐` handle a type that references itself
  (μ-types / coinduction ↔ Rigor's HKT). Adjacent to Part 3, tying up the mutual reference of
  α-equivalence.
- **Part 5 Real type inference:** fill in what the Little volume fell back to `untyped` with `⇒` —
  *arguments* in particular — from how they're used in the body. Widening `⇒`'s coverage (the
  frontier of 『しくみ』 ch. 9's exercises). The TypeProf comparison is consolidated here too.
- **Part 6 The complete FactStore:** extend the environment Γ into a flow-sensitive set of facts.
- **Part 7 Soundness and normalization:** the peak that pairs with this chapter's bidirectional map.
  Read the design that "deliberately lets go" of soundness in the language of
  progress/preservation. We recover §1-3's foreshadowing here.
- **Part 8 Toward real Rigor:** the bridge handing off from the minimal version to real Rigor.

---

## 1-7. This chapter's summary

We wrote almost no new code. Instead we gave names to what we built in the Little volume:

| What we wrote in the Little volume | What it really is |
|---|---|
| `type_of` (find a type) | synthesis `⇒` (synthesize) |
| the argument check of `accepts` / `dispatch` | checking `⇐` (check) = synthesize, then `<:` |
| `Scope` | the type environment Γ |
| "diagnostics only where there's an expectation" | a diagnostic is born only at a `⇐` position (follows from the structure) |
| "strict on returns, lenient on arguments" | synthesis `⇒` strict / checking `⇐` lenient (the robustness principle) |

---

### 1-7-a A note — the first point where `⇐` emits a diagnostic (`check(rbs:)` mode)

In §1-3 we said "a diagnostic only appears in checking (`⇐`)." The `check` up to the Little volume
used checking only in "`dispatch`'s argument check." Add to it a `check(source, rbs:)` mode that
**checks the body against a declared return type**, and a scene arises where `⇐` works outside the
"caller side" too. The only code we peek at this chapter is this one — a small window for seeing
where checking `⇐` first raises a diagnostic.

```ruby
rbs = <<~RBS
  class Greeter
    def greet: () -> Integer
  end
RBS

Chibirigor.check('def greet; "hi"; end', rbs: rbs)
# => [{line: 1, message: "return type Integer is declared but \"hi\" is returned"}]

Chibirigor.check('def greet; 1; end', rbs: rbs)   # => [] (matches)
Chibirigor.check('def greet; "hi"; end')           # => [] (no declaration = no check)
```

**How it works** (excerpt from `lib/chibirigor/checker.rb`)

1. Collect body errors in the usual `eval_statement` loop (as before).
2. If `rbs:` is present, load the user declarations with `Rbs.load(rbs)`, and for each `DefNode`
   call `check_against(node, declared_return, body_type, diagnostics)`.
3. `check_against` adds a diagnostic only when `Accepts.call(expected, actual) == :no`. If `untyped`
   is involved, it stays quiet (gradual's promise).

```ruby
# ⇐ subsumption: check actual against expected. diagnose only :no.
def check_against(node, expected, actual, diagnostics)
  return if expected.is_a?(Type::Dynamic) || actual.is_a?(Type::Dynamic)
  return unless Accepts.call(expected, actual) == :no

  diagnostics << diagnostic(node, "return type #{expected} is declared but #{actual} is returned")
end
```

This is the minimal point at which `⇐` does work outside "arguments." Recall §1-4's robustness
principle — "**the return type strictly** (checked with `⇐`)" appears in the implementation for the
first time here. It pairs with the Little volume's `dispatch` "checking arguments with `⇐`."

**Zero-FP guarantee.** Pass no `rbs:` and it checks nothing (opt-in). If the declaration is
`untyped`, no check. Even if the body is `untyped` (when inference lost the type), no check. With
this, "never frighten working code" is kept in return-type checking too.

> [!NOTE]
> **The `param:` directive does two jobs**
>
> Rigor has a **`param:` directive** written with the RBS extension annotation
> `%a{rigor:v1:param: name is String}` (it can't be written inline in the `.rb` body; it's placed
> on the RBS side). It brings **two effects at once**:
>
> 1. **Body narrowing** (gives the `⇒` side a starting point for the type): inside the body,
>    `name`'s type starts from the declared `String`. Inference proceeds from the declared type
>    from the start, without writing `is_a?`.
> 2. **`⇐` checking at the call site:** for a call like `greet(42)`, the argument type is checked
>    against the declaration with `accepts`, and `:no` adds a diagnostic.
>
> That is, `param:` carries both "a promise to the writer (the body follows the type)" and "a
> promise to the caller (keep to the argument)" at once. That both are unified as `⇐` (checking) is
> the clean point of bidirectional typing.

---

## Exercises

1. Using only the subsumption (Sub) rule, explain in two lines that "no diagnostic is born on the
   synthesis `⇒` side" (hint: where in Sub's premise does the failure of `S <: T` occur).
2. Consider annotation-free Ruby `1 + x` (`x` unbound). (a) What is `x`'s synthesis result? (b) Is
   `+`'s argument position a `⇐` position? (c) Why is there still no diagnostic — which of §1-3's two
   reasons works?
3. Restate the robustness principle "strict on returns, lenient on arguments" using only the words
   `⇒`/`⇐` (with a phrase for why each direction is strict/lenient).

---

**Next chapter (Part 2):** we build subsumption's `<:` head-on. The width/depth subtyping of
objects, and **contravariance** — where the argument direction alone reverses when you pass a
function — that climax 『しくみ』 ch. 7 called "variance changing direction in the implementation."
