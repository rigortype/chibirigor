# Fidelity review — EN edition vs JA source (book/v2)

Lens: bilingual (JA↔EN) fidelity. Verify the English transcreation preserves the *content*
of the Japanese source — no inventions, no softening/strengthening, no dropped or contradicted
claims, no wrong numbers/names/citations. Prose rewrites, reordering, paragraph splits, and
columns→GitHub-alerts are legitimate transcreation and are **not** flagged.

Reviewed: README.md, glossary.md, little/README.md, little/part0–part9,
seasoned/README.md, seasoned/part1–part8, seasoned/examples/README.md, appendix/a1–a5.
(Date: 2026-06-17.)

## Verdict

**No ERROR-level fidelity breaks.** No invented, contradicted, or softened "real Rigor"
claims; no wrong ADR numbers, defaults, citations, internal-spec filenames, or identifier
names. Every load-bearing fact tracks the JA. Three MINOR items below, all arguable
transcreation.

Fact anchors verified present and correct across the corpus: ADR-0/2/3/4/5/14/16/20/22/25/32/
35/41/46/47/51, fuel default **64**, constant-fold budget **1,000,000**, the internal-spec
filename **inference-engine.md**, TAPL chapter/§ pointers (ch. 8 §8.3, 11 §11.7/§11.8/§11.10,
12, 15, 16, 20–21, 22, 23, 26, 29), 『しくみ』 ch. 1–9 pointers, all a4 correspondence-table
rows, the a2-6 PHPStan vocabulary table, the a1 special-types correspondence tables, and the
deliberate hedges ("Status: Proposed / not implemented", "designed but unimplemented",
"implemented", "reduced/naive version", "out of scope in the body", "opt-in / stays silent").

---

## MINOR findings

### MINOR-1 — `seasoned/part8-toward-rigor.md:26` — table column header mistranslated

- EN (line 26): `| Part | Little volume's minimal version | Rigor's real thing |`
- JA (line 21): `| 部品 | 前編での最小版 | Rigorの本物 |`

`部品` means "component / piece," and the column lists software components (`type_of`,
`accepts`, `Scope`, `MethodDispatcher`, type carriers) — not chapter "Part"s. Rendering it
"Part" collides with the chapter-number sense of "Part" used everywhere else in the book; the
§8-6 recap table (en line 173) correctly uses "Part" for actual chapter numbers, so within
this same file "Part" now means two different things.

Fix: change the header at line 26 to **"Component"** (or "Piece").

### MINOR-2 — `appendix/a4-bibliography.md:31–32` — added self-referential clause

- EN (lines 31–32): `**Published in Japanese only** — where it is the sole correspondence and
  no English equivalent exists, the text says so plainly.`
- JA (line 23): `日本語のみ` ("Japanese only").

The EN appends a meta-promise ("the text says so plainly") the JA does not make. It is an
EN-audience transcreation, consistent with the EN-only "(Japanese readers only)" tags
elsewhere, and not a factual claim about Rigor — so it is defensible. But it is a strengthened
statement relative to the bare JA.

Fix (only if strict parity wanted): trim to "Published in Japanese only." or recast the added
clause as a translator's note.

### MINOR-3 — `appendix/a5-other-languages.md:58` — dropped parenthetical gloss

- EN (line 58): `Java is basically a **nominal subtyping** world`
- JA (line 30): `Javaは基本「名前で（継承宣言で）決まる」**名前的部分型**の世界`

The EN drops the parenthetical gloss "（継承宣言で）/ determined by inheritance declaration"
that the JA attaches to "by name." Pure prose compression, no factual change.

Fix: optional; not worth changing.

---

## Per-file confirmations (faithful)

- **README.md** — Faithful. The TAPL-before-『しくみ』 reorder and the honest "published in
  Japanese only" framing are legitimate EN-audience transcreation.
- **glossary.md** — Faithful. All TAPL chapter pointers (15, 16, 20–21, 22, 23, 29, 8 §8.3),
  carrier names (`Difference`/`Refined`/`IntegerRange`/`Constant<"FOO">`), and the
  refinement-carrier / set-difference explanation match.
- **little/README.md** — Faithful.
- **little/part0-introduction.md** — Faithful. Rigor claims (PHP-static-analysis decade,
  RubyKaigi 2026 Hakodate, RBS-on-top, TypeProf whole-program), two promises, three
  perspectives, citations all track.
- **little/part1-literals-and-arithmetic.md** — Faithful. `Const`/`Nominal`/`Dynamic`,
  rounding, ~50-vs-『しくみ』-ch.2-~40-lines claim, ADR-51, a3-2 pointer.
- **little/part2-method-dispatch.md** — Faithful. Five-stage cascade, `Constant<3>`,
  constant-fold budget `1_000_000`.
- **little/part3-scope-and-statements.md** — Faithful. Immutable `Scope`, reassignment-as-
  type-swap, FactStore handoff, 『しくみ』 ch. 3–4 / TAPL ch. 9 + ch. 11 §11.5.
- **little/part4-union.md** — Faithful. Untagged-union framing, `inference-engine.md` MUST
  clause verbatim, the Rigor-makes-`nil` vs chibirigor-collapses-to-`untyped` hedge.
- **little/part5-narrowing.md** — Faithful. `--unreachable` opt-in hedge, ADR-47, a5-1/a5-5/a1.
- **little/part6-hash-and-tuple.md** — Faithful. Width-subtyping direction, Hack/PHPStan/Psalm
  lineage, 『しくみ』 ch.5/ch.7 + TAPL §11.7/§11.8.
- **little/part7-accepts-and-trinary.md** — Faithful. Three-valued `accepts`, `:maybe`-not-
  punished, Postel asymmetry, 『しくみ』 ch.7 / TAPL ch.15.
- **little/part8-rbs-and-signatures.md** — Faithful. `Rbs::CORE` method set, void/`-> nil`
  contract, TypeProf-vs-Rigor design-judgment hedge, 『しくみ』 ch.9 / TAPL ch.22,23.
- **little/part9-gradual-philosophy.md** — Faithful. Four deliberate misses, baseline match-key
  reasoning, ADR-22, Top/Bot lattice, recap table, TAPL ch.8 §8.3.
- **seasoned/README.md** — Faithful. Chapter table, five runnable sketches, `check_docs.rb`,
  "neither required" hedge.
- **seasoned/part1-bidirectional-typing.md** — Faithful. Var-Synth/Sub rules, `ExpressionTyper`
  ⇒ / `accepts` ⇐ / Γ mapping, Pierce&Turner 2000 / Dunfield&Krishnaswami 2021.
- **seasoned/part2-subtyping-and-variance.md** — Faithful. S-Arrow variance, `subtype.rb` + 8
  PASS, declaration-site-variance designed-but-unimplemented (Slice 5), ADR-35.
- **seasoned/part3-generics-and-substitution.md** — Faithful. Capture-avoiding `subst`, TAPL
  23.7 erasure vs Java erasure, `RbsTypeTranslator`/`Type#erase_to_rbs`, 5a/5b/5c staging.
- **seasoned/part4-recursive-types.md** — Faithful. μ-unfold, iso/equi-recursive (TAPL 20.2),
  reduced-vs-TAPL-ch.21 hedge, `App[..]` arity 1, **fuel default 64**, HKT = TAPL ch. 29.
- **seasoned/part5-real-inference.md** — Faithful. Capability roles, `Dynamic[Top]`=`untyped`,
  TypeProf-vs-local+catalog, `rigor sig-gen` ch.11, ADR-5/2/16, TAPL ch.22, Kfoury–Wells 1994.
- **seasoned/part6-fact-store.md** — Faithful. Six buckets, `FactStore::Fact` fields, minimize-
  to-three, join semantics, `inference-engine.md`, a2-3/a2-4/a2-6 pointers.
- **seasoned/part7-soundness.md** — Faithful. Progress+preservation (TAPL 8.3), normalization
  (TAPL ch.12), gradual consistency `~`, HKT fuel 64 / ADR-20 "implemented", inference budget
  ADR-41 "Status: Proposed, designed-unimplemented", Siek&Taha 2006.
- **seasoned/part8-toward-rigor.md** — Faithful except MINOR-1. All 23 ADR numbers, doorway-ADR
  table, plugin-hook sketch, ADR reading guide (ADR-25/32, `inference-engine.md`), §8-6 recap.
- **seasoned/examples/README.md** — Faithful (the shared "only 3 sketches" count is a known
  JA-side item, not flagged; the EN matches it). The added "English edition note" about omitted
  include/run markers and `check_docs.rb` being wired against JA prose is an accurate edition-
  specific addition, not a defect.
- **appendix/a1-special-types.md** — Faithful. Go/PHP/C#/Elixir/Rigor rows, ADR-47 +
  `flow.unreachable-clause`, `lib/chibirigor/narrowing.rb`.
- **appendix/a2-narrowing-patterns.md** — Faithful. Six buckets, `Difference`/`Refined`/
  `IntegerRange` + ADR-3, full PHPStan correspondence table (incl. three "—" rows), a2-6 NOTE.
- **appendix/a3-tooling.md** — Faithful. Five-stage cascade, ③-beats-④ / `RBS::Extended`,
  `dump_type` mechanics, all filenames, 17-step trace, `--line=N`/`--json` flag differences.
- **appendix/a4-bibliography.md** — Faithful except MINOR-2. All three tables verified row-for-
  row (a4-2 Little×しくみ×TAPL, a4-3 Seasoned×TAPL×しくみ, a4-4 ADRs incl. ADR-20 "implemented"
  / ADR-41 "Status: Proposed, not implemented" / `inference-engine.md`).
- **appendix/a5-other-languages.md** — Faithful except MINOR-3. Hack→PHPStan/Psalm→Rigor
  lineage, nominal/structural distinction, untagged-Union rationale, missing-vs-unreachable-arm
  table, `--unreachable` hedge.

---

## Informational (out of EN-fidelity remit — JA-source hygiene)

Two JA source files end with stray literal authoring artifacts; the EN correctly does **not**
reproduce them, so EN↔JA meaning fidelity is unaffected. Noted only so the JA source can be
cleaned if desired:

- `book/v2/ja/seasoned/part2-subtyping-and-variance.md` (lines 251–252): stray `</content>` /
  `</invoke>` lines.
- `book/v2/ja/.../part2` (method-dispatch authoring artifact noted by reviewer) — stray
  `</content>` / `</invoke>`-style tail. (Verify against JA source before editing.)
