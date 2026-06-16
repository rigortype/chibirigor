# English edition — style & terminology guide

> This is a **contributor document**, not part of the reader's path. It records how the
> English edition of chibirigor is written so that every chapter reads as one book. If you
> are here to read chibirigor, start at [README.md](README.md).

The English edition is a **transcreation** of the Japanese original under
[`book/v1/ja/`](../ja/README.md), not a literal translation. The Japanese is the source of
truth for *content* (what is built, in what order, with which code, and which claims about
real Rigor); the English re-authors the *prose* so it reads as if written in English from
the start. When the two would fight, keep the content and rewrite the sentence.

This guide follows the model established by the sibling project
[php-ministan](https://github.com/) (`book/en/STYLE.md`): same transcreation stance, same
typography discipline, same honesty about reference gaps — adapted to Ruby, Rigor, and
chibirigor's two-voice *Little/Seasoned* framing.

## What is and isn't translated

chibirigor's running text is Japanese; the code and its surface artifacts are mostly
language-neutral, which keeps the translation surface small:

- **Translated:** chapter prose, the front matter ([README.md](README.md)), the two volume
  intros ([`little/README.md`](little/README.md), [`seasoned/README.md`](seasoned/README.md)),
  the [glossary](glossary.md), the appendices, figure captions/labels (the SVGs under
  [`figures/`](figures/)), the reference-reading notes (re-aimed for English readers — see
  below), and **the comments inside the code excerpts printed in the prose** (see the caveat
  below).
- **English already — shared as-is:** code identifiers and type names (`Const`, `Nominal`,
  `Dynamic`, `Union`, `HashShape`, `Tuple`, `Scope`, …), the public command names (`check`,
  `annotate`), the function names (`type_of`, `accepts`), and the `untyped` surface token.
  These are names in the codebase, never translated.
- **Transcreated in printed excerpts:** the **comments** inside code excerpts. The Japanese
  edition prints Japanese comments; the English edition prints English. The underlying code is
  shared and **English-canonical** (below), so each edition translates only the comments it shows.
- **English already — shared as-is in both editions:** the **diagnostic / CLI message strings**
  (`expected Integer but got …`, `wrong number of arguments for …`, `fell to untyped here …`,
  `No type errors`) and the `trace` output. The tool emits these in English; both editions show
  them verbatim (not per-edition-translated).

> **Resolved (2026-06-16): the shared code is English-canonical.** The shared Ruby — `lib/`,
> `exe/` (CLI diagnostics), `examples/`, `tools/`, the per-chapter sources `impls/steps/`, and the
> generated snapshots `impls/dist/` + `book/v1/ja/little/examples/dist/` — was migrated so that
> **comments, user-facing diagnostic/CLI strings, and the `trace` output are English** (matching
> php-ministan's model). The tool now emits English; a click-through to any `impls/dist/partN`
> snapshot shows English for both editions. Each book then translates only the **comments** in the
> excerpts it prints — `book/v1/ja/` shows Japanese comments, `book/v1/en/` English — while the
> diagnostic/CLI strings shown are the shared English. The Seasoned design sketches live per
> edition (`book/v1/<ed>/seasoned/examples/*.rb`), identical English code. (Still Japanese, out of
> this migration's scope: the experimental `impls/rust/` port and `impls/README.md`, neither
> click-through targets. Tooling follow-up: `check_docs.rb`'s byte-match `<!-- include: -->` no
> longer matches the JA prose's Japanese-commented excerpts against the English fixtures — see the
> handoff ledger.)

## Directory & path mapping

The English tree mirrors the Japanese one exactly:

| Japanese | English |
|----------|---------|
| `book/v1/ja/README.md` | `book/v1/en/README.md` |
| `book/v1/ja/glossary.md` | `book/v1/en/glossary.md` |
| `book/v1/ja/little/partN-*.md` | `book/v1/en/little/partN-*.md` |
| `book/v1/ja/seasoned/partN-*.md` | `book/v1/en/seasoned/partN-*.md` |
| `book/v1/ja/appendix/aN-*.md` | `book/v1/en/appendix/aN-*.md` |
| `book/v1/ja/figures/svg/*.svg` | `book/v1/en/figures/svg/*.svg` |

Because the depth is identical (`book/v1/en/little/partN.md` is four levels deep), the
relative links are the same tokens as the Japanese side:

- Figures: `../figures/svg/little-N-1.svg` (and `seasoned-N-1.svg`).
- Sibling chapter / glossary / front matter: `part2-method-dispatch.md`, `../glossary.md`,
  `../README.md`, `../appendix/a1-special-types.md`.
- Code snapshot (end-of-chapter link): the **same** GitHub URL the Japanese uses,
  `https://github.com/rigortype/chibirigor/tree/master/impls/dist/partN/lib` (see the
  open-question note above on its comment language).

## Front matter (Starlight)

chibirigor ships to an Astro + Starlight site, so **every chapter and intro carries YAML
front matter** (`title`, `description`, `sidebar.order`) — unlike php-ministan, whose
chapters have none. The English edition keeps the front matter, with `title`/`description`
transcreated into English and the **same `sidebar.order`** as the Japanese file:

```yaml
---
title: Part 1 — Literals and arithmetic
description: Build the smallest machine that gives values a type, and run check and annotate.
sidebar:
  order: 2
---
```

The H1 immediately below repeats the chapter title in the book's running form:
`# The Little chibirigor Part N — <title>` (or `The Seasoned chibirigor Part N — …`).

## Voice & tone

The *Little/Seasoned* framing — borrowed from Scheme's *The Little Schemer* / *The Seasoned
Schemer* — is already English and lands cleanly; don't localize it away. Hold the original's
register:

- **The Little chibirigor** (前編, the basics) — "grow a working thing." Start from the
  smallest running pieces and add **one hard idea per chapter**. Keep the forward momentum
  ("next chapter we add…"). Code leads; theory waits.
- **The Seasoned chibirigor** (後編, the advanced volume) — "read the back of it." Re-read
  what the Little volume built in the language of formal type theory. Concepts lead; code is
  a sketch.
- **Reference-book tone, not breezy tutorial** — but warm and plain. Calm, precise,
  second-person plural ("we build…"). It explains *why* a design is the way it is, then
  builds it. The Japanese is friendly and casual without being childish; match that — natural
  English, not *more* English. Don't pad: the Japanese is dense and economical.
- **The spine is non-rejecting / "never frighten working code."** Every chapter ties back to
  it: don't flag working code; collapse what you can't determine to `untyped`; stay silent
  when unsure. Keep that thread explicit.
- **Inference is the foundation, not a pre-pass.** chibirigor is *an inference-driven type
  checker*: it **synthesizes** a type up from each expression; it does **not** work backward
  from call sites to fill in a method's argument types (that is TypeProf's job, and the book
  says so). Never frame it as "an inferencer, with checking as a by-product" — checking and
  display both *consume* the inferred type.
- **No theory as a prerequisite (Little volume).** Type-theory vocabulary is introduced *as
  the thing that needs it is built*, never assumed. Equations and proofs stay out of the
  Little volume and are handled head-on in the Seasoned volume.

### The three voices (三題噺)

Each chapter is written as a small **three-voice piece** — the Japanese calls it 三題噺
(*sandai-banashi*, a rakugo turn improvised from three given topics). Render it in English as
**"the three voices"** and keep the three strands and their circled numbers:

1. **① Type theory** — the one concept the chapter meets (and where *The Mechanics of Type
   Systems* covers it, when it does).
2. **② In Ruby / RBS** — how that concept looks in Ruby, or *fails* to show up.
3. **③ Rigor's implementation problem** — why the naive implementation breaks against Ruby's
   reality, and how it was reconciled.

"Understanding Rigor" means watching the trouble in ③ arise *necessarily* from ② (Ruby's
reality) and settle *gently* under the concept in ①. Keep the circled numerals `①②③`
(U+2460…), matching the Japanese.

## Terminology (canonical English)

Code identifiers (`Const`, `Dynamic`, `Union`, `type_of`, `accepts`, `Scope`, …) are never
translated — they are names in the codebase. The conceptual vocabulary:

| Concept (JA) | Canonical English | Notes |
|---|---|---|
| 軸: 拒まない／受理寄り | **non-rejecting** | accept any code Ruby doesn't reject as a syntax error |
| never frighten working code | **kept verbatim** | Rigor's motto; already English |
| 型キャリア | **type carrier** | a Ruby object that represents a type (`Const`, `Nominal`, …) |
| `Dynamic`／untyped | **`untyped`** (the `Dynamic` carrier) | "lost the type — stay silent"; the heart of gradual typing |
| 抽象構文木 | **AST** / abstract syntax tree | spell out once on first use; Ruby's parser is **Prism** |
| 推論（合成） | **inference** / **synthesis** | building a type *up* from an expression (`type_of`) |
| 丸め／正規化 | **rounding** / **normalization** | collapse a fine type (`Const[3]`) back to a coarse one (`Integer`) |
| 拡大 | **widening** | drop precision when a `Const`/`Union` exceeds its budget (cf. abstract interpretation) |
| ユニオン型 | **union type** (`Union`) | "either `A` or `B`"; `Integer | String` |
| ナローイング／絞り込み | **narrowing** | tighten a variable's type per branch; verb "narrow" |
| ディスパッチ | **dispatch** | look up a return type from receiver + method name |
| 三値（`:yes`/`:no`/`:maybe`） | **three-valued** (trinary) | the verdict of `accepts`; `:maybe` is never punished |
| 受理判定 | **acceptance check** (`accepts`) | does this type fit where that type is expected |
| 部分型 | **subtype** / **supertype** (`<:`) | "the value fits in the expected type's box" |
| `HashShape`（レコード型） | **record type** (`HashShape`) | per-key types; Hack `shape(...)` → PHPStan/Psalm → Rigor |
| `Tuple` | **tuple** | an array remembered by per-position type |
| 漸進的型付け | **gradual typing** | mixing typed and untyped code (Siek & Taha, 2006) |
| ロバストネス原則 | **robustness principle** (Postel's law) | strict in what you return, liberal in what you accept |
| FactStore | **FactStore** | the flow-sensitive store of "facts" (naive in P5, full in Seasoned P6) |
| 健全性 | **soundness** | progress + preservation; a typed program won't hit undefined behavior |
| 双方向型付け | **bidirectional typing** | synthesis `⇒` vs. checking `⇐` |
| 変性 | **variance** | covariant returns, contravariant parameters |
| 型代入／置換 | **substitution** | put a type into a type variable (System F) |
| 再帰型（μ型）／余帰納 | **recursive type (μ)** / **coinduction** | self-referential types and their equivalence |
| 単一化 | **unification** | find the substitution that makes two types equal |
| HKT（高階型） | **higher-kinded type** (`App[F, A]`) | a type that takes a type and returns a type |
| erasure（境界での丸め） | **erasure** | drop internal precision at the RBS boundary — *not* Java's runtime type-erasure |
| RBS interface | **RBS interface** / **structural interface** | gloss "structural" on first use to head off the Java reading |
| 構造的契約 | **structural typing** | shape, not name |

When a term first appears, gloss it once (e.g., "narrowing — tightening a variable's type per
branch") and then use it bare. Keep `untyped` as the surface word readers see; `Dynamic` is
its carrier in code.

## House typography

- **Em dash:** a single em dash with surrounding spaces — ` — ` (U+2014). This mirrors the
  Japanese double-dash rhythm (the Japanese side uses a doubled `——`; English uses one),
  wraps cleanly on screen, and is easy to verify. Do **not** use the unspaced `word—word`
  form, the en dash, or `―` (U+2015) for this purpose.
- **Quotes:** curly quotes “ ” and ‘ ’ in prose. Keep **straight** quotes inside code spans
  and inside raw-HTML attribute values (the `<picture>`/`<img>` figure embeds, whose
  `media="…"` / `src="…"` / `alt="…"` must stay straight or the image won't load). When
  bulk-converting prose to curly quotes, protect both code spans and HTML tags.
- **Code:** inline code in backticks; identifiers, file paths, and CLI text always in code.
- **Circled numerals:** `①②③…` (U+2460…) for the three voices and step numbering — never the
  dingbat `➀` (U+2780) or reversed `❶` (U+2776) forms. Matches the Japanese.
- **Chapter labels:** "Part 0…Part 9" for the Little volume; "Part 1…Part 8" for the Seasoned
  volume (matching the Japanese and the project's chapter numbering).

## Chapter header notes

Each chapter opens the way the Japanese does, transcreated:

1. **Goal line / intro blockquote** (every chapter): the one-paragraph "what this chapter
   builds" the Japanese opens with.
2. **Reference-reading note** (optional; only chapters with real type-theory content — skip
   the theory-light ones, mirroring the Japanese policy).

End each chapter the way the Japanese does: **goal → body (the three voices) → summary table →
`## Exercises` → next-chapter teaser → end-of-chapter snapshot link.**

## Reference apparatus (re-aimed for English readers)

The Japanese edition pairs Pierce's **TAPL** with 遠藤侑介『型システムのしくみ』(*The
Mechanics of Type Systems*) — a build-a-**type-checker** companion that is **Japanese-only**,
and to which chibirigor is unusually close (the two are "two sides of the same coin": 『しくみ』
builds a checker for a typed mini-language, chibirigor builds one for untyped Ruby). An
English reader can't use 『しくみ』, and there is no English book of the same genre and
calibre, so the English edition does **not** force a substitute. Reaching for a
build-an-*interpreter* book (e.g. *Crafting Interpreters*) would be a false equivalence:
chibirigor is about type checking and type theory, not interpretation. The build-along
*ethos* is already credited to the book's stated model, chibivue. The apparatus is therefore
lean and honest:

- **TAPL** — Benjamin C. Pierce, *Types and Programming Languages* (MIT Press). The shared
  type-theory reference; **same chapter pointers as the Japanese edition** (the Japanese
  cites the Ohmsha translation; English readers go straight to the original).
- **Topical sources** where a chapter genuinely needs them: Siek & Taha, *Gradual Typing for
  Functional Languages* (2006) for the non-rejecting / gradual stance; PHPStan and Psalm
  design writing where an analyzer decision is at stake (Rigor descends from the
  Hack→PHPStan→Psalm lineage of shape/union/narrowing design).
- **Be honest about gaps.** Where 『しくみ』 was the *only* correspondence (its untagged-union
  treatment, its "afterword" frontier — union types, three-valued logic, gradual typing,
  flow-sensitive scope, the very areas chibirigor "pushes past" 『しくみ』), and no clean
  English equivalent exists, say so plainly rather than forcing a citation. The Japanese
  edition's candor about "no corresponding chapter" is part of the tone — keep it.

## Fidelity to real Rigor (don't drift)

chibirigor is a deliberate *simplification* of real Rigor, but every factual claim it makes
about real Rigor ("in real Rigor…", the ③ voice, "Rigor does X") must be true. The Japanese
side verifies these against the read-only Rigor checkout at
`/Users/megurine/repo/ruby/rigor` (`docs/handbook/`, `docs/type-specification/`,
`docs/internal-spec/`, `docs/adr/`, `lib/rigor/`). **Transcreation must not invent or soften
a Rigor claim** — if the English would read better with a sharper claim, check it first or
keep the Japanese's hedge. Intentional simplifications (local Scope vs. the full FactStore,
no HKT, untyped arguments, …) are *correct reductions*, not drift; only accidental
inaccuracies are bugs.

## Sync with the Japanese source

`book/v1/ja/` is the source of truth. When a Japanese chapter changes, **re-transcreate** the
matching English one rather than diffing word-for-word. Note the last-synced Japanese commit
in the commit message when a chapter lands.

## Review

The `chibirigor-review` skill currently targets `book/v1/ja/` (a 10-lens, 4-layer battery:
真 truth → 伝 teaching → 読 reading-balance → 整 polish). For the English edition, the model
to follow is php-ministan's **English read-feel battery** — three standing lenses run per
English chapter, weighting *read-feel as a native, well-edited English technical book* above
all:

- a **veteran US technical-book editor** (1990s–2010s) — economy, signposting,
  promise→payoff, active voice, buried subjects, antecedents, rhythm;
- a **modern technical-book editor** (2016–) — screen-first scannability, voice temperature,
  translation-smell, whether the *Little/Seasoned* framing still lands for a 2020s dev;
- an **outside FP researcher** (non-native English) — international read-feel *and*
  type-theory soundness (subtyping, three-valued logic, lattices, gradual typing).

Until that mode is wired into the skill, run the **language-neutral lenses** directly on the
English text — reproducibility (re-implement from the **English** prose and grade it against
reference behavior) and Rigor-fidelity — plus a native-English copyedit pass. The forbidden
moves are the same as the Japanese battery's: no demand for feature parity with real Rigor,
no "be stricter / catch more," no "add chapters." Intentional simplification is not a defect.
Notes go to `book/v1/en/.reviews/_<lens>-review.md` (gitignored).

## Per-chapter verification (prose-only changes)

English chapters are prose; they don't touch `lib/` or the tests, so the Ruby test gate
doesn't apply. After writing a chapter, check:

- **Links resolve** — every `](path)` target exists (forward links to unwritten chapters are
  allowed while the edition is in progress).
- **Em dash** — only the spaced ` — ` form; no `—word` / `word—`, no `――` / `―` / en-dash
  misuse. Verify: `grep -nP '\S—|—\S'` over the prose (excluding code spans/HTML) is empty.
- **Figures** — referenced SVGs exist under `figures/svg/`; if you added one, render it to
  PNG and eyeball it. chibirigor figures are **single-variant** (no light/dark split, unlike
  ministan); translate the text only and reuse the colors and geometry.
- **Terminology** — terms match the table above; code identifiers untranslated; the three
  voices keep `①②③`.
