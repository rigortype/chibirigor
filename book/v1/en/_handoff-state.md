# English edition — handoff / state ledger

Progress ledger for the English edition (`book/v1/en/`), a **transcreation** of `book/v1/ja/`
following php-ministan's English-edition approach (see [STYLE.md](STYLE.md)). Update at each
milestone. Mirrors the role of `book/v1/ja/_handoff-state.md`.

Last synced against `book/v1/ja/` at repo HEAD `8cc7a2b` (2026-06-16).

## Done

- **Scaffolding** — `book/v1/en/{little,seasoned,appendix,figures/svg,.reviews}/`.
- **[STYLE.md](STYLE.md)** — the English-edition writing guide (transcreation stance, what
  is/isn't translated, path mapping, Starlight front matter, voice & tone, the three voices,
  canonical-English terminology table, house typography, reference apparatus, fidelity,
  review model, per-chapter verification). The keystone.
- **[README.md](README.md)** — front matter / preface.
- **[glossary.md](glossary.md)** — full glossary (Little terms + Seasoned/Rigor terms).
- **[little/README.md](little/README.md)**, **[seasoned/README.md](seasoned/README.md)** —
  volume intros.
- **[little/part0-introduction.md](little/part0-introduction.md)** — Part 0 (the keystone
  chapter: the two promises, the three voices, the inference-as-foundation framing).
- **[little/part1-literals-and-arithmetic.md](little/part1-literals-and-arithmetic.md)** —
  Part 1 (`Const` / `type_of` / `check` / `annotate`).
- **[little/part2-method-dispatch.md](little/part2-method-dispatch.md)** — Part 2 (the
  `Dispatch` table, `class_of`, `matches?`, `dispatch`; + the constant-folding note).
- **[little/part3-scope-and-statements.md](little/part3-scope-and-statements.md)** — Part 3
  (the immutable `Scope`, `eval_statement`, threading statements). No figure (none in JA).
- **[little/part4-union.md](little/part4-union.md)** — Part 4 (`Union`, `union`, typing
  `IfNode`; + the Union-receiver distributive-dispatch note). Figure
  [little-4-1.svg](figures/svg/little-4-1.svg).
- **[little/part5-narrowing.md](little/part5-narrowing.md)** — Part 5 (`remove_nil`, `narrow`,
  the `possible?` guard, unreachable arm, the two laws; footnote preserved). Figure
  [little-5-1.svg](figures/svg/little-5-1.svg).
- **[little/part6-hash-and-tuple.md](little/part6-hash-and-tuple.md)** — Part 6 (`HashShape`,
  `Tuple`, `read_index`, the open-policy / width-subtyping climax). Figure
  [little-6-1.svg](figures/svg/little-6-1.svg).
- **[little/part7-accepts-and-trinary.md](little/part7-accepts-and-trinary.md)** — Part 7
  (`accepts`, `widen`, three-valued `:yes`/`:no`/`:maybe`, Union fold, "maybe is not punished,"
  Postel's-law column; footnote preserved). Figure [little-7-1.svg](figures/svg/little-7-1.svg).
- **[little/part8-rbs-and-signatures.md](little/part8-rbs-and-signatures.md)** — Part 8
  (`Rbs.load`, `Rbs::CORE`, swap table → RBS, `def` return-type synthesis, `method_signature`,
  the `void` contract note, the TypeProf note). No figure.
- **[little/part9-gradual-philosophy.md](little/part9-gradual-philosophy.md)** — Part 9 finale
  (`untyped` contagion, the four deliberate misses, baseline + ADR-22, the three special types,
  the gradual-typing stance table). No figure.

**The Little volume (Part 0–9 + README) is complete in English**, with figures `little-0-1`,
`-2-1`, `-4-1`, `-5-1`, `-6-1`, `-7-1` (Parts 1, 3, 8, 9 have no figure in JA).

- **Appendices (all five complete):**
  [a1-special-types](appendix/a1-special-types.md) (the lattice, `untyped`/`void`/`never`,
  `Top`/`Bot`, the unreachable-arm worked example),
  [a2-narrowing-patterns](appendix/a2-narrowing-patterns.md) (the six fact patterns, refinement
  carriers, the PHPStan vocabulary table),
  [a3-tooling](appendix/a3-tooling.md) (`--explain`, the two-layer display/erasure, the 5-stage
  dispatch cascade, the `trace` animation),
  [a4-bibliography](appendix/a4-bibliography.md) (the 『しくみ』/TAPL/ADR correspondence tables),
  [a5-other-languages](appendix/a5-other-languages.md) (null safety, nominal/structural,
  HashShape lineage, untagged unions, exhaustiveness direction).

Full sweep: em-dash clean across all of `little/*.md` and `appendix/*.md`; the Little volume +
appendices are self-contained in English.

### Seasoned volume (in progress)

- **[seasoned/part1-bidirectional-typing.md](seasoned/part1-bidirectional-typing.md)** — Part 1
  (synthesis `⇒` / checking `⇐`, the formal rules Var-Synth/Sub, why diagnostics live at `⇐`,
  robustness = direction, the `check(rbs:)` mode note, the `param:` directive column). Figure
  [seasoned-1-1.svg](figures/svg/seasoned-1-1.svg). Verified.
- **[seasoned/part2-subtyping-and-variance.md](seasoned/part2-subtyping-and-variance.md)** —
  Part 2 (subtyping `<:`, width/depth records, S-Arrow co/contravariance, the `subtype` sketch,
  robustness=LSP=S-Arrow convergence, container variance, gradual consistency, algorithmic
  subtyping, the Sorbet column). Figures [seasoned-2-1.svg](figures/svg/seasoned-2-1.svg),
  [seasoned-2-2.svg](figures/svg/seasoned-2-2.svg). Verified.

- **[seasoned/part3-generics-and-substitution.md](seasoned/part3-generics-and-substitution.md)**
  — Part 3 (type abstraction/application, the `subst` shadowing + capture pitfalls, fresh/α-conv,
  α-equivalence, erasure, the generics-5a element-read lib note). No figure. Verified.
- **[seasoned/part4-recursive-types.md](seasoned/part4-recursive-types.md)** — Part 4 (μ-types,
  fold/unfold, equi/iso-recursive, the `type_eq` coinduction sketch, Rigor's HKT/`App`+fuel
  alternative, the JSON `symbolize_names` and three-valued-HKT columns). Figure
  [seasoned-4-1.svg](figures/svg/seasoned-4-1.svg). Verified.
- **[seasoned/part5-real-inference.md](seasoned/part5-real-inference.md)** — Part 5 (Road A
  capability/duck, Road B constraints+`unify` sketch, the three HM-problems column, the TypeProf
  vs. local+catalog comparison, generics 5b/5c push-down note). No figure. Verified.
- **[seasoned/part6-fact-store.md](seasoned/part6-fact-store.md)** — Part 6 (the six buckets,
  stability/invalidation, the `FactStore` sketch, closure capture, join; footnote preserved).
  Figure [seasoned-6-1.svg](figures/svg/seasoned-6-1.svg). Verified.
- **[seasoned/part7-soundness.md](seasoned/part7-soundness.md)** — Part 7 (type-system
  narrow/broad senses, progress+preservation, normalization, the "four deliberate-unsound holes
  restated in progress terms" table, gradual consistency `~` + the gradual guarantee, the
  `assert:` directive column, coinduction→fuel→budget termination engineering). No figure. Verified.
- **[seasoned/part8-toward-rigor.md](seasoned/part8-toward-rigor.md)** — Part 8 finale
  (chibirigor↔Rigor correspondence, the engineering-ADR table, the `register_method` tiny-plugin
  note, the ADR reading map, three doorways, the two-volume recap). No figure. Verified.
- **[seasoned/examples/README.md](seasoned/examples/README.md)** — design-sketch index +
  `check_docs.rb` drift-prevention doc (with an English-edition note on the dropped markers).

### ✅ Status: the full v1 book is transcreated to English

Front matter + **Little Part 0–9** + **Seasoned Part 1–8** + **appendices a1–a5** + glossary +
both volume READMEs + examples README + **11 figures** (little-0-1/2-1/4-1/5-1/6-1/7-1,
seasoned-1-1/2-1/2-2/4-1/6-1). 30 `.md` files. Final full-tree sweep: em-dash clean (the only
U+2014/U+2015 hits are deliberate code-span examples in STYLE.md, excluded by the documented
check); every figure rendered and eyeballed; the **only** unresolved relative links are the five
shared `seasoned/examples/*.rb` design sketches (forward-refs under the open shared-tree decision,
below). Synced against JA HEAD `8cc7a2b`.

**Remaining work** is no longer chapter translation but: (1) the **English review pass** (read-feel
battery — see "Review" below); (2) resolving the **open shared-tree decision** (code comments /
CLI diagnostics / the `seasoned/examples/*.rb` sketches are Japanese today). Neither blocks the
book being readable end-to-end in English.

**Seasoned `examples/` links:** Seasoned chapters link to design sketches `examples/subtype.rb`,
`examples/subst.rb`, etc. (same relative tokens as JA). These are **shared design-sketch code**
(`book/v1/ja/seasoned/examples/*.rb`, Japanese comments) and fall under the open shared-tree
decision — the `book/v1/en/seasoned/examples/` tree is **not** created yet, so these links are
intended-pending forward-references (like the `impls/dist` snapshot links). Resolve them when the
shared-tree decision lands. The printed excerpts carry English-translated comments.
- **Figures:** [figures/svg/little-0-1.svg](figures/svg/little-0-1.svg) (Part 0),
  [figures/svg/little-2-1.svg](figures/svg/little-2-1.svg) (Part 2). Text-only English
  (geometry/colors reused from the JA single-variant SVGs). Verified: English text fits;
  renders identically to the JA source under inkscape (boxes/lines need the site's `--rigor-*`
  CSS vars to show — a renderer limitation shared with the JA figures, not a regression).

**Convention settled while translating:** the JA chapters carry a `<!-- run: examples/partN.rb -->`
build marker before some output blocks (it asserts the block equals the JA example's output).
English chapters **drop it** — there is no English `examples/` tree, and from Part 2 on the
output contains transcreated diagnostic strings that the JA example would not emit. English
example output is verified by reading; a future English `examples/` tree (the open decision
below) could restore an analogous check.

Verification run (each chapter): em-dash typography clean (spaced ` — ` only; en dash kept only
for numeric ranges like `20–21`); all unresolved relative links are intended forward-references
to not-yet-translated chapters/appendices; figures rendered and eyeballed.

## Review iteration 1 — DONE (4 layers, fixes applied & committed)

Ran the multi-lens battery (independent-context subagents, layered 真→読→整):
- **L1 真 (truth):** `_fidelity-review` (en↔ja claim preservation) and `_en-fp-researcher-review`
  (type-theory soundness) — **both passed clean: zero ERROR / zero MISLEADING.** All ADR numbers,
  defaults (fuel 64, fold budget 1,000,000), TAPL/『しくみ』 citations, hedges ("Status: Proposed",
  the reduced-coinduction caveat, etc.) preserved; formal content sound.
- **L3 読 (read-feel):** `_en-veteran-editor-review` + `_en-modern-editor-review` — strong
  transcreation; convergent translation-smell fixes **applied**: 怒る→flag/complain/blame
  (~11×), 口→point/place/raise (4×), 倒す→fall back to/collapse to/default to/drop to (~17×),
  手元で→"when you run it yourself" (4×), 流儀/作法→way/discipline/practice, running start→head
  start. Both editors judged the Little/Seasoned + three-voices framing lands cleanly in 2020s
  English.
- **L4 整 (polish):** `_en-copyedit-review` — "near-publication" clean; fixes **applied**:
  judgements→judgments (US), "option hash"→"options hash" (3×).
- **L2 伝 (reproducibility): intentionally skipped** — the teaching structure is unchanged from
  the JA, which already passed reproducibility 34/34; the translation risk was read-feel +
  fidelity, both covered. Re-run if a future edit changes the *teaching* prose, not just wording.

Notes committed to `book/v1/en/.reviews/` (tracked, like the JA side). Verification after fixes:
em-dash clean, no residual flagged calques, links unchanged (only the 5 shared `examples/*.rb`).

## Review iteration 2 — DONE (L2 伝 teaching, the layer iteration 1 skipped)

Ran the two target-reader personas on the **English** Little volume (independent contexts):
- `_java-reader-review` (Java mid, no type theory) — **teaches cleanly, zero STUCK.**
- `_ruby-reader-review` (Ruby/Rails junior, "type ≈ class", no annotation model) — **lands, zero
  BLOCK.** Both praised the same load-bearing moves: P1 "a type is *data*, flip one switch" riding
  in on `Data.define`, P3 Scope = "a mental note turned into data", the P5/a5-1 null-safety reframe.

Convergent teaching-clarity fixes **applied** (additive glosses, no new Rigor claim, fidelity-safe):
- **part6 §6-3** — added a value-set gloss defusing the counterintuitive "more keys = the *smaller*
  type", and deferred the word "subtype" to Part 7 (both readers snagged here).
- **part4 §4-1** — prefixed the untagged/tagged-variant note with a "skim now, circle back after
  §4-1 builds a Union" orientation (both readers hit the jargon before the Union existed).
- **part8 §8-1** — led with "read each line as a Ruby `def` header with types penciled in", started
  from the simpler `() -> String`, and split the `->`-is-not-a-lambda gotcha into its own aside
  (the Ruby reader's milestone — first type they ever read).

Not applied (deliberately): naming Liskov in the part7 §7-5 footnote (Seasoned P2 owns it);
folding the part0 §0.1 "annotation" gloss inline (matches JA, FRICTION not BLOCK). Logged in the
notes.

> **Candidate JA back-ports:** the three fixes above address gaps the *Japanese* original shares
> (the note-ordering in P4, the early "subtype" lean in P6, the dense first-RBS read in P8). The
> EN now reads slightly ahead of the JA here. Consider re-converging by back-porting to
> `book/v1/ja/` so both editions stay in sync (per STYLE "Sync with the Japanese source").

## Next (in order)

1. **Open shared-tree decision** (author call) — see below; the only substantive open item.
2. **Resolve the open shared-tree decision** (author call) — see below. If "migrate to English,"
   the `seasoned/examples/*.rb` links resolve and the snapshot/CLI click-throughs match the prose.
3. **Optional: curly-quote pass** — prose currently uses curly quotes/apostrophes inline; a
   bulk-verify pass (protecting code spans + HTML attrs per STYLE) before publishing.
4. **Optional: Starlight wiring** — confirm the English locale picks up `book/v1/en/` front
   matter (`title`/`description`/`sidebar.order` mirror JA).

## Open decision for the author (flagged, not resolved here)

**Shared-tree language.** chibirigor's `lib/`, the `impls/dist/partN` snapshots, and the
`examples/` trees carry **Japanese** comments and emit **Japanese** diagnostics. The English
prose renders excerpts/output in English, so a click-through (or running the tool) still shows
Japanese. ministan resolved the analogous gap by migrating the shared code to English. The
options for chibirigor — (a) migrate shared comments + message strings to English; (b) generate
an English-commented `book/v1/en/little/examples/` tree; (c) leave as-is and keep the
honest note — are a repo-wide call left to the author. A chapter-translation session should
**not** undertake a/b on its own. See STYLE.md "Open question (shared-tree language)."

## Conventions reminder

- Source of truth is `book/v1/ja/`; **re-transcreate**, don't word-for-word diff.
- Commit per logical unit, `git add <individual files>` (not `-A`), Japanese commit messages;
  **don't push until asked** (current branch `master`).
- Per-chapter verification (prose-only): links resolve · em dash spaced-only · figures exist
  and eyeballed · terminology matches the table · three voices keep `①②③`.
