# English edition (v2) — handoff / state ledger

Progress ledger for the **v2** English edition (`book/v2/en/`), a **transcreation** of
`book/v2/ja/`. See [STYLE.md](STYLE.md) for the writing guide. Update at each milestone.
Mirrors the role of v1's `book/v1/en/_handoff-state.md`.

Source of truth: `book/v2/ja/` at repo HEAD `7a70e7b` (2026-06-17).

## Approach

v2/ja shares v1/ja's file structure but its prose is **substantially rewritten** (≈30–60% of
lines changed per file) and carries two deliberate format changes:

1. **「三題噺」→「三つの視点」.** v1 framed each chapter as "the three voices"; v2 renames it
   **the three perspectives** (三つの視点 / パースペクティブ). The English follows; never
   "three voices."
2. **Columns are GitHub alerts.** v2 writes columns as `> [!NOTE]` / `> [!TIP]` /
   `> [!IMPORTANT]`. The English mirrors the same alert kind on the same block. Plain leads
   (goal line, figure captions) stay plain `>`.

Method: transcreate from `book/v2/ja/`, using `book/v1/en/` as a **terminology & voice
anchor** (reuse its wording where v2 matches v1; re-translate the rewritten passages). Figures
and example `.rb` are reused as-is (English-canonical, byte-identical to v1).

## ✅ Status: the full v2 book is transcreated to English

30 `.md` files, all written and verified:

- **Front matter** — [README.md](README.md), [glossary.md](glossary.md),
  [little/README.md](little/README.md), [seasoned/README.md](seasoned/README.md), plus
  [STYLE.md](STYLE.md) (contributor) and this ledger.
- **Little volume** — [Part 0–9](little/) complete.
- **Seasoned volume** — [Part 1–8](seasoned/) complete, plus
  [seasoned/examples/README.md](seasoned/examples/README.md).
- **Appendices** — [a1–a5](appendix/) complete.
- **Figures** — all 11 SVGs copied from `book/v1/en/figures/svg/` (every v2 SVG is
  byte-identical to its v1 counterpart, and v1/en's are already translated). English, valid SVG.
- **Seasoned example sketches** — `subtype.rb` / `subst.rb` / `mu_typeeq.rb` / `unification.rb`
  / `fact_invalidation.rb` + `check_docs.rb` copied from `book/v2/ja/seasoned/examples/`
  (English-canonical). All five self-check **green** (`ruby <file>` exits 0). The Seasoned
  chapters' `examples/*.rb` relative links now resolve.

### Verification run (whole tree)

- **Em dash** — clean: only the spaced ` — ` form. The only U+2014/U+2015/U+3000 hits are
  deliberate code-span examples in STYLE.md (documented exclusion).
- **Terminology** — "three perspectives" everywhere; `①②③` kept; no "three voices" outside
  STYLE/ledger (which document the rename rule).
- **Links** — all relative links resolve (the lone grep hit is the literal `` `](path)` `` inside
  STYLE.md's rule text, not a link). End-of-chapter snapshot links are the shared GitHub URLs
  (external, intended).
- **Figures** — 11/11 English, valid SVG.
- **Markers** — the `<!-- run: -->` / `<!-- include: -->` build markers are dropped from EN prose
  (per the v1/en convention); the only occurrences are in `seasoned/examples/README.md` where the
  mechanism is *described*, not applied.

## Review pass — DONE (English read-feel battery, 5 lenses)

Ran the established English review model (independent-context subagents; the `chibirigor-review`
skill itself targets `book/v1/ja/`, so it doesn't apply to EN prose). Notes in
[.reviews/](.reviews/): `_en-veteran-editor-review.md`, `_en-modern-editor-review.md`,
`_en-fp-researcher-review.md`, `_en-copyedit-review.md`, `_fidelity-review.md`.

- **Verdict: clean.** Zero type-theory ERRORs, zero fidelity breaks, near-publication copy. The
  formal content (bidirectional rules, S-Arrow variance, capture-avoiding `subst`, equi/iso
  split, progress + preservation, gradual consistency) and every citation (TAPL chapters,
  Kfoury–Wells / Wells / Siek & Taha / Milner / Liskov, fuel = 64, budget 1,000,000) checked out.
- **Fixes applied (axis-preserving):** a small recurring set of calques + mechanical copyedits —
  現場 "on-the-ground" → "working code"/"real-world"; 回らない "turns over" → "grinds to a halt";
  本体 "the chibirigor body" → "chibirigor proper / itself"; libに入った "entered lib" → "now lives
  in lib"; 汚す "soiling the core" → "polluting"/"without touching a single line"; 送られてくる論点
  "a point sent from" → "carried over from"; "the own terrain of" → "terrain unique to";
  "unfrightened" → "unflagged"; "unsynthesizable syntax" → "a form with no synthesis rule"; a
  few veteran prose tightenings (Part 4 subject-verb split, Part 8/9 flat tags). Copyedits:
  `(is a a subtype of b)` → backticked vars; "method an `untyped` value" → "on an"; `tyenv` →
  `tyEnv`; "equirecursive" → "equi-recursive"; 5 front-matter titles quoted (YAML `: ` safety);
  README 『しくみ』 subtitle normalized to the edition-wide gloss; Part 8 §8-1 table header
  `部品` "Part" → "Component".
- **Stop-slop pass (earlier):** scan found the worst AI tells absent; one redundant filler cut
  (Part 1 "simply").

## Conventions reminder

- Source of truth is `book/v2/ja/`; **re-transcreate**, don't word-for-word diff. v1/en is the
  anchor, not the source.
- Per-chapter verification (prose-only): links resolve · em dash spaced-only (en dash only for
  numeric ranges) · no U+3000 in English · figures exist · alerts mirror the JA block kind ·
  terminology matches the table · three perspectives keep `①②③`, never "three voices."
- Commit per logical unit, `git add <individual files>` (not `-A`), Japanese commit messages;
  **don't push until asked** (current branch `master`).

## Open items for the author (flagged, not resolved here)

1. **`seasoned/examples/README.md` lists only 3 sketches.** v2/ja's `examples/README.md` table
   (and its `$ ruby …` block) lists only `subtype.rb` / `mu_typeeq.rb` / `subst.rb`, while the
   Seasoned volume intro and the chapters reference all **five** (adds `unification.rb` Part 5,
   `fact_invalidation.rb` Part 6), and all five files exist. The EN edition faithfully reproduces
   v2/ja's 3-row table. This looks like a v2/ja oversight (v1 had all five) — consider
   back-porting the two missing rows to `book/v2/ja/seasoned/examples/README.md`, after which the
   EN table should be re-synced.
2. **HM / "type reconstruction" definition — EN fixed, JA back-port pending.** The glossary
   entry used to read "recovers types *from* annotations," which inverts the defining property:
   HM recovers types *without* annotations (it reconstructs the *omitted* ones; TAPL ch. 22). The
   FP-researcher lens flagged it as the one finding with real teeth. **EN now reads
   "recovers types *without* annotations, from how terms are used (it reconstructs the omitted
   ones)"** (`glossary.md`). The **canonical JA still carries the old wording**
   (`book/v2/ja/glossary.md:55` 「注釈から型を復元する推論」) — back-port the same fix at source
   (in Zenn → 逆輸入), so the EN currently reads slightly ahead here.
3. **Optional: wire `check_docs.rb` against `book/v2/en/`** (the EN prose currently omits the
   `<!-- include/run -->` markers; output is verified by reading). Same open follow-up noted in
   the examples README's English-edition note.
4. **Minor, faithful to JA (no action needed):** a4-2's Little-Part-8 row cites TAPL ch. 23/22
   (the homes of Seasoned P3/P5) under an explicit "partial / distant relative" hedge — matches
   the JA a4 table. a4-1's 『しくみ』 "Japanese only — the text says so plainly" is a deliberate
   EN-audience addition (consistent with README / Part 0).
