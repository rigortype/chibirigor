# chibirigor v2/en — modern technical-book editor review (screen-first, voice, translation-smell)

Reviewer lens: a 2016–present technical-book editor reading for screen-first scannability, voice
temperature (warm-but-precise), and translation-smell / calques. Source of truth is `book/v2/ja`;
voice anchor is `book/v1/en`. House typography (spaced ` — ` em dash, `①②③`, `『しくみ』`, "the three
perspectives," TAPL/ADR refs) is respected and never flagged.

## Headline

The edition reads **strongly** and almost always idiomatically. The Little volume in particular
(Part 0, Part 1, Part 5, Part 7, appendix a1, a5) is among the warmest, most natural English in any
transcreated tech book of this kind — the three-perspectives framing lands cleanly, the GitHub-alert
blocks are well-titled and well-toned, and the v1→v2 evolution (three voices → three perspectives,
blockquotes → `[!NOTE]/[!TIP]/[!IMPORTANT]`) is consistent and successful.

The defects worth fixing are a **small, recurring set of calques** concentrated in the Seasoned
volume and a couple of dead/literal metaphors. None affect correctness or the pedagogy; all are
"an English dev wouldn't write it this way" register issues. They cluster, so a single sweep fixes
most of them.

No ERROR-severity (factual/structural) findings. Everything below is IMPROVE or NIT.

---

## IMPROVE

### 1. Calque: "on-the-ground code stops turning over"
`little/part9-gradual-philosophy.md:71-72`
> **② In Ruby:** types bolted onto a dynamic language. Too strict and on-the-ground code stops
> turning over.

"stops turning over" is a literal rendering of 現場のコードが回らなくなる. In English "turn over"
(of code) carries no "won't run / won't ship" meaning — it reads as a non-sequitur. Suggested:
> Too strict, and real-world code grinds to a halt.

or "…and real production code stops shipping." (The book elsewhere renders 現場 well as "on the
ground" / "real-world," so only the verb is the problem.)

### 2. Calque: "a point sent from Seasoned Part X §Y"
`seasoned/part7-soundness.md:123` and `seasoned/part7-soundness.md:215`
> This is called **gradual consistency** `~` (a point sent from Seasoned Part 2 §2-5):
> … + a **fuel budget** the alternative (a point sent from Seasoned Part 4 §4-5).

Literal of 〜から送られてくる論点. "a point sent from" reads mechanical. The book *already* handles this
metaphor naturally elsewhere — "gradual's guarantee **goes to** Seasoned Part 7" (part2:248),
"the formal theory is **handed to** the Seasoned volume" (little/README:13). Match that register:
> …gradual consistency `~` (the thread we picked up in §2-5):
> …(carried over from §4-5).

or "(set up back in §2-5)".

### 3. Calque: "entered lib" / "promoted into the chibirigor body"
`seasoned/part3-generics-and-substitution.md:204, 207, 212, 233-234`;
`seasoned/part5-real-inference.md:269, 271`
> ## 3-6x. A note: reading the element type entered lib (generics 5a)
> Lib-ifying generics proceeds in three stages…
> …has been promoted into the chibirigor body (`element_read` …)
> …the **push-down (5b)** … entered lib too;

Literal of libに入った / chibirigor本体に昇格. "entered lib" and "promoted into the body" read as
machine-translated status verbs. An English dev says a feature *graduated from sketch into the
codebase* / *now lives in lib* / *made it into chibirigor proper*. Suggested headings/sentences:
> ## 3-6x. A note: reading the element type now lives in lib (generics 5a)
> Moving generics into lib proceeds in three stages…
> …has graduated into chibirigor's codebase (`element_read` …)
> …the **push-down (5b)** … made it into lib too;

"the chibirigor body" specifically should become "chibirigor proper" or "chibirigor's lib" — "body"
for 本体 is a false friend here.

### 4. Calque: "soiling the core" / "core-soiling"
`seasoned/part8-toward-rigor.md:48, 71`; `appendix/a3-tooling.md:332`
> …from outside without soiling the core …
> …Writing directly into it causes *core-soiling* problems …
> …**soiling not one line of the code** copied in the main volume …

Literal of コアを汚す. "soil" for "pollute/contaminate/dirty" reads archaic and slightly off in a
software context. English devs say "polluting the core," "contaminating the core," "without touching
the core," or "not dirtying a single line." Suggested:
> …from outside without polluting the core …
> …causes *core-pollution* problems …
> …**without touching a single line** of the code copied in the main volume …

---

## NIT

### 5. "terrain of its own" / "own terrain" (recurring noun phrase)
`little/part5-narrowing.md:219, 242`; `seasoned/part6-fact-store.md:12`;
`appendix/a4-bibliography.md:70, 71, 96, 101`
> type information grows by case (terrain of its own that 『しくみ』 doesn't cover…)
> …this is the own terrain of gradual, practical checkers.
> …are **own terrain** with no…

Calque of 独自の領分 / 独擅場. "own terrain" / "the own terrain" reads slightly translated as a bare
noun phrase (and "the own terrain" at part6:12 is ungrammatical — "own" needs a possessive). It
recurs consistently, so it has become a quasi-term; if kept, at least fix "the own terrain" →
"the home turf of" / "territory unique to." Natural alternatives: "ground chibirigor breaks on its
own," "territory unique to gradual checkers," "where 『しくみ』 has no chapter — this is chibirigor's
own ground." Low priority because it's consistent and the meaning is clear, but it's the most visible
remaining translation tic after #1–#4.

### 6. "the spine of the book stays these two" / metaphor density (monitor, not a defect)
`little/part0-introduction.md:45`, README:33, and throughout. The book leans hard on a few stock
metaphors — "doorway," "spine," "skeleton/flesh," "the map," "the bookend," "climax." Individually
each reads fine and they are part of the voice; flagged only as something to watch so a future pass
doesn't let "climax" (used for nearly every Seasoned chapter's core section: part1:296, part2:19,
part3:21, part7-style) flatten into filler. No change required now.

### 7. Seasoned cross-reference sentences run long on a phone screen
`seasoned/part1-bidirectional-typing.md:176-197` (the §1-6 "volume's map" bullets),
`seasoned/part5-real-inference.md:160-191` (the nested `[!NOTE]` with ①②③ sub-problems).
These are correct and well-structured for desktop, but several bullets exceed ~3 lines on a narrow
viewport and stack clauses ("Adjacent to Part 3, tying up the mutual reference of α-equivalence.").
Screen-first nit only: where a bullet carries two ideas, consider splitting. Not blocking.

---

## What reads clean (explicitly)

- **Part 0** (both volumes): the two-promises setup, the data-flow ASCII figure, the `[!IMPORTANT]`
  core-principle box — all idiomatic and well-paced.
- **The three-perspectives framing** (①②③) lands consistently across every chapter's closing table
  and lead bullets. It reads as a genuine teaching device, not a gimmick, for a 2020s dev.
- **Little/Seasoned framing** ("build it" / "read it") is crisp and reinforced in every README and
  chapter footer.
- **GitHub-alert blocks**: titles and tone are well-judged throughout (e.g. "Don't know" comes in
  two opposite schools" at part1:70; the `param:`/`assert:` directive notes; the Sorbet/TypeProf
  comparison NOTEs). No alert block needs retitling or retoning.
- **Appendix a1, a5**: confident, idiomatic, genuinely warm-but-precise — model prose for the edition.
- **Tables** scan well and use the perspective/comparison structure consistently.

## Fix economy

Findings #1–#4 are the priority; they are a handful of distinct calque expressions, each occurring
1–4 times, fixable in one focused pass without touching surrounding structure. #5 is a judgment call
(keep-as-tic vs. naturalize) but at minimum the ungrammatical "the own terrain" should be fixed.
#6–#7 are watch-items, not edits.
