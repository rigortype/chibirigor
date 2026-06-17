# English copyedit review — chibirigor v2/en

Near-publication copyedit pass over the reader-facing prose of the English edition
(`book/v2/en/`). Lens: grammar, US spelling, articles/prepositions, agreement, hyphenation
consistency, capitalization consistency, doubled words, parallel structure, recurring-term
consistency, and front-matter YAML. House typography (spaced em dash ` — `, en dash for numeric
ranges, circled numerals `①②③`, curly quotes, `『しくみ』`, code identifiers) is treated as
correct per the project constraints and is **not** flagged.

Severity: **ERROR** (grammatical/typo defect a reader will trip on) · **FIX** (clear consistency
or correctness defect) · **NIT** (minor polish / latent risk).

Each entry gives `file:line`, the exact current text, and the exact corrected text so the fix can
be applied mechanically.

Overall: the prose is clean and remarkably consistent. The findings below are few and small; none
are substantive. The copy is publication-ready after these mechanical fixes.

---

## ERROR

### E1 — doubled word "a a" (and a missing code span)
`little/part7-accepts-and-trinary.md:26`

Current:
> This is an **acceptance check.** In 『しくみ』 ch. 7 it's built as `subtype(a, b)` (is a a subtype
> of b), and the answer was a two-way `true`/`false`.

The parenthetical glosses the call `subtype(a, b)`; "is a a subtype of b" has a doubled "a" and
leaves the variable names `a`/`b` as bare prose. Corrected:
> This is an **acceptance check.** In 『しくみ』 ch. 7 it's built as `subtype(a, b)` (is `a` a subtype
> of `b`), and the answer was a two-way `true`/`false`.

(Minimal alternative if code spans are unwanted here: "(is `a` a subtype of `b`)" → "(is a a
subtype of b)" must at least lose one "a": "(is *a* a subtype of *b*)".)

### E2 — missing preposition "on"
`seasoned/part7-soundness.md:92` (table cell, row ①)

Current:
> … | calling a method an `untyped` value doesn't answer → `NoMethodError` |

"calling a method an untyped value" is missing "on". Parallels row ②'s "a call to `nil`".
Corrected:
> … | calling a method on an `untyped` value doesn't answer → `NoMethodError` |

---

## FIX

### F1 — `tyEnv` / `tyenv` capitalization inconsistency
`little/part3-scope-and-statements.md`

The same identifier is spelled `tyEnv` at lines 16, 17, 50 (and `a4-bibliography.md:69`) but
`tyenv` at lines 63 and 185. Pick one (the camelCase `tyEnv`, matching 『しくみ』's variable name
and the a4 table, is the better anchor). Two changes:

- Line 63 — Current:
  > - **① Type theory:** the mapping that remembers variables' types = the type environment, tyenv
  Corrected:
  > - **① Type theory:** the mapping that remembers variables' types = the type environment, `tyEnv`
- Line 185 — Current:
  > | ① Type theory (『しくみ』 ch. 3–4 / TAPL ch. 9, 11) | The type environment tyenv that remembers variables' types; sequencing that threads statements |
  Corrected:
  > | ① Type theory (『しくみ』 ch. 3–4 / TAPL ch. 9, 11) | The type environment `tyEnv` that remembers variables' types; sequencing that threads statements |

(The lines 16/17/50 occurrences already use `tyEnv` in code spans; if the author prefers the bare
lowercase form instead, change those plus a4 — but the camelCase form is the dominant and clearer
choice.)

### F2 — `equirecursive` vs `equi-recursive` hyphenation
`appendix/a4-bibliography.md:94`

`seasoned/part4-recursive-types.md` (the chapter that defines the terms) and `part7` consistently
hyphenate: **equi-recursive** / **iso-recursive**. The a4 table uses the unhyphenated
`equirecursive`. Current:
> | 4 | Recursive types: μ and coinduction | ch. 20, 21 (HKT in ch. 29) | ch. 8 | match (equirecursive, coinduction = greatest fixed point; the HKT aside has separate grounds) |

Corrected (hyphenate to match the chapter):
> | 4 | Recursive types: μ and coinduction | ch. 20, 21 (HKT in ch. 29) | ch. 8 | match (equi-recursive, coinduction = greatest fixed point; the HKT aside has separate grounds) |

### F3 — `『しくみ』` English title gloss inconsistent in README
`README.md:82-86`

Everywhere else the book glosses 『しくみ』 as **"*The Mechanics of Type Systems*"** with the full
subtitle "*…Learning Types and Programming Languages by Building Them in TypeScript*"
(`little/part0-introduction.md:95-96`, `little/part1-…:16`, `appendix/a4-bibliography.md:27-28`,
`glossary.md`, the page titles). README alone glosses it as "*Building a Type System in
TypeScript*". Current (README.md:82):
> - **『しくみ』** (*The Mechanics of Type Systems*) — Yusuke Endoh, *Building a Type System in
>   TypeScript* (Lambda Note). …

Corrected (match the a4 / Part 0 wording — the subtitle, not a different title):
> - **『しくみ』** (*The Mechanics of Type Systems*) — Yusuke Endoh, *Learning Types and
>   Programming Languages by Building Them in TypeScript* (Lambda Note). …

Note: this is a title/subtitle consistency fix, not a content change; if "*Building a Type System
in TypeScript*" is a deliberate looser gloss the author wants to keep in the README, leave it —
but at minimum the edition should not present two different subtitles for the same book. Flagged
for an author confirm.

---

## NIT

### N1 — front-matter titles with an embedded `: ` should be quoted (consistency + YAML safety)
`little/part0-introduction.md:2`, `little/part4-union.md:2`, `little/part5-narrowing.md:2`,
`seasoned/part4-recursive-types.md:2`, `seasoned/part5-real-inference.md:2`

These five `title:` values contain an unquoted `": "` (a colon-space inside the scalar), e.g.
`title: Part 0 — Introduction: an inference-driven type checker`. Several sibling files already
quote their titles (e.g. `part6-hash-and-tuple.md` → `title: "Part 6 — Hash and array types"`,
`seasoned/part2-…` → `title: "Part 2 — Subtyping and variance"`, `seasoned/part8-…`,
`appendix/a1-…`). A `: ` inside an unquoted YAML scalar is a latent parse ambiguity and is
stylistically inconsistent with the quoted-title files. Quote all five, e.g.:

- `title: Part 0 — Introduction: an inference-driven type checker`
  → `title: "Part 0 — Introduction: an inference-driven type checker"`
- `title: Part 4 — Union: when a type doesn't settle on one`
  → `title: "Part 4 — Union: when a type doesn't settle on one"`
- `title: Part 5 — Narrowing: splitting by case`
  → `title: "Part 5 — Narrowing: splitting by case"`
- `title: Part 4 — Recursive types: μ and coinduction`
  → `title: "Part 4 — Recursive types: μ and coinduction"`
- `title: Part 5 — Real type inference: filling in arguments`
  → `title: "Part 5 — Real type inference: filling in arguments"`

(Current parsers accept the unquoted form because the value doesn't begin like a mapping key, so
this is a NIT, not an ERROR — but quoting removes the risk and matches the rest of the edition.)

---

## Checked and clean (no action)

These categories were swept across the corpus and found consistent:

- **US spelling** — no British spellings in prose (no behaviour/colour/normalis/judgement/whilst/
  towards/-ise/-yse forms). "judgment", "behavior", "normalization" used correctly where they
  appear.
- **Recurring terms** — "the Little volume" / "the Seasoned volume", **fail-soft** (incl.
  fail-softs/fail-softed), **options hash(es)**, **flow-sensitive**, **three-valued** (with
  "trinary" used only as the intentional gloss), **type carrier**, **width / depth subtyping**,
  **never frighten working code** — all consistent.
- **Hyphenation** — three-valued, flow-sensitive, equi-recursive/iso-recursive (except F2),
  type-check(er), Hindley–Milner / Kfoury–Wells (en dash, correct for compound proper names),
  numeric ranges (`ch. 20–21`, en dash) — consistent.
- **Articles a/an** — verified across every "a/an + initialism/vowel" hit (an RBS, an FP, an
  `:info`, a URI, an `Integer`, an `else`, an `untyped`): all correct (article matches the spoken
  next word).
- **Doubled words** — only the E1 prose case; all other regex hits are inside code/diagram fences
  (`S S`, `U U`, `x x`, `returns returns` in rule/ASCII blocks) — correct.
- **Em dash / full-width space** — spaced ` — ` house form throughout; no stray non-spaced em
  dash in prose; no U+3000 full-width spaces. (Constraint honored — not flagged.)
- **Front-matter YAML** — `sidebar.order` present and sequential; `description:` values quoted
  where they contain colons; only the title-quoting NIT (N1) outstanding.
- **Parallel structure** — summary tables (the `① / ② / ③` three-perspective rows, the
  "Handed to the sequel" lists, the cross-language tables in a1/a5, the a3 "this book / the real
  thing" tables) are internally parallel.
