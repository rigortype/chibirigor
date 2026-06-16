# EN review — FP-researcher lens (type-theory soundness + international-English clarity)

**Reviewer seat:** French FP/type-theory researcher (ML/OCaml/Coq, gradual typing), non-native English
reader. Two eyes: (a) formal soundness of the claims, (b) places where an idiom/ellipsis makes a
*technical* claim ambiguous for an international reader.

## Verdict

**The formal content is sound.** I found **no ERROR** and **no MISLEADING-class soundness defect**
across Seasoned Part 1–8, appendix a1, and the three type-bearing Little chapters. The hard rules are
correct: Var-Synth / Sub (P1), S-Arrow with contravariant args + covariant returns and the
`Bot <: T <: Top` lattice (P2), capture-avoiding `subst` / α-equivalence / erasure (P3), μ
fold/unfold + coinductive `seen` with an honest "reduced-version / greatest-fixed-point" caveat (P4),
HM / rank-2 decidable / rank-3+ undecidable (Kfoury–Wells) + full-System-F undecidable (Wells), and
occurs-check honesty (P5), progress+preservation / Ω untypeable / gradual consistency `~`
symmetric+non-transitive / the gradual guarantee (P7), and the a1 Top/Bot/untyped/void/never lattice.
The conscious simplifications are all labeled. Everything below is **clarity** or **nitpick** — small
international-English snags where a technical sentence is harder to parse than it needs to be, plus a
few precision notes that the text already half-addresses elsewhere.

---

## Findings

### clarity-1 — Part 2 §2-1 lattice diagram · clarity
**Quote:** ASCII lattice puts `{name:}  Integer  String` on one rank with the side-note
`{name:, age:} <: {name:}` only.
**Issue:** The diagram draws `{name:}` as a flat antichain peer of `Integer`/`String`, but a record
type is *not* a maximal element on that level — it sits above its own width/depth refinements. A
careful reader briefly reads the picture as "`{name:}` is an atom like `Integer`." Appendix a1-4 has
exactly this caveat ("not a maximal element on the same level … extends downward by width subtyping");
Part 2-1, where the lattice is *introduced*, does not.
**Fix:** add a one-line note under the §2-1 figure mirroring a1-4: "`{name:}` is a schematic
representative of the concrete types; structurally it extends downward by width subtyping
(`{name:, age:} <: {name:}`), so it is not actually an atom on this rank." (The depth annotation
already on the line is good; this is just the missing "not an atom" disclaimer at first appearance.)
**Severity:** clarity

### clarity-2 — Part 1 §1-7-a heading + body · clarity (idiom)
**Quote:** "the first **mouth** where `⇐` emits a diagnostic"; "a small window for seeing where
checking `⇐` **opens a mouth** for diagnostics"; "This is the minimal **mouth** that makes `⇐` work."
**Issue:** "mouth" used as a noun metaphor for "point of emission" (口 in the JA) does not carry into
English — to a non-native reader "opens a mouth for diagnostics" is opaque and reads as a
mistranslation rather than a deliberate figure. The metaphor recurs 3× in one section, so it is
load-bearing for comprehension, not decorative.
**Fix:** render as "site"/"point"/"place": "the first **point** where `⇐` emits a diagnostic," "where
checking `⇐` **first produces** a diagnostic," "the minimal **site** that makes `⇐` work outside
arguments." (Same call applies to the §1-3 / Part-7 echoes of "opens a mouth," if any reader-facing.)
**Severity:** clarity

### clarity-3 — Part 7 §7-4 row ④ · clarity
**Quote:** "abandons **the *detection* of** progress (doesn't narrow disjoint / `Dynamic`)" / "doesn't
report the real error of a branch it couldn't narrow and missed."
**Issue:** Rows ①–③ say "abandons progress"; row ④ shifts to "abandons the *detection* of progress,"
which is a different, subtler claim (it is not that the *guarantee* is dropped, but that a *latent*
stuck-state goes *unreported*). For a reader tracking the table's parallelism this asymmetry is
correct but under-explained — "detection of progress" is not a standard phrase and the one-line cell
can't carry it.
**Fix:** spell it in the cell: "does not *abandon* progress so much as fail to **flag** a latent
progress violation — a branch it declined to narrow may hide a real error it never reports." One extra
clause removes the ambiguity. (The claim itself is sound; only the compression misleads.)
**Severity:** clarity

### precision-1 — Part 6 §6-5 join/meet wording · nitpick (already correct, easy to misread)
**Quote:** "'join' is dataflow analysis's idiom … corresponding **not to the type lattice's join
(upper bound) but to the fact lattice's meet (common part).**"
**Issue:** This is *formally correct and commendable* — the author correctly notes that the dataflow
"join" at a control-flow merge is the **meet** in the fact/information lattice (intersection of facts),
while the value types are joined (union). But the sentence packs "join (the word) = meet (the lattice
op)" into one breath; a reader who knows lattice theory does a double-take, and a reader who doesn't
may conflate the two lattices. Nothing to fix in the math.
**Fix (optional):** split into two beats — "At the *value-type* level we **join** (union: the upper
bound). At the *fact* level we **meet** (intersect: keep only facts true on both branches). Dataflow
calls the whole merge operation 'join,' confusingly, after the control-flow sense." Lifts a true-but-
dense sentence.
**Severity:** nitpick

### precision-2 — Part 7 §7-4 table row ② framing · nitpick (labeled simplification, within axis)
**Quote:** "② open hash · unknown key → `nil` | abandons progress (**loosens width subtyping on
arguments**) | a key that's actually absent is `nil` → halts on a call to `nil`"
**Issue:** Strictly, *width* subtyping (accepting a record with **extra** keys) is sound; the hole
that admits the stuck-state is reading an **absent** key as `nil` (a presence/depth concern), not the
width relaxation per se. The cell fuses "open-hash policy" and "width subtyping" into one phrase. This
is the author's deliberate framing of a labeled "deliberately miss," and matches the JA original
verbatim — I flag it only so a soundness-minded reader isn't tripped, **not** as a defect to fix
(per the brief's axis: do not demand rigor beyond the labeled simplification).
**Fix:** none required. If ever revisited: "loosens the *closed-record* assumption on arguments
(open-hash policy) — a missing key resolves to `nil` instead of being rejected."
**Severity:** nitpick

### precision-3 — Part 4 §4-4 "sound reduced version" · nitpick (verified correct)
**Quote:** "This is a sound *reduced version*, weaker than TAPL ch. 21's proper coinduction — which
takes the **greatest fixed point** of pairs reached after unfolding (it catches fewer 'equals')."
**Issue / verification:** Checked against the sketch and the JA. The claim is **correct**: the `seen`
test matches only α-equivalent *pre-unfold* pairs (no unfolding inside `seen`), which strictly
under-approximates the gfp equality — so it never wrongly equates two distinct recursive types
(the "sound" direction holds) and reports fewer equalities (the "weaker / catches fewer" claim holds).
The minor wording wrinkle: "catches fewer 'equals'" applies to *the reduced version* (fewer than gfp),
but the parenthetical sits right after "greatest fixed point," so it momentarily reads as if the gfp
catches fewer. Antecedent is slightly ambiguous.
**Fix (optional):** "(so this reduced test reports *fewer* pairs as equal than the full gfp would)."
**Severity:** nitpick

### precision-4 — Part 5 §5-3 occurs-check note · positive note (no change)
**Quote:** "this sketch omits the occurs-check (a self-reference like `unify(X, X->X)` would pass) …
in real HM the occurs-check is essential to termination and soundness."
**Issue:** None — this is exactly the honesty I want to see. The example `unify(X, X->X)` is the
canonical occurs-check failure, and the claim "essential to termination and soundness" is precisely
right (without it you build an infinite/cyclic type). Flagging as a **commendation**, not a defect.
**Severity:** (none — positive)

---

## Items checked and confirmed SOUND (no finding)

- **P1** Var-Synth, Sub (subsumption = "synthesize then `<:`"), and the claim that no diagnostic is
  born on the `⇒` side (it can only break in Sub's `S <: T` premise, i.e. at a `⇐` position). Correct.
- **P2** S-Arrow (`A' <: A`, `B <: B'`), the running-Ruby `subtype` with `subtype(tp, sp)` swap =
  contravariance, width `(i∈1..n+k)` / depth covariance, `Bot <: T <: Top`, Top/Bot duality,
  read-covariant/write-contravariant/both-invariant containers, algorithmic-vs-declarative subtyping.
  All correct. The `? extends T` / `? super T` Java bridge is accurate.
- **P3** capture-avoiding `subst` (shadowing short-circuit on `params.include?(x)`, then α-rename to
  fresh, then outer subst), α-equivalence via name-correspondence map, erasure theorem vs. Java
  "type erasure" distinction. Correct, and the worked capture trace (`U` vs `U@1`) is right.
- **P4** μ fold/unfold as the same type, equi/iso-recursive trade-off, coinductive `seen`, the
  greatest-fixed-point caveat, HKT grounded in TAPL ch. 29 (kinding) rather than ch. 20/21. Correct.
- **P5** Road A capability gathering ("only messages definitely sent"), Road B constraints +
  unification, the rank/decidability column (HM rank-1 decidable, rank-2 decidable, rank-3+
  undecidable per Kfoury–Wells 1994; unrestricted System-F typeability undecidable per Wells
  1994/1999), occurs-check honesty, `let`-polymorphism correctly placed as out-of-scope, TypeProf
  whole-program vs. Rigor local+catalog. Correct.
- **P7** progress + preservation (Milner "well-typed programs cannot go wrong"), normalization /
  Ω = `(λx.xx)(λx.xx)` untypeable because it needs `A = A->B`, the "what it lets go is mainly
  progress, preservation trivially via widening to `untyped`" framing, gradual consistency `~`
  symmetric + non-transitive (the `Integer ~ untyped ~ String` but not `Integer ~ String` example),
  the gradual guarantee on annotation amount. Correct.
- **a1** the two-axis decomposition of `untyped` (position-in-lattice vs. checked-or-not), `any` as
  "top and bottom at once = a switch turning checking off," `unknown` as a sound top, `void` as a
  return-position-only top-like marker (folded to `top` in value position), `never = Bot` with
  `Bot <: T`, the cross-language tables. All correct.
- **Little P4/P5/P7** untagged-union introduction (correctly distinguished from tagged variants),
  `nil`-narrowing + `is_a?` dead-branch via `possible?` guard (correctly avoids the false positive of
  narrowing `Integer` to `String`), three-valued `accepts` with Union "weakest answer" (actual-side)
  / "strongest answer" (expected-side), "`:maybe` not punished." All correct.
