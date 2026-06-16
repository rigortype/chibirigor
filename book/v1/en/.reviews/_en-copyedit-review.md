# EN copyedit review — final mechanical-correctness pass

**Reviewer seat:** native-English copyeditor doing the FINAL polish pass. Scope is **mechanical
correctness and consistency only** — articles, number/agreement, punctuation, hyphenation,
US-spelling, its/it's, doubled words, prepositions. No style/flow/voice/register changes; no
technical-claim changes; code blocks / identifiers / RBS / CLI text / inline code spans / the
`①②③` device / the spaced ` — ` em-dash convention / 『しくみ』 left untouched.

**Coverage:** whole book read closely, Little volume first (README + part0–part9), then shared
README + glossary, then Seasoned volume (part1–part8) and appendices a1–a5. Targeted grep
sweeps run for every high-yield pattern (hyphenation of recurring compounds, US vs. British
spelling, a/an before countable singulars and abbreviations, subject–verb agreement, its/it's,
affect/effect, differ-from, doubled words, em-dash style).

---

## SHORT VERDICT

**The prose is exceptionally clean — near-publication mechanical quality.** This reads as
English-first, and the translation-from-Japanese tells (missing/extra articles, number
mismatches, comma splices, calqued prepositions) that this pass hunts for are essentially
**absent.** US spelling is uniform; em-dash convention (` — `) is uniform; `:yes`/`:no`/`:maybe`,
"Little volume" (90×) / "Seasoned volume" (49×) / "main volume" (34×), "type checker" (noun, 30×
open) vs. "type-check" (verb, hyphenated) are all internally consistent and correctly
distinguished. The verb/noun split on compound terms (e.g. "two-layer" adjective vs. "two layers"
noun; "type-check time" modifier vs. "type checker" noun) is handled correctly throughout.

Only **two genuine, tiny findings** survive the sweep — one spelling-consistency slip and one
soft compound-term wobble. Neither is an error of grammar or meaning; both are one-token fixes.
I did not manufacture corrections to fill a quota.

---

## Findings

| File · line/section | before | after | category |
|---|---|---|---|
| `seasoned/part1-bidirectional-typing.md` · L66 | `There are two judgements:` | `There are two judgments:` | consistency (US spelling) |
| `little/part6-hash-and-tuple.md` · §6-3 L119, L122 | `option hashes` / `option hash` | `options hashes` / `options hash` | consistency (compound term) |

### Notes on the two findings

**1 · "judgements" → "judgments" (US spelling consistency).** The book uses the US spelling
"judgment" **22 times** across both volumes and the appendices (e.g. part2, part6, part7,
seasoned part2/4/5/7). This single British "judgements" in Seasoned Part 1 §1-2 is the lone
outlier and should match. (No other British spellings exist anywhere — `behaviour`, `-ise`
verbs, `colour`, etc. all came back clean.)

**2 · "option hash" vs. "options hash" (PATTERN, low severity).** The compound varies within a
single chapter: Part 6 uses **"option hash(es)"** at L119/L122 but **"options hash(es)"** at
L140/L188; elsewhere it's "option hashes" (part5 L263, part9 L54) and "options hashes" (part6
L188, a5 L76). Both forms are valid English, but for one-book consistency I'd standardize on the
conventional Ruby-community form **"options hash."** All locations:
`little/part6-hash-and-tuple.md` L119, L122, L140, L188; `little/part5-narrowing.md` L263;
`little/part9-gradual-philosophy.md` L54; `appendix/a5-other-languages.md` L76. (If the author
prefers "option hash," standardize the other way — the point is just to pick one.) This is a
judgment call, not an error; safe to leave if intentional.

---

## What was checked and came back clean (so the author can trust the sweep)

- **Articles (a/an/the):** no missing/extra/wrong articles before countable singulars found.
  "an RBS" (14×, vowel sound — correct), "an HKT" (correct), "an Integer" (correct), "an
  unimplemented" (correct) all verified. No `a`-before-vowel or `an`-before-consonant slips.
- **Number / subject–verb agreement:** every flagged candidate ("…in types is", "code with no
  annotations is", "…arguments was `⇐`") has a correctly singular subject (a gerund or a
  notional singular). No real mismatches.
- **Punctuation:** no comma splices or run-ons surfaced in close reading; em-dash style is the
  spaced ` — ` throughout, with **zero** unspaced `word—word`, en-dash, or `―` (U+2015) misuse.
- **Hyphenation of recurring compounds:** "type checker"/"type checking" (open noun, 30×/4×) vs.
  "type-check"/"type-checked"/"type-check time" (hyphenated verb/modifier) — correct split.
  "two-layer" (adjective) vs. "two layers" (noun) — correct split. "call site" (open, 11×),
  "fail-soft" (hyphenated, uniform), "non-nil" (hyphenated, uniform), "untagged union" (open,
  uniform in reader prose), "annotation-free" (uniform). All consistent.
- **US spelling:** uniform; no `-ise`/`-our`/`-re`/`-ll-` British forms in reader-facing prose.
- **its / it's:** every `it's` is a correct contraction; no possessive confusion.
- **affect / effect:** all uses correct (noun "effect", etc.).
- **Doubled words:** none.
- **Prepositions / calques:** no "differ to/with", no obvious calqued discourse markers
  surviving (the prior read-feel pass already cleared "gets angry" etc., outside this pass's
  remit).
- **`:yes`/`:no`/`:maybe`, "Little/Seasoned/main volume" caps, `①②③`, 『しくみ』, RBS/CLI
  text:** all consistent / untouched as required.

(`.reviews/` files, `STYLE.md`, and `_handoff-state.md` are contributor docs and were used as
the consistency oracle, not edited. The `STYLE.md`-only "untagged-union"/"two-layer" hyphenations
there don't bind reader prose.)
