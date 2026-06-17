# English edition review — FP / type-theory researcher lens

Reviewer seat: outside functional-programming / type-theory researcher, non-native English reader.
Scope read in full: Seasoned `part1`–`part8`, appendix `a1-special-types.md`, `glossary.md`,
`appendix/a4-bibliography.md`, Little `part4`–`part7`, `part9`, plus the Seasoned/Little READMEs.
High-risk claims (rank-N decidability, Kfoury–Wells / Wells, fuel default, all TAPL chapter
numbers, named results) were cross-checked against `book/v2/ja` (the source of truth).

## Headline verdict

**The formal content is sound.** I found no type-theory ERROR. The bidirectional rules
(Var-Synth, Sub), S-Arrow variance (`A' <: A`, `B <: B'`), width/depth subtyping direction
(more keys = subtype), capture-avoiding substitution, α-equivalence, equi/iso-recursive split,
the reduced-coinduction caveat, progress + preservation, and gradual consistency (`~` symmetric,
non-transitive) are all stated correctly. Every citation I checked is accurate:

- TAPL chapter numbers: 8 (§8.3 safety), 9 (STLC), 11 (§11.8 records / §11.7 tuples / §11.10
  variants), 12 (normalization), 15/16 (subtyping / metatheory), 20/21 (recursive types /
  metatheory), 22 (type reconstruction), 23 (System F), 26 (bounded quantification), 29 (type
  operators and kinding). All correct.
- Named results: Pierce & Turner "Local Type Inference" (2000); Dunfield & Krishnaswami
  "Bidirectional Typing" (2021); Siek & Taha (2006); the `~` symbol attributed to Siek; Milner
  "well-typed programs cannot go wrong"; Liskov LSP; **Kfoury–Wells 1994** (rank-3+ undecidable),
  **Wells 1994/1999** (unrestricted System F typeability undecidable, rank-independent). All
  correct and correctly distinguished.
- Decidability ladder (part5 §5-4): "rank 1 decidable, rank 2 decidable, rank 3+ undecidable" is
  the standard result and matches the JA verbatim.
- fuel default = 64 (part4 §4-5 footnote, part7 §7-6): matches JA ("既定64").

The careful caveats are a real strength: the part4 `[!NOTE]` flagging the sketch's `seen`-check
as a *sound reduced version, weaker than TAPL ch. 21's greatest-fixed-point coinduction*; the
part5 `unification.rb` comment that the sketch *omits the occurs-check* and explaining why it is
nonetheless safe in a TVar/TCon-only world but essential in real HM; the part2 §2-4 note that
Rigor's current impl treats Nominal args uniformly covariantly (declaration-site variance
designed/unimplemented). None of these are defects — they are correct reductions, honestly
labelled.

The two intentional reductions I specifically checked are *not* over-sharpened relative to the JA:
the "deliberately unsound / abandons progress" framing in part7 §7-4, and the gradual guarantee
in §7-5, both track the source faithfully.

---

## Findings

### MISLEADING

**M1 — glossary.md:97 — "type reconstruction / HM" defined as "inference that recovers types
from annotations."**
> `type reconstruction / HM` 〔Seasoned P5〕 — inference that recovers types from
> annotations. TAPL ch. 22.

Type reconstruction / Hindley–Milner recovers types **in the absence of annotations**, from the
*terms / how variables are used* — not "from annotations." (TAPL ch. 22 is literally about
reconstructing the missing type annotations.) The phrasing inverts the defining property. It is
also slightly in tension with part5 itself, whose whole thesis is filling in argument types that
were *not* annotated.

Note for the author: this is a **faithful translation of the JA** (`glossary.md:55`:
「注釈から型を復元する推論」), so under the source-of-truth rule it is not an EN transcreation
defect — I flag it because it is a genuine type-theory imprecision that lives in *both* editions
and the author may wish to correct at the source. Suggested fix (both ja + en): "inference that
recovers types **without annotations**, from how terms are used" / 「注釈なしに、項の使われ方
から型を復元する推論」.

**M2 — appendix/a4-bibliography.md:74 — Little Part 8 mapped to "TAPL ch. 23, 22".**
> | 8 | RBS and type signatures | … | ch. 23, 22 | partial match (distant relative of type
> substitution → return-type synthesis; …) |

Little Part 8 is return-type *synthesis* from a body plus RBS plumbing; pointing it at ch. 22
(type reconstruction) and ch. 23 (System F universal types) is a stretch — those are the natural
homes of Seasoned Part 5 and Part 3 respectively, where the table already (correctly) places them.
Defensible as "distant relative," and the prose hedges with "partial match / distant relative,"
so MISLEADING not ERROR. Consider softening to a single "(distant relative)" pointer or dropping
the ch. numbers for the Little row to avoid implying Little P8 covers reconstruction / System F.

### READ-FEEL

**R1 — seasoned/part1-bidirectional-typing.md:118, 129 — "working code still go unfrightened" /
"working code goes unfrightened".**
"unfrightened" is an unusual coinage; a non-native reader stumbles on it (and it reads as
"not yet frightened" rather than "not frightened"). The JA is an ordinary passive
(「動くコードが脅かされない」). Suggest: "why does working code still **escape being flagged**" /
"working code **goes unflagged** not because …". (House term elsewhere is "never frighten working
code," so "goes unflagged / stays unflagged" keeps the register without the coinage.)

**R2 — seasoned/part1-bidirectional-typing.md:122 — "unsynthesizable syntax".**
Dense nominalization stacked next to "unbound variable." Readable in context, but "syntax that
can't be synthesized" or "a form with no synthesis rule" would land more easily for the target
audience. Minor.

**R3 — seasoned/part5-real-inference.md:296 — "adding no plumbing (minimal, budget-first)".**
"plumbing" as a noun for internal wiring is idiomatic to native engineers but opaque to a
non-native reader; appears again at a4-bibliography phrasing. Optional: "adding no extra
machinery."

**R4 — seasoned/part2-subtyping-and-variance.md:138 / part7 §7-6 — "settles to remember" /
"land on the safe side".**
Light calque feel ("settles to remember 'you may return less…'"). Comprehensible; flagged only
as a cluster of phrasings a non-native reader re-reads. Not worth a targeted edit unless doing a
read-feel pass.

---

## Things explicitly checked and found CORRECT (so they are not re-litigated later)

- part1 Sub rule and "checking = synthesize then `<:`" — correct; subsumption direction right.
- part2 S-Arrow: `A' <: A B <: B'` ⊢ `(A)->B <: (A')->B'` — correct contravariant arg / covariant
  return; the `subtype.rb` `subtype(tp, sp)` swap matches.
- part2 §2-4 mutable-container variance (cov read / contra write / invariant both) — correct, and
  honestly separated from Rigor's current uniform-covariant impl.
- part2 §2-6 algorithmic vs declarative subtyping (absorb refl/trans) — correct.
- part3 capture/shadowing handling and `fresh_name` α-conversion — correct; TAPL 23.7 erasure
  theorem vs Java type erasure distinction — correct.
- part4 equi vs iso-recursive, fold/unfold, coinduction `seen`, greatest-fixed-point, HKT grounds
  = ch. 29 (not 20/21) — correct and a nice precise call.
- part5 rank ladder + Kfoury–Wells / Wells split, occurs-check caveat, HM `let`-polymorphism left
  out and labelled — correct.
- part6 join-at-merge described as fact-lattice meet (intersection) vs type-lattice join — correct
  and carefully disambiguated (6-5).
- part7 progress vs preservation assignment of the four holes, "mostly keeps preservation by
  widening to untyped" — correct and faithful to JA.
- appendix a1 axis-A/axis-B decomposition of `any`/`unknown`/`Dynamic[Top]`, `void` top-like but
  return-position-only, `never`/`Bot` `Bot <: T` — all correct.
