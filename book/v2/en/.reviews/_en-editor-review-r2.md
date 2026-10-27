# chibirigor v2/en — editorial review r2 (sync since b1e59ad)

Lens: technical-book editor (screen-first voice/scannability/calques + read-feel economy,
signposting, antecedents, promise→payoff). Not a correctness pass. Scope: prose changed in
`git diff b1e59ad..HEAD -- book/v2/en` (25 files), read against the surrounding paragraphs.
House style (spaced ` — `, `①②③`, `『しくみ』`, "the three perspectives", GitHub alerts) is not
flagged. Teaching code blocks are left alone. Line numbers are against HEAD `e6c11d0`.

## Headline

The synced corrections are **accurate in content but have mostly been added, not woven in.**
The pattern is the same nearly everywhere: an existing sentence about "Rigor" is narrowed to
"chibirigor", and a new "Real Rigor does X instead" sentence (or paragraph) is added after it.
That keeps the facts straight, but it leaves four kinds of seam:

1. **Stale neighbours.** A sentence nearby still says "Rigor" where the chapter now says
   chibirigor (Little P7:107/229), or an example no longer fits the claim around it (a5-5, a5-3
   Hack, Little P6 NOTE).
2. **Caveat pile-up.** The same "real Rigor differs" point appears two or three times in one
   chapter (Seasoned P4: cycle detection + fuel ×3; Seasoned P5: "precision only, no diagnostics"
   ×3; sig-gen's `untyped` default twice, back to back).
3. **Over-long, hedged sentences** that came straight from the JA corrections, often with
   object-fronting calques ("An unsolvable argument it drops…", "Whether a block is … Rigor looks
   up…", "How far real Rigor goes in reporting, we'll see…").
4. **Framing that wasn't updated.** Seasoned P7 §7-6's intro dropped the "self-similar" framing,
   but its subsection heading and summary still use it. Seasoned P4/P7 still introduce *fuel* as
   "the alternative" and only then bring in cycle detection as the main mechanism.

There are 8 ERRORs (broken logic, a wrong signpost, two garbled or ambiguous sentences), 30
IMPROVEs and 11 NITs. Most are local, so each can be replaced mechanically. The Little volume's
warm voice mostly holds. The parts that drift cooler are the new "Real Rigor…" paragraphs:
they read like spec notes, not like the book's voice.

---

## ERROR

### E1. seasoned/part6-fact-store.md:67-68 — garbled clause
Current:
```
5. **dynamic_origin** — facts about dynamic provenance (in real Rigor, for example, a block escaping
   so that when it gets called can't be read).
```
Proposed:
```
5. **dynamic_origin** — facts about dynamic provenance (in real Rigor, for example, that a block
   has escaped, so we can't tell when it will be called).
```
Why: "so that when it gets called can't be read" has no subject. It is a literal rendering of
「いつ呼ばれるか読めない」.

### E2. appendix/a3-tooling.md:120-123 — "the internal type of 1" reads as the type of the literal `1`
Current:
```
What `annotate` and diagnostics show is the internal type of 1; 2 appears when `sig-gen` writes out
RBS. The real thing has a double structure of
"**knowing precisely inside, throwing it away at the boundary**," so "inference seems to know more,
yet why is the signature (RBS) so coarse" happens. The answer is this two-layer setup.
```
Proposed:
```
`annotate` and the diagnostics show layer 1, the internal type; layer 2 appears only when
`sig-gen` writes out RBS. Because the real thing "**knows precisely inside and throws it away at
the boundary**," you meet the puzzle "inference seems to know more — so why is the signature (RBS)
so coarse?" The answer is this two-layer setup.
```
Why: in a book full of `Const[1]`, "the internal type of 1" reads as a type for the value 1. It
should say "layer 1". The "…happens" clause is also a calque.

### E3. seasoned/part5-real-inference.md:209 — wrong signpost
Current:
```
The previous section's "Road B" gathered constraints from how the body uses a value and solved them.
```
Proposed:
```
§5-3's "Road B" gathered constraints from how the body uses a value and solved them.
```
Why: the previous section is §5-4 ("Why we don't do all of it"). Road B is in §5-3.

### E4. little/part6-hash-and-tuple.md:156-160 — broken causal link
Current:
```
> "A structural hash type that remembers keys and value types" is not Rigor's invention. Several
> type checkers have hit the same problem (Hack's `shape`, PHPStan/Psalm's `array{...}`), because a
> naive join mixes the value types into `String | Integer` and loses the per-key information. Each
> tool has drawn its own line between open and closed, and which one is the default differs by
> tool: Hack's `shape` is closed by default and becomes open when you write `...`. chibirigor's `HashShape` is the minimal implementation of that
> lineage. (For each tool's syntax and history, see appendix
```
Proposed:
```
> "A structural hash type that remembers keys and value types" is not Rigor's invention. Several
> type checkers hit the same problem and reached the same kind of type (Hack's `shape`,
> PHPStan/Psalm's `array{...}`): a naive join mixes the value types into `String | Integer` and
> loses the per-key information. Where each tool draws the line between open and closed, and which
> one is the default, differs — Hack's `shape`, for instance, is closed by default and opens up when
> you write `...`. chibirigor's `HashShape` is the minimal implementation of that lineage. (For each
> tool's syntax and history, see appendix
```
Why: the old sentence linked "chose open — because naive join…". After the sync, "because"
attaches to "hit the same problem", which doesn't follow: the naive join is why they built the
type, not why they hit the problem.

### E5. appendix/a5-other-languages.md:73-75 — the Hack bullet contradicts itself
Current:
```
- **Hack (Facebook):** a language that added static types to PHP. It introduced the type
  `shape('name' => string, 'age' => int)` and adopted the design "spell out the keys." The default
  is closed; writing `...` makes it allow extras (open). A design conscious of coexisting with options hashes from the start.
```
Proposed:
```
- **Hack (Facebook):** a language that added static types to PHP. It introduced the type
  `shape('name' => string, 'age' => int)` and adopted the design "spell out the keys." The default
  is closed; writing `...` opens it to extra keys, which is how it makes room for options-hash-style
  code.
```
Why: the left-over closing sentence ("conscious of coexisting with options hashes from the
start") was written for the old "open" claim. Next to "the default is closed" it reads as a
contradiction. (If the fidelity lens doesn't want the "how it makes room" gloss, just delete
the sentence.)

### E6. appendix/a5-other-languages.md:110-117 — the illustrating example is the one case Rigor doesn't report
Current:
```
if the `case`s don't cover every pattern, the compiler stops you with a "*missing arm*." Rigor (and
chibirigor) don't ask about "missing"; instead they report an "**unreachable arm**." When you write
`if x.is_a?(String)` even though `x : Integer`, that branch never executes. chibirigor finds it with
`check --unreachable` and tells you "this is a superfluous branch" (**opt-in**; by default it keeps
"stay quiet about working code" and stays silent. A runnable worked example is in appendix
[a1-3x](a1-special-types.md)). Real Rigor reports, by default, `case`/`when` clauses that can never
match and branches like `if false`, but dead branches arising from `is_a?` are out of scope at
present.
```
Proposed:
```
if the `case`s don't cover every pattern, the compiler stops you with a "*missing arm*." Rigor (and
chibirigor) don't ask about "missing"; instead they report an "**unreachable arm**." Real Rigor
reports, by default, a `case`/`when` clause that can never match and a branch like `if false`.
chibirigor goes after a different case: write `if x.is_a?(String)` even though `x : Integer`, and
`check --unreachable` tells you "this is a superfluous branch" (**opt-in**; by default it stays
quiet about working code. A runnable worked example is in appendix [a1-3x](a1-special-types.md)).
Real Rigor doesn't report that `is_a?` case at present.
```
Why: "Rigor (and chibirigor) … report an unreachable arm. When you write `if x.is_a?…`" makes
the `is_a?` case the shared example, and then the paragraph's last clause takes it back. Put
the real-Rigor cases first, then chibirigor's.

### E7. little/part7-accepts-and-trinary.md:107-108 and :229-230 — "Rigor" left behind after the chapter switched to chibirigor
Line 12 now says "the single biggest reason **chibirigor** can stay gentle", and line 115 says
real Rigor answers `:yes`, not `:maybe`. Two sentences still give Rigor the `:maybe` mechanism.

Current (107-108):
```
Just one line, but it's one of the main pillars making Rigor a "type checker gentle to dynamic
languages." `untyped` is the mark of "the type was lost." About something lost, not insisting it
```
Proposed:
```
Just one line, but it's one of the main pillars making chibirigor a "type checker gentle to dynamic
languages." `untyped` is the mark of "the type was lost." About something lost, not insisting it
```
Current (229-230):
```
Rigor goes one step further, **placing "unknown = maybe = stay quiet" at the center of the
mechanism,** making false positives hard to produce in the first place.
```
Proposed:
```
chibirigor goes one step further, **placing "unknown = stay quiet" at the center of the
mechanism** (here, as `:maybe`), making false positives hard to produce in the first place.
```
Why: within one chapter the reader is told that Rigor centres `:maybe`, and eight lines later
that it doesn't. (The proposed 229 keeps the "stay quiet" spine, which Rigor shares, and pins
`:maybe` to chibirigor.)

### E8. seasoned/part4-recursive-types.md:234-237 — the table header no longer matches its rows
Current:
```
> | Why a verdict can't be made | chibirigor's `accepts` | real Rigor |
> |---|---|---|
> | `untyped` is mixed in | `:maybe` | `:yes` (passes through gradually) |
> | the HKT's upper bound `bound` is still the default `untyped` | — | `:yes` (passes through gradually) |
```
Proposed:
```
> | Where the type is unknown | chibirigor's `accepts` | real Rigor |
> |---|---|---|
> | `untyped` is mixed in | `:maybe` | `:yes` (let through, gradual) |
> | the HKT's upper bound `bound` is still the default `untyped` | (no HKT) | `:yes` (let through, gradual) |
```
Why: in the real-Rigor column a verdict *is* made (`:yes`), so "Why a verdict can't be made" is
now false for half the table. A bare "—" in the chibirigor column is ambiguous. On "passes
through gradually", see I10.

---

## IMPROVE

### I1. little/part0-introduction.md:86-91 — "pre-pass" collides with line 65; "This one line" lost its antecedent
Current:
```
> inference and buy scale and "silence" (no false positives)** instead. Rigor doesn't run the
> whole program from an entry point either, and by default it doesn't infer arguments backward.
> There is an opt-in path that collects types from call sites; turning it on adds a pre-pass
> that visits every call site once. Even so, what it collects is used only to add precision —
> never as grounds for stricter diagnostics. This one line we won't
> cross — not inferring arguments backward — the Little volume returns to concretely just once,
> in Part 8, and the Seasoned volume treats carefully in Part 5.
```
Proposed:
```
> inference and buy scale and "silence" (no false positives)** instead. Rigor doesn't run the
> whole program from an entry point either, and by default it doesn't infer arguments backward.
> It does offer an opt-in that collects types from call sites (an extra pass over them), but what
> it collects only adds precision — it's never grounds for stricter diagnostics. The line
> chibirigor won't cross — not inferring arguments backward — comes back concretely just once in
> the Little volume, in Part 8, and the Seasoned volume treats it carefully in Part 5.
```
Why: line 65 says "Inference isn't a separate pre-pass", and twenty lines later a "pre-pass"
appears with a different meaning. "This one line we won't cross" now follows the Rigor opt-in,
so "this" points to the wrong thing. The OSV inversion ("…the Little volume returns to") is
also a calque.

### I2. little/part1-literals-and-arithmetic.md:273-277 — `sig-gen` is defined after it's used, and "writes RBS out" appears twice
Current:
```
> Real Rigor has a two-layer structure — "know precisely inside, round coarsely at the RBS
> boundary." What `annotate` and the diagnostics show is the precise internal type; the
> rounding kicks in when `sig-gen` writes out RBS. `sig-gen` is the feature that writes RBS out
> from the inferred types; Part 8 covers it. So even though inference knows more, the
> outward-facing signature comes out coarse. That machinery is in appendix [a3-2](../appendix/a3-tooling.md).
```
Proposed:
```
> Real Rigor has a two-layer structure — "know precisely inside, round coarsely at the RBS
> boundary." `annotate` and the diagnostics show the precise internal type; the rounding happens
> only when `sig-gen` (the feature that writes RBS from inferred types, covered in Part 8) writes a
> signature out. So even though inference knows more, the outward-facing signature comes out
> coarse. That machinery is in appendix [a3-2](../appendix/a3-tooling.md).
```

### I3. little/part2-method-dispatch.md:228-232 — "the real thing" twice; the ③ bullet runs five lines
Current:
```
  spoonful in Part 8; the real resolution in the sequel). Because its table is close to the real
  thing, real Rigor reports `call.undefined-method` only when it can prove "this known class has
  no such method." It quietly falls back to `Dynamic` when it has lost the receiver's type.
```
Proposed:
```
  spoonful in Part 8; the real resolution in the sequel). With a near-complete table, real Rigor
  reports `call.undefined-method` only when it can prove "this known class has no such method,"
  and quietly falls back to `Dynamic` when it has lost the receiver's type.
```

### I4. little/part5-narrowing.md:148-151 — "belongs to this book's minimal version"
Current:
```
untouched). We don't narrow `Dynamic` either (something whose type is unknown, we let through
unknown). That last choice belongs to this book's minimal version: real Rigor does narrow
`untyped` to `String` in the branch where `is_a?(String)` holds — inside that branch the value
really is a `String` (or a subclass of it), so narrowing it can't produce a false positive.
```
Proposed:
```
untouched). We don't narrow `Dynamic` either (something whose type is unknown, we let through
unknown). That last choice is a simplification of this book's minimal version. Real Rigor does
narrow `untyped` to `String` where `is_a?(String)` holds: inside that branch the value really is a
`String` (or a subclass), so narrowing can't produce a false positive.
```

### I5. little/part5-narrowing.md:182-186 — "is, for now, outside that report"
Current:
```
`flow.unreachable-clause` at info level. The dead branch that `is_a?` produces is, for now, outside that
report.
```
Proposed:
```
`flow.unreachable-clause` at info level. The dead branch that `is_a?` produces isn't reported, for
now.
```

### I6. little/part5-narrowing.md:262-267 — "but not implemented yet" wedged mid-sentence; "It" points to the wrong thing
Current:
```
- **the Union size budget:** the last chapter's `union` helper only drops duplicates. Real Rigor's
  specification sets a budget (`union_size`): when a Union's member count exceeds the cap, it is
  **widened** to a Union of each member's nominal class (`Integer`, `String`, …) — but this is not
  implemented yet. It is the same idea as constant
  folding's "round if it's too big" — another expression of the design principle that "types, too,
  *have a budget*."
```
Proposed:
```
- **the Union size budget:** the last chapter's `union` helper only drops duplicates. Real Rigor's
  specification sets a budget (`union_size`): when a Union's member count exceeds the cap, it is
  **widened** to a Union of each member's nominal class (`Integer`, `String`, …). It isn't
  implemented yet, but the idea is the same as constant folding's "round if it's too big" —
  another expression of the design principle that "types, too, *have a budget*."
```

### I7. little/part6-hash-and-tuple.md:121 — "What differs" lost its comparison
Current:
```
chibirigor makes this **a fit.** What differs is the *motive and scope*. It isn't handling
```
Proposed:
```
chibirigor, too, makes this **a fit.** What differs from 『しくみ』 is the *motive and scope*. It isn't handling
```
Why: before the sync, "Rigor, too" tied this back to 『しくみ』 in the previous paragraph. Without
"too" and a named comparison, "What differs" has nothing to differ from.

### I8. little/part6-hash-and-tuple.md:200 — a summary-table cell that has grown into a paragraph
Current:
```
| ③ Rigor's implementation problem | This book makes the expectation open (at least): allow extras, blame only what's missing = apply width subtyping to a *dynamic hash* and avoid false positives. Real Rigor holds open/closed as a carrier policy, and a record written in RBS is closed, so extras are flagged |
```
Proposed:
```
| ③ Rigor's implementation problem | This book reads every expectation as open (at least): allow extras, blame only what's missing = width subtyping on a *dynamic hash*, no false positives. Real Rigor makes open/closed a carrier policy: an RBS record is closed and flags extras |
```

### I9. little/part6-hash-and-tuple.md:147-151 — the ③ bullet: "so nothing is said"
Current:
```
  as `{ name: String }` is read as a closed shape, and extra known keys are flagged. An undeclared
  options hash has no expected type to begin with, so nothing is said. chibirigor reads every
  expected shape as open, even where one is declared — a simplification that doesn't tell the
  declared case from the undeclared one.
```
Proposed:
```
  as `{ name: String }` is read as a closed shape, and extra known keys are flagged. An undeclared
  options hash has no expected type to begin with, so there's nothing to flag. chibirigor reads
  every expected shape as open, declared or not — a simplification.
```

### I10. "passes through gradually" / "lets it through gradually" — calque (漸進的に), wrong in English
Locations: little/part7-accepts-and-trinary.md:114-115; seasoned/part4-recursive-types.md:227,
236, 237 (the last two are covered by E8).
In English "gradually" means "slowly, little by little", so the phrase misleads. Say *gradual*
as the mode instead.

little/part7:113-115 current:
```
- **③ In Rigor:** this book's `accepts` stays quiet by answering `:maybe` where it doesn't know.
  This connects directly to "don't frighten" in the next section. Real Rigor lets it through
  gradually as `:yes`, but it stays quiet just the same.
```
Proposed:
```
- **③ In Rigor:** this book's `accepts` stays quiet by answering `:maybe` where it doesn't know.
  This connects directly to "don't frighten" in the next section. Real Rigor answers `:yes`
  instead — gradual typing's way of letting it through — and stays just as quiet.
```
seasoned/part4:224-227 current:
```
> from above, so passes come out lenient. `bound` defaults to `untyped` (`Dynamic[Top]`), and real
> Rigor answers `:yes` with `Dynamic` on either side, so an `App` registered without an upper bound
> passes through gradually.
```
Proposed:
```
> from above, so it errs toward passing. `bound` defaults to `untyped` (`Dynamic[Top]`), and real
> Rigor answers `:yes` with `Dynamic` on either side, so an `App` registered without an upper bound
> is simply let through, the gradual way.
```
("passes come out lenient" is also a calque.)

### I11. little/part9-gradual-philosophy.md:59-60 — "is the premise" (前提)
Current:
```
2. **A hash's unknown key is asserted to be `nil`** (Part 6). Reading the hash as open is the
   premise. Reject by exact match, as 『しくみ』 does, and working options hashes go bright red.
```
Proposed:
```
2. **A hash's unknown key is asserted to be `nil`** (Part 6), which rests on reading the hash as
   open. Reject by exact match, as 『しくみ』 does, and working options hashes go bright red.
```

### I12. little/part9-gradual-philosophy.md:68-70 — "surplus keys", where every other chapter says "extra keys"
Current:
```
That said, real Rigor draws the line a little differently for 2 and 4. In a record written in
RBS, it flags surplus keys (Part 6) — allowing surplus keys is not a hole in itself, just a
different policy. And it narrows even `untyped` under a trusted guard like `is_a?`
```
Proposed:
```
That said, real Rigor draws the line a little differently for 2 and 4. In a record written in
RBS, it flags extra keys (Part 6) — allowing extra keys isn't a hole in itself, just a different
policy. And it narrows even `untyped` under a trusted guard like `is_a?`
```

### I13. seasoned/part3-generics-and-substitution.md:184-191 — a semicolon chain with a gapped verb; hard to parse
Current:
```
For Rigor, erasure has one more meaning — the operation of **conservatively returning the rich
internal type to RBS.** `HashShape` returns to an RBS record or `Hash[K,V]`; a kind of literal RBS can't
spell, or an over-budget literal union in a hash's keys or values, to its base class; `Dynamic[T]` to `untyped`. Literals RBS
*can* spell — integers, strings, symbols and the like — stay literals. **It may get wider, but never narrower** (a sound
approximation). This is the substance of "Rigor is a superset of RBS, writable back to RBS at any
time." That holds for covariant positions such as value types, though. Widening a type in an
argument position makes the function type a *stronger* promise instead (Part 2's contravariance) —
which is also part of why `sig-gen` writes parameters as `untyped` by default.
```
Proposed:
```
For Rigor, erasure has one more meaning — the operation of **conservatively returning the rich
internal type to RBS**:

- `HashShape` returns to an RBS record or `Hash[K,V]`.
- A kind of literal RBS can't spell, or an over-budget literal union in a hash's keys or values,
  returns to its base class. Literals RBS *can* spell (integers, strings, symbols and the like)
  stay literals.
- `Dynamic[T]` returns to `untyped`.

**It may get wider, but never narrower** (a sound approximation). This is the substance of "Rigor
is a superset of RBS, writable back to RBS at any time" — for covariant positions such as value
types. In an argument position, widening a type makes the function type a *stronger* promise
instead (Part 2's contravariance), which is also part of why `sig-gen` writes parameters as
`untyped` by default.
```
Why: the reader has to carry "returns to" across two semicolons. "…though." is a weak hinge
for an important caveat.

### I14. seasoned/part4-recursive-types.md:169-180 — overloaded paragraph that says "exact stop / fuel cuts off" twice
Current:
```
What stops reduction is mainly **cycle detection** on the reduction stack. If reduction meets the
same `(URI, arguments)` application again partway through, it returns the `App` being unfolded
as-is and ties the knot (ADR-20 D4). Like μ-folding, this is an *exact* stop on the side that
*builds* the type — a different thing from `seen`, which decides a relation between two types by
coinduction. Rigor's acceptance check defers to `bound` (see below), so it has nothing that plays
the role of `seen`. Any unfolding that still doesn't stop is cut off, approximately, by the outer
safety net, the **fuel budget.** To the same problem of "how to stop the unfolding of recursion,"
『しくみ』/TAPL answer *theoretically* (correct equivalence test by coinduction), Rigor *in
engineering*: building stops exactly at a cycle, and whatever still won't stop is cut off by fuel.
**The sum-up of this "coinduction
vs. budget" engineering of termination is gathered in Seasoned Part 7, which treats soundness** (read
together with gradual's settling).
```
Proposed:
```
What stops reduction is mainly **cycle detection** on the reduction stack. If reduction meets the
same `(URI, arguments)` application again partway through, it returns the `App` being unfolded
as-is and ties the knot (ADR-20 D4). Like μ-folding, this is an *exact* stop on the side that
*builds* the type. It is a different thing from `seen`, which decides a relation between two types
by coinduction; Rigor's acceptance check defers to `bound` (see below), so nothing in it plays the
role of `seen`. Any unfolding that still doesn't stop is cut off, approximately, by the outer
safety net: the **fuel budget.**

So to the same problem — "how do we stop the unfolding of recursion?" — 『しくみ』/TAPL answer
*theoretically* (a correct equivalence test by coinduction), and Rigor *in engineering* (an exact
stop at a cycle, with fuel behind it). **Seasoned Part 7, which treats soundness, gathers this
engineering of termination in one place** (read together with gradual's settling).
```
Why: the last sentence repeats the third, and "the sum-up of this … is gathered in" is a calque.
The label "coinduction vs. budget" no longer covers what the paragraph describes.

### I15. seasoned/part4-recursive-types.md:208-210 — the same caveat a third time in one chapter
Current:
```
> for HKT type-argument selection too. Reduction can unfold infinitely, so it's cut off on the safe
> side: **cycle detection** stops it exactly, and **fuel (default 64)** cuts it off as the outer
> safety net.
```
Proposed:
```
> for HKT type-argument selection too. And since reduction could unfold forever, it's stopped on
> the safe side as described above — cycle detection first, **fuel (default 64)** as the backstop.
```
Why: §4-5's body, this note, and the summary row all spell out the same two-tier stop. The note
only needs to point back (it keeps the "default 64" detail, which appears nowhere else in the
chapter).

### I16. seasoned/part4-recursive-types.md:213-214 — opening of the TIP is a calque; subject–verb agreement
Current:
```
> **Reference note.** When a type that couldn't be reduced and stayed an `App` is put to the test,
> what backs its correctness is the following framework. The primary grounds for HKT (a type that takes a type and returns a type) is
> not ch. 20/21 on recursive types, but **TAPL ch. 29 "Type operators and kinding."** The
```
Proposed:
```
> **Reference note.** What backs an `App` that stays unreduced is not ch. 20/21 on recursive types
> but **TAPL ch. 29 "Type operators and kinding"** — the primary grounds for HKT (a type that takes
> a type and returns a type). The
```
Why: "is put to the test, what backs its correctness is the following framework" is wordy and
stiff, and "The primary grounds … is" doesn't agree in number.

### I17. seasoned/part4-recursive-types.md:280-283 — "consolidate here" in a *next chapter* teaser
Current:
```
**Next chapter (Part 5):** we extend the Little volume's naive type inference to Rigor's real **type
inference.** We treat unification, which collects argument types as constraints from how the body
uses them and solves them, and consolidate here the comparison with TypeProf, which collects types
from the callers.
```
Proposed:
```
**Next chapter (Part 5):** we extend the Little volume's naive type inference to Rigor's real **type
inference.** We treat unification — collecting constraints on argument types from how the body uses
them, then solving them — and gather in one place the comparison with TypeProf, which collects types
from the callers.
```
Why: "here" in a Part 4 paragraph points at Part 4.

### I18. seasoned/part5-real-inference.md:224-227 — OSV inversion; the sentence runs on
Current:
```
> merges the calls into the parameter's type. An unsolvable argument it drops to `Dynamic[Top]` (= `untyped`) and
> cuts off, keeping cost down with hardcoded cutoff caps (Part 6's cache works too). A configurable
> inference budget is being designed in ADR-41 and isn't wired in yet. Its goal is to run on the
> whole codebase every push, so it prioritizes scale above all.
```
Proposed:
```
> merges the calls into the parameter's type. It drops an unsolvable argument to `Dynamic[Top]`
> (= `untyped`) and moves on, keeping cost down with hard-coded caps (Part 6's cache helps too); a
> configurable inference budget is designed in ADR-41 but not yet wired in. Its goal is to run on
> the whole codebase every push, so it prioritizes scale above all.
```

### I19. seasoned/part5-real-inference.md:242 — "(Chapter 11)" is ambiguous
Current:
```
Note that `rigor sig-gen` (Chapter 11) does the same job as TypeProf (generate RBS from zero
```
Proposed:
```
Note that `rigor sig-gen` (the Rigor handbook, ch. 11) does the same job as TypeProf (generate RBS from zero
```
Why: this book has no Chapter 11. It means `rigor/docs/handbook/11-sig-gen.md`. Line 198 already
uses the "(details: the Rigor handbook, …)" form. The paragraph wasn't changed in this sync,
but the new §5-4b right after it leans on it.

### I20. seasoned/part5-real-inference.md:256-260 — repeats 242-245 one section later
Both paragraphs explain that sig-gen leaves parameters `untyped` by default and that a human
decides. Trim the second so it builds on the first:

Current:
```
`rigor sig-gen --params=observed` also gathers argument types from observed calls, but it stands in a
different place. It writes the gathered types into the RBS as a *proposal* for the parameter types;
adopt it and it becomes a contract, and diagnostics at the boundary get stricter. That's why the
default is `--params=untyped`, and whether to pin the arguments down is for a human to review and
decide (ADR-5, ADR-14).
```
Proposed:
```
`rigor sig-gen --params=observed` also gathers argument types from observed calls, but for a
different purpose: it writes them into the RBS as a *proposal*. Adopted, the proposal becomes a
contract and tightens diagnostics at the boundary — which is why, as §5-4a said, the default is
`--params=untyped` and a human decides (ADR-5, ADR-14).
```

### I21. seasoned/part5-real-inference.md:262 — "There's just one hard part here." (ただ一つ難所がある)
Current:
```
There's just one hard part here. **The gathered type is a floor, not a ceiling** (in the subtype order, a
```
Proposed:
```
The catch: **the gathered type is a floor, not a ceiling** (in the subtype order, a
```

### I22. seasoned/part5-real-inference.md:264-267 — "gathered type" three times in two lines
Current:
```
`parameter_inference:` gathers are used only to raise the precision of the body's inference, never
as grounds to reject a call at the argument boundary. Wherever the gathered type flows, too, Rigor emits no negative diagnostic that leans on
the gathered type (an undefined method, a possible nil, and so on). Add precision, but don't add
```
Proposed:
```
`parameter_inference:` gathers are used only to raise the precision of the body's inference, never
as grounds to reject a call at the argument boundary. Nor, wherever that type flows, does Rigor emit
a negative diagnostic that leans on it (an undefined method, a possible `nil`, and so on). Add precision, but don't add
```

### I23. seasoned/part5-real-inference.md:284-286 — repeats §5-4b's point one section later
Current:
```
Real Rigor doesn't rely on that alone: it explicitly suppresses negative diagnostics derived from
inference. In Rigor's own development, too, opening the opt-in gate at first let new diagnostics leak
out, and guards were added to hold them back (ADR-67 WD6d).
```
Proposed:
```
Real Rigor doesn't rely on that alone; as §5-4b showed, it explicitly suppresses negative
diagnostics derived from inference. That wasn't free: when the opt-in gate was first opened, new
diagnostics leaked out until guards were added (ADR-67 WD6d).
```
Why: tie it back instead of restating it, and lead with the new information (the leak story).
Together with the 5-7 summary bullet, the point is already made three times.

### I24. seasoned/part5-real-inference.md:294-297 — "Put as an analogy, it would be a carrier like…"
Current:
```
  - That said, inference that gathers capabilities from how the body uses a value (Road A itself)
    is on hold in real Rigor too (ADR-67 WD2), because although named capability roles exist, there's no carrier yet to hold an
    anonymous gathered capability set (which methods a value responds to). Put as an analogy, it
    would be a carrier like a method version of the Little volume's Part 6 `HashShape`.
```
Proposed:
```
  - That said, Road A itself — gathering capabilities from how the body uses a value — is on hold
    in real Rigor too (ADR-67 WD2). Named capability roles exist, but there's no carrier yet for an
    anonymous gathered set of capabilities (which methods a value responds to); think of it as a
    method-level counterpart of Little Part 6's `HashShape`.
```

### I25. seasoned/part6-fact-store.md:190 — "the union type of `nil` or `Integer`"
Current:
```
scope (ADR-56). In the example above, it keeps `x` as the union type of `nil` or `Integer` instead
```
Proposed:
```
scope (ADR-56). In the example above, it keeps `x` as `nil | Integer` instead
```

### I26. seasoned/part6-fact-store.md:262-265 — stacked "and … and …" in the next-chapter teaser
Current:
```
chapter we take those head-on — **soundness and normalization**, and the four places chibirigor is
*deliberately* unsound, and how real Rigor differs. We gather gradual's two disciplines (consistency and guarantee) and
"coinduction vs. budget" in one place, pairing with Seasoned Part 1's bidirectional map.
```
Proposed:
```
chapter we take those head-on — **soundness and normalization**, and the four places chibirigor is
*deliberately* unsound (and where real Rigor differs). We gather gradual's two disciplines
(consistency and guarantee) and the engineering of termination in one place, pairing with Seasoned
Part 1's bidirectional map.
```
("coinduction vs. budget" no longer names what §7-6 covers. See I28.)

### I27. seasoned/part7-soundness.md:97-98 — "too" is ambiguous
Current:
```
Note that ④'s "doesn't narrow `Dynamic`" is a gap in chibirigor. Under a trusted `is_a?` guard, real
Rigor narrows `Dynamic` to a concrete type too (ADR-83).
```
Proposed:
```
Note that ④'s "doesn't narrow `Dynamic`" is chibirigor's own simplification: under a trusted
`is_a?` guard, real Rigor does narrow `Dynamic` to a concrete type (ADR-83).
```
Why: "too" can be read as "Rigor also narrows to a concrete type, like chibirigor". "A gap"
also collides with the chapter's "holes".

### I28. seasoned/part7-soundness.md:214-215 vs 243, 245-246, 270-272 — the framing changed in the intro, not elsewhere
The intro (214-215) dropped "self-similar shape … at three sizes" and switched to "stop
correctly / stop when dangerous". The subsection heading at 243 ("a larger self-similar shape"),
line 246 ("a large self-similar shape of fuel") and the summary (272, "a self-similar shape at
varying scales") still use the old framing. Either put a clause back in the intro or drop it
from the summary. The summary bullet also now has two parentheticals in a row:

Current (270-272):
```
- Instead of normalization, it keeps termination by **engineering** — coinduction and HKT cycle
  detection (stop correctly), HKT fuel (ADR-20, implemented) and the inference budget (ADR-41,
  designed, unimplemented) (stop when dangerous), a self-similar shape at varying scales.
```
Proposed:
```
- Instead of normalization, it keeps termination by **engineering**: coinduction and HKT cycle
  detection *stop correctly*; HKT fuel (ADR-20, implemented) and the inference budget (ADR-41,
  designed, unimplemented) *stop when it gets dangerous* — the same answer at varying scales.
```
And optionally restore the thread in the intro (214-215), current:
```
*engineering.* The termination tools that appeared up to this chapter line up as a contrast between
"stop it correctly" and "stop when it gets dangerous."
```
Proposed:
```
*engineering.* The termination tools that appeared up to this chapter line up as a contrast between
"stop it correctly" and "stop when it gets dangerous," repeated at growing scales.
```

### I29. seasoned/part7-soundness.md:226-227 — still introduces fuel as "the alternative", then reveals cycle detection
Current:
```
Rigor doesn't implement recursive types directly with μ + coinduction; it makes a lightweight HKT
(`Type::App`) + a **fuel budget** the alternative (carried over from Seasoned Part 4 §4-5). If reduction
```
Proposed:
```
Rigor doesn't implement recursive types directly with μ + coinduction; its alternative is a
lightweight HKT (`Type::App`), stopped mainly by cycle detection with a **fuel budget** behind it
(carried over from Seasoned Part 4 §4-5). If reduction
```
Why: the subsection heading (224) now says "cycle detection and fuel", but its first sentence
still frames the mechanism as HKT + fuel. (§4-5's opening, 159-162, is fine; its title was
updated.)

### I30. seasoned/part7-soundness.md:250-252 — comma-joined parallel lists
Current:
```
by the *engineering* of "**cut off the analysis with a budget.**" Coinduction and cycle detection (stop correctly),
fuel and budget (stop when dangerous) answer the same question of "how to stop the unfolding" at
varying scales. That a practical checker can choose "stop when dangerous" was because of gradual's
```
Proposed:
```
by the *engineering* of "**cut off the analysis with a budget.**" Coinduction and cycle detection stop
correctly; fuel and budget stop when it gets dangerous. All four answer the same question — "how do
we stop the unfolding?" — at varying scales. That a practical checker can choose "stop when dangerous" was because of gradual's
```

### I31. appendix/a1-special-types.md:52-54 — "is something … keeps" and "That X can … is because" calques
Current:
```
- *Where* and *why* the type was lost is something real Rigor keeps in a record outside the type.
  That Little Part 9's `rigor check --explain` can map every fail-soft site is because inference
  records each of those moments.
```
Proposed:
```
- Real Rigor keeps *where* and *why* the type was lost in a record outside the type: inference
  records each of those moments, which is how Little Part 9's `rigor check --explain` can map every
  fail-soft site.
```

### I32. appendix/a1-special-types.md:206-213 — long; "narrowing narrowed"; ends on an afterthought
Current:
```
This is a miniature that borrows the idea of real Rigor's ADR-47 (`flow.unreachable-clause`). The
real thing targets the clauses of `case`/`when` and `case`/`in`: it judges a clause where narrowing
narrowed the subject to `bot` as dead, and has an FP envelope (a fence of exclusion conditions to
prevent false positives) excluding loops, blocks, and gradual. For `if`/`unless`, it reports only
branches decided by a literal, like `if false`, as `flow.unreachable-branch`; dead branches that
arise from `is_a?` are not reported at present. Both diagnostics are on by default. chibirigor
minimizes the same idea down to three points — "leaf-class disjointness," "closed types only," and
"opt-in" — and applies it to `is_a?` in an `if`.
```
Proposed:
```
This miniature borrows the idea of real Rigor's ADR-47 (`flow.unreachable-clause`). The real thing
targets the clauses of `case`/`when` and `case`/`in`: a clause whose subject narrowing has reduced
to `bot` is judged dead, inside an FP envelope (a fence of exclusion conditions against false
positives) that leaves out loops, blocks, and gradual types. For `if`/`unless`, it reports only
branches decided by a literal, like `if false`, as `flow.unreachable-branch`; a dead branch arising
from `is_a?` isn't reported at present. Both diagnostics are on by default. chibirigor shrinks the
same idea to three points — "leaf-class disjointness," "closed types only," and "opt-in" — and
applies it to `is_a?` in an `if`.
```

### I33. appendix/a2-narrowing-patterns.md:116-119 — OSV inversion
Current:
```
Whether a block is "immediately invoked" or "called later" Rigor looks up in a built-in catalog of
(class, method) pairs. It does not infer this from RBS signatures; the plan is to let extension
annotations override the catalog in the future. When it can't decide, it treats it as an
escape, adopting the **when-in-doubt-erase** (fall to the looser side) policy.
```
Proposed:
```
Rigor decides whether a block is "immediately invoked" or "called later" by looking it up in a
built-in catalog of (class, method) pairs. It doesn't infer this from RBS signatures, though the
plan is to let extension annotations override the catalog. When it can't decide, it treats the
block as escaping — the **when-in-doubt-erase** policy (fall to the looser side).
```

### I34. appendix/a2-narrowing-patterns.md:108-109 — "runs at *any timing*"
Current:
```
The block passed to `Thread.new` runs at *any timing.* That block rewrites `x`, so we can't rule out
that `x` has become a different type at any later point.
```
Proposed:
```
The block passed to `Thread.new` can run at *any time.* It rewrites `x`, so from here on we can't
rule out that `x` has become a different type.
```

### I35. appendix/a3-tooling.md:59-62 — "each moment it falls soft"
Current:
```
This listing works because inference itself **records each moment it falls soft.** Every time it
can't look up a type and falls to `Dynamic[Top]`, inference leaves that site behind as one record.
`--explain` merely lays those records back out as `:info` diagnostics — the same idea as
`rigor trace`, which we'll see later, laying out inference events.
```
Proposed:
```
This listing works because inference itself **records every fail-soft as it happens.** Each time it
can't look up a type and falls back to `Dynamic[Top]`, it leaves a record of that site.
`--explain` simply lays those records out as `:info` diagnostics — the same idea as `rigor trace`
(§a3-3) laying out inference events.
```
Why: "falls soft" isn't English. The book's term is "fail-soft" (noun/adjective) or "fall back".

### I36. appendix/a3-tooling.md:103-105 — "doesn't record again a call on…"
Current:
```
`lib/chibirigor/dispatch.rb`). The other is line 2. Real Rigor doesn't record again a call on a
value that has already become `untyped`, so line 2 doesn't appear in its listing; chibirigor
deliberately shows both to make the propagation visible. The idea of **mapping the silence** is the
```
Proposed:
```
`lib/chibirigor/dispatch.rb`). The other is line 2. Real Rigor adds no second record for a call on a
value that is already `untyped`, so its listing would stop at line 1; chibirigor deliberately shows
both to make the propagation visible. The idea of **mapping the silence** is the
```

### I37. appendix/a3-tooling.md:150-151 — garden-path sentence
Current:
```
> sprinkling them doesn't turn diagnostics red). The type editor hover shows is the result of the
> same inference, too.
```
Proposed:
```
> sprinkling them doesn't turn diagnostics red). Editor hover shows the result of the same
> inference.
```

### I38. appendix/a5-other-languages.md:81-83 — "even where one exists" (one what?)
Current:
```
  known keys. An undeclared options hash has no expected type, so Rigor stays quiet. Receiving
  every expected shape uniformly as an open "at least," even where one exists, is chibirigor's
  simplification.
```
Proposed:
```
  known keys. An undeclared options hash has no expected type, so Rigor stays quiet. Reading every
  expected shape as an open "at least," declared or not, is chibirigor's simplification.
```

### I39. appendix/a2-narrowing-patterns.md:121-124 — seam with the new Seasoned P6 §6-4 paragraph (for the fidelity lens)
Current:
```
> Also, when a block **rewrites an outer local** (`x = nil; [1,2,3].each { |i| x = i }`), its
> `captured_local` fact is invalidated too — if the block could reassign `x`, the narrowing fact is
> in jeopardy. This is treated in the main line of closure capture in Seasoned Part 6.
```
Seasoned P6:188-192 now says that for a non-escaping block like `each`, real Rigor re-evaluates
the block body, joins the result and writes it back (`x` becomes `nil | Integer`) instead of
throwing the fact away. a2 is a real-Rigor appendix, and it still describes plain invalidation
for exactly this `each` example. I am not judging correctness here. Editorially, the two passages
now disagree, so either label a2's note as "in the minimal model" or add "(real Rigor goes one
step further and writes back the joined type, ADR-56)". Hand this to the fidelity lens.

---

## NIT

- **N1. seasoned/part5-real-inference.md:11-13** — "answer … answer". Current: `> used in the body — the frontier of inference, which 『しくみ』 left without an answer, saying "we` /
  `> don't know the correct answer."` → Proposed: `> used in the body — the frontier of inference, where 『しくみ』 stops with "we don't know the` /
  `> correct answer."`
- **N2. seasoned/part5-real-inference.md:213-214 and :234** — the "Rigor handbook positions it" hedge
  appears in the prose and again in the table cell. Drop it from the table: `| Scale goal | small files, prototypes (as the Rigor handbook positions it) |` → `| Scale goal | small files, prototypes |`.
- **N3. seasoned/part6-fact-store.md:83-84** — Current: `` > to." `dynamic_origin` alone is the odd one out, holding not a target scope but facts about dynamic `` / `> provenance.` → Proposed: `` > to." `dynamic_origin` alone is the odd one out: its facts are about dynamic provenance, not about `` / `> a target scope.`
- **N4. seasoned/part6-fact-store.md:98-100** — Current: `up as the sixth in the real spec too, but its role is separate). Tracking "*where* \`untyped\` was` / `born" itself is, in real Rigor, the job of an advisory table outside the FactStore` → Proposed: `up as the sixth in the real spec too, but its role is separate). In real Rigor, tracking "*where*` / `` `untyped` was born" is the job of a separate advisory table outside the FactStore `` (the remainder of the footnote stays as is).
- **N5. seasoned/part8-toward-rigor.md:124-125 vs :132-133** — ADR-4 is glossed "how the type inference engine works" in one bullet and "introducing the FactStore" in another. Use one gloss that covers both: `ADR-4 (the inference engine, which introduces the FactStore)`.
- **N6. seasoned/part8-toward-rigor.md:128** — `← Seasoned Part 4 (fuel), Part 7 (budget)` → `← Seasoned Part 4 (cycle detection, fuel), Part 7 (budget)`, so it matches the updated Part 4/7.
  (Same in appendix/a4-bibliography.md:126's third column, `Seasoned Part 4 (fuel)`.)
- **N7. seasoned/part8-toward-rigor.md:168** — `` `PRISM_DISPATCH` table. It looks up the handling method from the node kind: the `case` of the `` → `` `PRISM_DISPATCH` table. It maps each node kind to the method that handles it: the `case` of the ``.
- **N8. little/part8-rbs-and-signatures.md:295-296** — passive. Current: `> types into the proposed RBS as parameter types. The default is \`--params=untyped\`, and whether` / `> to narrow a parameter is decided by the human who adopts that proposal.` → Proposed: `> types into the proposed RBS as parameter types. The default is \`--params=untyped\`, and the human` / `> who adopts that proposal decides whether to narrow a parameter.`
- **N9. seasoned/part4-recursive-types.md:254-255** — `The HKT version is a separate implementation with a URI` / `reference + fuel.` → `The HKT version is a separate implementation with a URI` / `reference, cycle detection and fuel.` (matches the updated summary row).
- **N10. little/part1-literals-and-arithmetic.md:220** — fronted object. `*Don't stop, don't frighten.* How far real Rigor goes in reporting, we'll see in Part 2.` → `*Don't stop, don't frighten.* Part 2 shows how far real Rigor goes in reporting.` (Good promise→payoff: Part 2:230-232 delivers on it.)
- **N11. little/part9-gradual-philosophy.md:16 / :42-48** — the heading "`untyped` is contagious" now heads a section whose second paragraph says real Rigor *doesn't* make it contagious (`1 | Dynamic[Top]`). That's acceptable, since the section builds chibirigor. If you want a softer seam, add "in chibirigor" to the first sentence after the code: `` `untyped` honestly. This is the most conservative form, and the one chibirigor chose. `` is already fine. No change is needed beyond being aware of it.

---

## Clean / no action

- glossary.md (HM / erasure entries), seasoned/examples/README.md, little/part4-union.md's new
  `nil` paragraph, seasoned/part1's `param:` note, seasoned/part2:78-91, seasoned/part8:167-170's
  new closing beat ("opened out into a table so it stays readable as the branches multiply") —
  all read well and fit their surroundings.
- The new "Real Rigor draws this line differently" openers (Little P5:182, P9:44) are good
  signposts. Keep that phrasing as the pattern for any future syncs.
