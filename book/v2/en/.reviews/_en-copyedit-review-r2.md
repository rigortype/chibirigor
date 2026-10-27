# English copyedit review r2 — chibirigor v2/en

Second near-publication copyedit pass. **Scope:** the English text changed since the last
copyedit (`git diff b1e59ad..HEAD -- book/v2/en`, excluding `.reviews/` and `_handoff-state.md`),
plus the labels in `figures/svg/seasoned-6-1.svg`. Terms introduced in the diff were grepped across
the whole edition for spelling, hyphenation, and casing consistency.

Lens: grammar, US spelling, articles/prepositions, agreement, hyphenation and capitalization
consistency, doubled words, parallel structure, recurring-term consistency, broken Markdown, and
front-matter YAML. House typography (spaced em dash, en dash for ranges, `①②③`, quotes, `『しくみ』`,
code identifiers) is correct by definition and is **not** flagged. Code-block contents are not
touched. Technical content is not judged.

Severity: **ERROR** (grammatical defect a reader will trip on) · **FIX** (clear consistency or
correctness defect) · **NIT** (minor polish / latent risk).

Overall: the new prose is clean and the new vocabulary is consistent across the edition (see
"Terms checked" at the end). There are two grammatical errors, a handful of FIXes, and some NITs.
No broken Markdown or YAML. The SVG labels match the text figure.

---

## ERROR

### E1 — subject–verb agreement ("grounds … is")
`seasoned/part4-recursive-types.md:214` (TIP, "Reference note")

Current:
> The primary grounds for HKT (a type that takes a type and returns a type) is
> not ch. 20/21 on recursive types, but **TAPL ch. 29 …**

Corrected:
> The primary grounds for HKT (a type that takes a type and returns a type) are
> not ch. 20/21 on recursive types, but **TAPL ch. 29 …**

(Line 267 of the same file already says "HKT's grounds **are** TAPL ch. 29.")

### E2 — missing preposition ("runs the opposite direction")
`seasoned/part5-real-inference.md:210`

Current:
> TypeProf runs the opposite *direction*, raising argument types from the caller.

Corrected:
> TypeProf runs in the opposite *direction*, raising argument types from the caller.

---

## FIX

### F1 — missing comma after a fronted object clause
`appendix/a2-narrowing-patterns.md:116`

Current:
> Whether a block is "immediately invoked" or "called later" Rigor looks up in a built-in catalog of

Corrected:
> Whether a block is "immediately invoked" or "called later," Rigor looks up in a built-in catalog of

### F2 — "The other is line 2" has no predicate it can bear
`appendix/a3-tooling.md:103`

The sentence lists the second *difference*; "line 2" itself is not a difference. (The Japanese
says 「2行目の扱い」.)

Current:
> The other is line 2. Real Rigor doesn't record again a call on a
> value that has already become `untyped`, so line 2 doesn't appear in its listing;

Corrected:
> The other is how line 2 is handled. Real Rigor doesn't make a fresh record for a call on a
> value that has already become `untyped`, so line 2 doesn't appear in its listing;

("doesn't record again a call" is unidiomatic word order; the replacement keeps the meaning.)

### F3 — garden path ("The type editor hover shows")
`appendix/a3-tooling.md:150–151`

Reads first as "the type editor."

Current:
> The type editor hover shows is the result of the
> same inference, too.

Corrected:
> The type shown on editor hover is the result of the
> same inference, too.

### F4 — "What differs" lost its referent; "-ly" adverb hyphenated
`little/part6-hash-and-tuple.md:121–122`

The earlier text said "Rigor, *too*, makes …," which set up the comparison with 『しくみ』 (previous
paragraph). The revision dropped "too," so "What differs" no longer says from what. Separately, a
compound with an "-ly" adverb takes no hyphen in US style.

Current:
> chibirigor makes this **a fit.** What differs is the *motive and scope*. It isn't handling
> statically-written records for soundness.

Corrected:
> chibirigor, too, makes this **a fit.** What differs from 『しくみ』 is the *motive and scope*. It isn't handling
> statically written records for soundness.

### F5 — "surplus keys" vs. the edition's "extra keys"
`little/part9-gradual-philosophy.md:68–69`

Everywhere else (Little P6, Seasoned P2, a5, and line 59 of this chapter's own list) the term is
"extra keys."

Current:
> In a record written in
> RBS, it flags surplus keys (Part 6) — allowing surplus keys is not a hole in itself, just a

Corrected:
> In a record written in
> RBS, it flags extra keys (Part 6) — allowing extra keys is not a hole in itself, just a

### F6 — "union type of `nil` or `Integer`"
`seasoned/part6-fact-store.md:190`

A union is *of* its members joined with "and" (cf. part5:246, "the union type of the
actual-argument types").

Current:
> it keeps `x` as the union type of `nil` or `Integer` instead

Corrected:
> it keeps `x` as the union type of `nil` and `Integer` instead

### F7 — "This table is the chibirigor `accepts`"
`seasoned/part7-soundness.md:148` (NOTE)

A table can't *be* a method; it describes one.

Current:
> This table is the chibirigor `accepts` built in the Little volume.

Corrected:
> This table describes the chibirigor `accepts` built in the Little volume.

---

## NIT

### N1 — serial comma in the new "cycle detection" lists
The edition uses the serial comma (e.g. a1:205 "loops, blocks, and gradual"; part6:188 "…, joins
the types written, and writes them back"). The new three-item lists omit it.

- `seasoned/part4-recursive-types.md:157`
  Current: `## 4-5. A note: Rigor's alternative answer — HKT / `App`, cycle detection and fuel`
  Corrected: `## 4-5. A note: Rigor's alternative answer — HKT / `App`, cycle detection, and fuel`
- `seasoned/part4-recursive-types.md:276`
  Current: `"coinduction (`seen`)," "cycle detection" and "fuel budget" — and sort out`
  Corrected: `"coinduction (`seen`)," "cycle detection," and "fuel budget" — and sort out`
- `seasoned/part3-generics-and-substitution.md:187`
  Current: `integers, strings, symbols and the like`
  Corrected: `integers, strings, symbols, and the like`

### N2 — two pairs joined by a comma
`seasoned/part7-soundness.md:250–251`

Current:
> Coinduction and cycle detection (stop correctly),
> fuel and budget (stop when dangerous) answer the same question

Corrected:
> Coinduction and cycle detection (stop correctly) on one side, and
> fuel and budget (stop when dangerous) on the other, answer the same question

### N3 — volume TOC / front matter still say "HKT … + fuel" only
The chapter (subtitle line 12, §4-5 heading, summary table) now frames Rigor's answer as
"HKT / `App`, cycle detection and fuel." Two summaries lag behind:

- `seasoned/README.md:35`
  Current: `(+ a note: HKT / `App` + fuel)`
  Corrected: `(+ a note: HKT / `App`, cycle detection, and fuel)`
- `seasoned/part4-recursive-types.md:3` (front-matter `description`)
  Current: `…read the correspondence with Rigor's HKT / fuel implementation."`
  Corrected: `…read the correspondence with Rigor's HKT / cycle-detection / fuel implementation."`

(Check whether the Japanese README / front matter should move in step; if the Japanese is left
as is, leave the English as is too.)

### N4 — `HKT/`App`` vs. `HKT / `App`` spacing
The same file mixes both: lines 12 and 157 (and `seasoned/README.md:35`) use `HKT / `App``; lines
248 and 267 use `HKT/`App``. Pick the spaced form (the majority):

- `seasoned/part4-recursive-types.md:267`
  Current: `**HKT/`App`, cycle detection + fuel**`
  Corrected: `**HKT / `App`, cycle detection + fuel**`
- `seasoned/part4-recursive-types.md:248`
  Current: `declared HKT/`App` and fuel unimplemented`
  Corrected: `declared HKT / `App` and fuel unimplemented`

### N5 — "as is" vs. "as-is" (adverb)
The Little volume and appendices write the adverb "as is" (a3, part0/1/2/4/5, Seasoned P2); the
Seasoned volume writes "as-is" in the same function. The new lines add two more "as-is":
`seasoned/part4-recursive-types.md:171` ("returns the `App` being unfolded as-is") and
`seasoned/part7-soundness.md:228` (same). US style leaves the adverb open. Suggested: "as is" in
both new lines (and optionally part4:87, part5:89/244, examples/README.md:41/62).

### N6 — double parentheses
`seasoned/part5-real-inference.md:198`

Current:
> Yusuke Endoh (mametter) (details: the Rigor handbook, appendix-typeprof).

Corrected:
> Yusuke Endoh (mametter; details: the Rigor handbook, appendix-typeprof).

### N7 — cross-reference form
`little/part7-accepts-and-trinary.md:169`

Elsewhere the edition writes section references as "Part N §N-M" (e.g. "Seasoned Part 4 §4-5",
"Seasoned Part 7 §7-5"). This is the only "Part N's N-M".

Current: `as we saw in Part 4's 4-1x,`
Corrected: `as we saw in Part 4 §4-1x,`

### N8 — "the Little volume's Part 6"
`seasoned/part5-real-inference.md:297`

The edition's form is "Little Part 6."

Current: `a method version of the Little volume's Part 6 `HashShape`.`
Corrected: `a method version of Little Part 6's `HashShape`.`

### N9 — hard-to-parse gloss
`seasoned/part6-fact-store.md:67–68`

Current:
> facts about dynamic provenance (in real Rigor, for example, a block escaping
>    so that when it gets called can't be read).

Corrected:
> facts about dynamic provenance (in real Rigor, for example, that a block has escaped,
>    so when it will be called can't be determined).

### N10 — other "-ly" hyphenated compounds (edition-wide, outside the diff; optional)
Same rule as F4. `appendix/a2-narrowing-patterns.md` "immediately-invoked",
`seasoned/part6-fact-store.md` "immediately-invoked", `little/part8-rbs-and-signatures.md`
"externally-observed", `seasoned/part3-generics-and-substitution.md` "easily-crossed" → open
compounds ("immediately invoked," etc.). "simply-typed" is a term of art and may stay.

---

## Terms checked (consistent; no action)

- **opt-in** (noun/adjective) ×12, hyphenated everywhere; the one verb use "You opt in"
  (a1:181) is correctly open.
- **cycle detection** ×15, **safety net** ×6, **upper bound** ×9 — always open, lowercase.
- **join** — lowercase, used as the noun/verb of the lattice operation (little P6, seasoned P5/P6).
- **fail-soft** hyphenated as adjective/noun (×17); verb "falls soft" (a3) open — fine.
- **closed shape / open shape** — open, lowercase; italicized in Seasoned P2 for emphasis only.
- **`--params=observed` / `--params=untyped`**, **`parameter_inference:`**, **`sig-gen`**,
  **rbs-inline** — same spelling and code-span treatment in every occurrence.
- **μ-folding**, **pre-pass**, **hardcoded**, **re-evaluates** — single occurrences or consistent.
- **run time** (noun) / **runtime** (adjective) — used correctly by part of speech.
- **info level / warning level** — open, single use each.
- `figures/svg/seasoned-6-1.svg`: "block escaped" / "dynamic provenance" match the text figure
  and prose in `seasoned/part6-fact-store.md:68,78,84`.
- `seasoned/examples/README.md`: the two new linked files (`unification.rb`,
  `fact_invalidation.rb`) exist; table columns intact.
