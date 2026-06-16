# Glossary

The terms the prose introduces with "this is actually called ◯◯," gathered so you can look
them up.

**Two parts.** **If you're reading only the Little volume, the first half — "Terms you meet in
the Little volume" — is all you need.** The second half, "Seasoned & Rigor-specific terms," is
formal vocabulary and real-Rigor internals; reach for it when you move on to the Seasoned
volume. (Terms that appear in both volumes are placed where you *first* meet them, on the
Little side, with the chapter of first appearance noted in v1 numbering. Cross-volume
references are also organized in the [appendices](appendix/).)

---

# Terms you meet in the Little volume (The Little chibirigor)

## Types and values

- **type carrier** 〔Little P1〕 — a Ruby object that represents a type: `Const` / `Nominal` /
  `Dynamic` / `Union` / `HashShape` / `Tuple`, and so on.
- **`Const` (literal type)** 〔Little P1〕 — a type for "this exact value." E.g. `Const[1]`.
- **`Nominal` (nominal type)** 〔Little P1〕 — a type for a named class. E.g. `Nominal[:Integer]`.
- **`Dynamic` / `untyped`** 〔Little P1〕 — the mark for "the type was lost." The heart of
  gradual typing. The cross-language table and the axis-A/axis-B discussion are in
  appendix a1 (the catalog of special types).
- **`Union` (union type)** 〔Little P4〕 — "either `A` or `B`." E.g. `Integer | String`.
- **`HashShape` (record type)** 〔Little P6〕 — a hash type that remembers a type per key. A
  design that starts from Hack's `shape(...)`, passes through PHPStan/Psalm, and arrives at
  Rigor (see the Little P6 column).
- **rounding / normalization** 〔Little P1〕 — returning a fine-grained type (`Const[3]`) to a
  coarse one (`Integer`).
- **widening** 〔Little P2〕 — when a `Const`'s size exceeds its budget (a threshold), moving
  to a coarse type instead of hoarding values without limit. The same happens when a `Union`'s
  member count exceeds its cap (Little P5). This is the widening of abstract interpretation —
  the design where "types, too, *have a budget*." Where rounding *always* coarsens (it is
  normalization), widening is a cutoff *when the budget is exceeded*.

## Inference and judgment

- **three-valued (trinary) `:yes` / `:no` / `:maybe`** 〔Little P7〕 — the answer of an
  acceptance check. `:maybe` is never punished.
- **narrowing** 〔Little P5〕 — tightening a variable's type per branch of a conditional.
- **dispatch** 〔Little P2〕 — looking up a return type from the receiver and method name.
- **subtype (`<:`)** 〔Little P7 / Seasoned P2〕 — the relation "a value fits in the expected
  type's box." In the Little volume you meet it as `accepts`'s "does it fit in the box"; the
  formalism (the `<:` rules) is handled in Seasoned P2. TAPL ch. 15.

## Design philosophy

- **gradual typing** 〔Little P9〕 — typing that mixes places that have types with places that
  don't.
- **never frighten working code** 〔Little P0〕 — the discipline of avoiding false positives
  above all else.
- **robustness principle (Postel's law)** 〔Little P7 / Seasoned P1〕 — strict in what you
  return, liberal in what you accept.
- **FactStore** 〔Little P5 (naive) / Seasoned P6 (full)〕 — the flow-sensitive store of
  "facts." In the Little volume it is narrowing's naive fact store; the full version is in
  Seasoned P6.

---

# Seasoned & Rigor-specific terms (The Seasoned chibirigor / Rigor)

> From here on are the terms handled as formal vocabulary in the Seasoned volume, and concepts
> internal to real Rigor. **If you're reading only the Little volume, you can skip this.**

## Types and values (fine-grained carriers · Rigor internals)

- **refinement carrier** 〔Seasoned P6 / Rigor-specific〕 — a *type refined by a predicate*,
  such as "non-empty," "a positive value," "literal-derived." Not a subclass of `Nominal`; it
  arises automatically from flow facts. After passing `unless s.empty?`, `s` becomes
  `non-empty-string`. A different concept from `Const[42]` ("the value itself") — `Const` is a
  specific value, a refinement carrier is *the set of values satisfying a predicate*. There are
  `non-empty-string`, `positive-int`, and others, many sharing PHPStan's names (a deliberate
  naming correspondence to lower the learning cost). **The list of built-in carriers (the
  PHPStan correspondence) and the explanation of "why a set difference" are canonical in
  appendix a2-6**; individual narrowing patterns are also in appendix a2. Seasoned P6 §6-1.
- **`Difference` type** 〔Rigor-internal〕 — a type carrier for "the set of values in `A` with
  `B` removed" (set difference `A - B`). `non-empty-string` is implemented internally as
  `String - ""`. This is where the answer to "why that name" lives for the *point-removal* kind
  of refinement carrier (the predicate-subset kind is a separate carrier, `Refined`; ranged
  integers are `IntegerRange`). chibirigor does not handle it. **The details and the two-layer
  structure are canonical in appendix a2-6.**

## Inference and type checking

- **synthesis (`⇒`)** 〔Seasoned P1〕 — building a type *up* from an expression. The Little
  volume's `type_of`.
- **checking (`⇐`)** 〔Seasoned P1〕 — verifying that an expression *fits* against an expected
  type. The Little volume's `accepts`.
- **bidirectional typing** 〔Seasoned P1〕 — the framework that splits typing into the two
  directions, synthesis and checking.
- **type reconstruction / HM** 〔Seasoned P5〕 — inference that recovers types from
  annotations. TAPL ch. 22.
- **unification** 〔Seasoned P5〕 — the operation that finds the substitution making two types
  equal. The core of type reconstruction.
- **erasure (dropping precision at the boundary)** 〔Little P1 (preview: appendix a3-2) /
  Seasoned P3 in full〕 — dropping Rigor's internal precise types (`Constant<"FOO">`, etc.) to
  the coarse types RBS can express (`String`). The operation of "giving up precision at the
  boundary to match the outward-facing type." **Not the same as Java generics' "type
  erasure"** — that erases the type argument `<String>` at runtime; this rounds static type
  *precision* at the boundary. Used inside `sig-gen` (signature generation).

## Subtyping and polymorphism

- **subsumption** 〔Seasoned P1〕 — the rule that a synthesized `S` can be checked against an
  expected `T` if `S <: T`.
- **variance** 〔Seasoned P2〕 — the direction of subtyping at a constructor's argument
  position. Covariant returns, contravariant parameters.
- **algorithmic subtyping** 〔Seasoned P2〕 — reworking the declarative `<:` rules into a
  decision procedure with one rule per type shape. The Little volume's `accepts` is exactly
  this. TAPL ch. 16.
- **kind** 〔Seasoned P4〕 — "the type of a type." The grounds for the well-formedness of a type
  application like `App[F, A]`. TAPL ch. 29.
- **gradual consistency** 〔Seasoned P2 / in full P7〕 — the symmetric, non-transitive relation
  that holds when `untyped` is involved. Distinct from `<:`.
- **substitution / System F** 〔Seasoned P3〕 — the operation of putting a type into a type
  variable. TAPL ch. 23.
- **recursive type (μ) / coinduction** 〔Seasoned P4〕 — a self-referential type and the test
  for its equivalence. TAPL ch. 20–21.
- **HKT (higher-kinded type)** 〔Seasoned P4〕 — a type that takes a type and returns a type.
  `App[F, A]`. TAPL ch. 29.

## Design philosophy

- **soundness = progress + preservation** 〔Seasoned P7〕 — the guarantee that a typed program
  does not fall into undefined behavior. TAPL ch. 8 §8.3.
