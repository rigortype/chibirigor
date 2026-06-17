# English edition — style & terminology guide (v2)

> This is a **contributor document**, not part of the reader's path. It records how the
> English edition of chibirigor (v2) is written so that every chapter reads as one book. If
> you are here to read chibirigor, start at [README.md](README.md).

The English edition is a **transcreation** of the Japanese original under
[`book/v2/ja/`](../ja/README.md), not a literal translation. The Japanese is the source of
truth for *content* (what is built, in what order, with which code, and which claims about
real Rigor); the English re-authors the *prose* so it reads as if written in English from
the start. When the two would fight, keep the content and rewrite the sentence.

This guide descends from the v1 English edition's [`book/v1/en/STYLE.md`](../../v1/en/STYLE.md)
(itself modelled on the sibling project php-ministan): same transcreation stance, same
typography discipline, same honesty about reference gaps. **v2 carries two deliberate changes
from v1** — see the call-outs below:

- **「三題噺」→「三つの視点」.** v1 framed each chapter as a rakugo *three-topic turn* (三題噺,
  rendered "the three voices"). v2 drops the rakugo metaphor for the plainer **three
  perspectives** (三つの視点 / パースペクティブ). The English follows: render it **"the three
  perspectives,"** never "three voices."
- **Columns are now GitHub alerts.** v2 writes its columns/notes as GitHub alert blockquotes
  (`> [!NOTE]` / `> [!TIP]` / `> [!IMPORTANT]`) rather than plain `>` blockquotes. The English
  mirrors the *same alert kind on the same block*. See "House typography" below.

## What is and isn't translated

chibirigor's running text is Japanese; the code and its surface artifacts are mostly
language-neutral, which keeps the translation surface small:

- **Translated:** chapter prose, the front matter ([README.md](README.md)), the two volume
  intros ([`little/README.md`](little/README.md), [`seasoned/README.md`](seasoned/README.md)),
  the [glossary](glossary.md), the appendices, and figure captions/labels (the SVGs under
  [`figures/`](figures/)).
- **English already — shared as-is:** code identifiers and type names (`Const`, `Nominal`,
  `Dynamic`, `Union`, `HashShape`, `Tuple`, `Scope`, …), the public command names (`check`,
  `annotate`), the function names (`type_of`, `accepts`), and the `untyped` surface token.
  These are names in the codebase, never translated.
- **Transcreated in printed excerpts:** the **comments** inside code excerpts. The Japanese
  edition prints Japanese comments; the English edition prints English. The underlying shared
  code is **English-canonical** (below), so the English comments usually match the shared
  source verbatim.
- **English already — shared as-is in both editions:** the **diagnostic / CLI message strings**
  (`expected Integer but got …`, `wrong number of arguments for …`, `fell to untyped here …`,
  `No type errors`) and the `trace` output. The tool emits these in English; both editions show
  them verbatim (not per-edition-translated).

> [!NOTE]
> **The shared code is English-canonical** (settled in v1, 2026-06-16, and inherited by v2).
> The shared Ruby — `lib/`, `exe/` (CLI diagnostics), `examples/`, `tools/`, the per-chapter
> sources `impls/steps/`, and the generated snapshots `impls/dist/` + the book's
> `little/examples/dist/` — has its comments, user-facing diagnostic/CLI strings, and `trace`
> output **in English**. A click-through to any `impls/dist/partN` snapshot shows English for
> both editions. v2's `book/v2/ja/little/examples/*.rb` are byte-identical to v1's
> (English-canonical), so the English edition reuses them as-is.

## Directory & path mapping

The English tree mirrors the Japanese one exactly:

| Japanese | English |
|----------|---------|
| `book/v2/ja/README.md` | `book/v2/en/README.md` |
| `book/v2/ja/glossary.md` | `book/v2/en/glossary.md` |
| `book/v2/ja/little/partN-*.md` | `book/v2/en/little/partN-*.md` |
| `book/v2/ja/seasoned/partN-*.md` | `book/v2/en/seasoned/partN-*.md` |
| `book/v2/ja/appendix/aN-*.md` | `book/v2/en/appendix/aN-*.md` |
| `book/v2/ja/figures/svg/*.svg` | `book/v2/en/figures/svg/*.svg` |

The relative links are the same tokens as the Japanese side (identical depth):

- Figures: `../figures/svg/little-N-1.svg` (and `seasoned-N-1.svg`).
- Sibling chapter / glossary / front matter: `part2-method-dispatch.md`, `../glossary.md`,
  `../README.md`, `../appendix/a1-special-types.md`.
- Code snapshot (end-of-chapter link): the **same** GitHub URL the Japanese uses,
  `https://github.com/rigortype/chibirigor/tree/master/impls/dist/partN/lib`.

**Figures are reused from v1/en.** Every v2 SVG is byte-identical to its v1 counterpart, and
v1/en already ships translated SVGs — they were copied straight into `figures/svg/`. If a v2
figure ever diverges from v1, re-translate the text only and reuse the colors/geometry
(chibirigor figures are single-variant — no light/dark split).

## Front matter (Starlight)

chibirigor ships to an Astro + Starlight site, so **every chapter and intro carries YAML
front matter** (`title`, `description`, `sidebar.order`). The English edition keeps it, with
`title`/`description` transcreated into English and the **same `sidebar.order`** as the
Japanese file:

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

Note the Japanese uses a full-width space (`　`, U+3000) between "Part N" and the title in
both the front-matter `title` and the H1; the English uses the spaced em dash ` — `.

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

### The three perspectives (三つの視点)

Each chapter is written from a small set of **three perspectives** — the Japanese calls them
三つの視点 (パースペクティブ). Render it in English as **"the three perspectives"** and keep
the three strands and their circled numbers:

1. **① Type theory** — the one concept the chapter meets (and where *The Mechanics of Type
   Systems* covers it, when it does).
2. **② In Ruby / RBS** — how that concept looks in Ruby, or *fails* to show up.
3. **③ Rigor's implementation problem** — why the naive implementation breaks against Ruby's
   reality, and how it was reconciled.

"Understanding Rigor" means watching the trouble in ③ arise *necessarily* from ② (Ruby's
reality) and settle *gently* under the concept in ①. Keep the circled numerals `①②③`
(U+2460…), matching the Japanese. (v1 called this "the three voices / 三題噺"; v2 renamed it —
do not carry the old wording over.)

## Terminology (canonical English)

Code identifiers (`Const`, `Dynamic`, `Union`, `type_of`, `accepts`, `Scope`, …) are never
translated — they are names in the codebase. The conceptual vocabulary:

| Concept (JA) | Canonical English | Notes |
|---|---|---|
| 軸: 拒まない／受理寄り | **non-rejecting** | accept any code Ruby doesn't reject as a syntax error |
| 動くコードを脅かさない | **never frighten working code** | Rigor's motto; kept verbatim |
| 型キャリア | **type carrier** | a Ruby object that represents a type (`Const`, `Nominal`, …) |
| `Dynamic`／untyped | **`untyped`** (the `Dynamic` carrier) | "lost the type — stay silent"; the heart of gradual typing |
| 抽象構文木 | **AST** / abstract syntax tree | spell out once on first use; Ruby's parser is **Prism** |
| 推論（合成） | **inference** / **synthesis** | building a type *up* from an expression (`type_of`) |
| 丸め／正規化 | **rounding** / **normalization** | collapse a fine type (`Const[3]`) back to a coarse one (`Integer`) |
| 拡大 | **widening** | drop precision when a `Const`/`Union` exceeds its budget (cf. abstract interpretation) |
| ユニオン型 | **union type** (`Union`) | "either `A` or `B`"; `Integer \| String` |
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
| 無タグのUnion | **untagged union** | a union with no discriminating tag (cf. tagged/variant) |

When a term first appears, gloss it once (e.g., "narrowing — tightening a variable's type per
branch") and then use it bare. Keep `untyped` as the surface word readers see; `Dynamic` is
its carrier in code.

## House typography

- **Em dash:** a single em dash with surrounding spaces — ` — ` (U+2014). This mirrors the
  Japanese double-dash rhythm (the Japanese side uses a doubled `——`; English uses one),
  wraps cleanly on screen, and is easy to verify. Do **not** use the unspaced `word—word`
  form, the en dash, or `―` (U+2015) for this purpose. (En dash `–` is fine for numeric
  ranges like `20–21`.)
- **Full-width space (`　`, U+3000):** the Japanese titles/headers use it as a separator
  (`Part 0　はじめに…`); the English uses the spaced em dash ` — ` instead. Don't leave
  U+3000 in English text.
- **Quotes:** curly quotes “ ” and ‘ ’ in prose. Keep **straight** quotes inside code spans
  and inside raw-HTML attribute values. When bulk-converting prose to curly quotes, protect
  both code spans and HTML tags.
- **Code:** inline code in backticks; identifiers, file paths, and CLI text always in code.
- **Circled numerals:** `①②③…` (U+2460…) for the three perspectives and step numbering —
  never the dingbat `➀` (U+2780) or reversed `❶` (U+2776) forms. Matches the Japanese.
- **Chapter labels:** "Part 0…Part 9" for the Little volume; "Part 1…Part 8" for the Seasoned
  volume (matching the Japanese and the project's chapter numbering).

### GitHub alerts (v2 columns)

v2 renders its columns/notes/asides as GitHub alert blockquotes. The English keeps the **same
alert kind on the same block** as the Japanese. The roles, as used in the Japanese source:

- **`> [!NOTE]`** — background / context / history (e.g., Rigor's backstory, the TypeProf
  aside). Neutral "by the way" material.
- **`> [!TIP]`** — optional / reference-reading material (the 参考書 notes, the a4 pointer,
  how to check the exercises). Skippable extras.
- **`> [!IMPORTANT]`** — the spine: the core principle a chapter must not let the reader miss
  (the non-rejecting / never-frighten statement, "this chapter has no impl file," etc.).

Keep the alert keyword in English (`[!NOTE]`, never a translated label). A GitHub alert's
**first line after the keyword is its title** — transcreate a bolded title line the same way.
**Not every blockquote is an alert:** the chapter's opening **goal line** (the one-paragraph
"what this chapter builds"), **figure captions/footers** (`> ▼ Figure 0-1 …`), and other
plain leads stay **plain `>` blockquotes** — only convert a block to an alert if the Japanese
marks it as one.

## Chapter header notes

Each chapter opens the way the Japanese does, transcreated:

1. **Goal line / intro blockquote** (every chapter): the one-paragraph "what this chapter
   builds" the Japanese opens with — a plain `>` blockquote.
2. **Reference-reading note** (optional; only chapters with real type-theory content — skip
   the theory-light ones, mirroring the Japanese policy) — a `> [!TIP]` alert.

End each chapter the way the Japanese does: **goal → body (the three perspectives) → summary
table → `## Exercises` → next-chapter teaser → end-of-chapter snapshot link.**

## Reference apparatus (re-aimed for English readers)

The Japanese edition pairs Pierce's **TAPL** with 遠藤侑介『型システムのしくみ』(*The
Mechanics of Type Systems*) — a build-a-**type-checker** companion that is **Japanese-only**,
and to which chibirigor is unusually close (the two are "two sides of the same coin": 『しくみ』
builds a checker for a typed mini-language, chibirigor builds one for untyped Ruby). An
English reader can't use 『しくみ』, and there is no English book of the same genre and
calibre, so the English edition does **not** force a substitute. The apparatus is lean and
honest:

- **TAPL** — Benjamin C. Pierce, *Types and Programming Languages* (MIT Press). The shared
  type-theory reference; **same chapter pointers as the Japanese edition** (the Japanese
  cites the Ohmsha translation; English readers go straight to the original).
- **『しくみ』** — name it, gloss it as *The Mechanics of Type Systems* (Lambda Note), and
  say plainly that it is **Japanese only**. Keep the candor: where 『しくみ』 was the *only*
  correspondence and no clean English equivalent exists (untagged unions, three-valued logic,
  gradual typing, flow-sensitive scope — the areas chibirigor "pushes past" 『しくみ』), say
  so rather than forcing a citation.
- **Topical sources** where a chapter genuinely needs them: Siek & Taha, *Gradual Typing for
  Functional Languages* (2006); PHPStan and Psalm design writing (Rigor descends from the
  Hack→PHPStan→Psalm lineage of shape/union/narrowing design).

## Fidelity to real Rigor (don't drift)

chibirigor is a deliberate *simplification* of real Rigor, but every factual claim it makes
about real Rigor ("in real Rigor…", the ③ perspective, "Rigor does X") must be true. The
Japanese side verifies these against the read-only Rigor checkout at
`/Users/megurine/repo/ruby/rigor`. **Transcreation must not invent or soften a Rigor claim** —
if the English would read better with a sharper claim, check it first or keep the Japanese's
hedge. Intentional simplifications (local Scope vs. the full FactStore, no HKT, untyped
arguments, …) are *correct reductions*, not drift; only accidental inaccuracies are bugs.

Note v2 adds material v1 lacked (e.g., Part 0's "Rigor's backstory" column citing RubyKaigi
2026 in Hakodate). Translate such additions faithfully; don't import v1/en wording where v2
has rewritten the passage.

## Sync with the Japanese source

`book/v2/ja/` is the source of truth. When a Japanese chapter changes, **re-transcreate** the
matching English one rather than diffing word-for-word. v1/en is an excellent **terminology
and voice anchor**, but v2's prose is substantially rewritten — transcreate from v2/ja, then
cross-check wording against v1/en. Note the last-synced Japanese commit in the commit message
when a chapter lands.

## Per-chapter verification (prose-only changes)

English chapters are prose; they don't touch `lib/` or the tests, so the Ruby test gate
doesn't apply. After writing a chapter, check:

- **Links resolve** — every `](path)` target exists (forward links to unwritten chapters are
  allowed while the edition is in progress).
- **Em dash** — only the spaced ` — ` form; no `—word` / `word—`, no `――` / `―` misuse.
  Verify: `grep -nP '\S—|—\S'` over the prose (excluding code spans/HTML) is empty.
- **No stray full-width space** — `grep -nP '\x{3000}'` over the English prose is empty.
- **Figures** — referenced SVGs exist under `figures/svg/` (reused from v1/en).
- **Alerts** — each `> [!NOTE]/[!TIP]/[!IMPORTANT]` matches the Japanese block's kind; plain
  leads/captions stay plain.
- **Terminology** — terms match the table above; code identifiers untranslated; the three
  perspectives keep `①②③`; never "three voices."
