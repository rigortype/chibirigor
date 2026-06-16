# Ruby-only reader review — The Little chibirigor (English edition)

**Reviewer seat:** a Rails-fluent junior whose *entire* mental model is "type ≈ class." I know
`String`/`Integer`/`Array`/`Hash`/`Data.define`, `nil`, `if`, exceptions, blocks. I have **never
written a type annotation** and have **no picture of "compile-time match/mismatch."** I can't read
TypeScript syntax (`x: number`, `A | B`) as a foothold. I read Part 0 → Part 9 in order.

## Verdict

**The Little volume lands.** For a reader like me it mostly holds — and at the one place that
would normally break me hardest (the very first time a "type" is something other than a class), the
book stops and *flips the switch by hand* (P1 §1-1: "a type is **not** a class like `Integer`
itself; it's *data* that represents a type"). That single sentence is the spine that carries the
whole "type ≈ class" reader through Union, HashShape, and the special types. The TypeScript
comparisons are, without exception I found, fenced as optional bonuses and *always paired with a
Ruby restatement* — they never act as a foothold I'm supposed to already get. No BLOCKs.

The remaining issues are FRICTION (I snag but the surrounding prose recovers me) and a few
nitpicks — mostly English phrasing that buries the point, plus two spots where the *act* of
type-annotation is leaned on a half-beat before it's grounded.

### Two places the "type ≈ class" footing is handled *well* (keep these)

1. **P1 §1-1 — "flip one switch in your head."** This is the load-bearing moment of the whole book
   for my profile. It (a) names my exact wrong model, (b) corrects it ("a type is *data*"), and
   (c) immediately motivates *why* with `Const[1]` ("fine types you can't write as a class"). It
   even lands on `Data.define`, which I use daily — so the unfamiliar idea ("a type is a value")
   rides in on familiar machinery. Textbook handling.
2. **P3 §3-1 — Scope as "that mental note turned into data."** "See `x = 1`, remember 'x is a
   number'; a few lines down recall 'that number from before' — Scope is that *mental note* turned
   into data." It explains a type environment with zero jargon by pointing at something I already
   do in my head. The immutability motivation ("a note for this branch only … without fouling the
   original") is also grounded before it's needed. Excellent.

Honorable mention: **P5 §5-1 column** turns null-safety from a foreign concept into "catching
crash-on-nil in the types" — a thing every Rubyist has been bitten by.

---

## Findings

### FRICTION

**F1 · little/part0-introduction.md §0.1 · annotation assumed a half-beat before it's defined**
- *The break (reader-voice):* The section opens "Most type checkers assume *type annotations
  written into the program* and decide whether those annotations contradict one another." I've
  never written an annotation and don't yet know what "contradict" means for one, so the opening
  sentence asks me to picture a workflow I've literally never done. The parenthetical that defines
  "type annotation" (*writing a type into the code*, "this variable is an `Integer`") comes
  *right after*, so I recover — but for one sentence the lead clause floats.
- *The fix:* Lead with the definition. Move the parenthetical's "writing a type into the code, as
  in 'this variable is an `Integer`'" gloss *into or ahead of* the first sentence, e.g. "Most type
  checkers assume the programmer has *written types into the code* — 'this variable is an
  `Integer`' — and check whether those written types contradict one another." Then I never float.

**F2 · little/part8-rbs-and-signatures.md §8-1 · "the first type you ever write" deserves more air**
- *The break (reader-voice):* The text correctly flags "this may be the first notation in which
  you've 'written' a type" — good. But this is *the* milestone for my whole profile (the act I've
  never performed), and it's read out in a single dense paragraph: `def +: (Integer) -> Integer`,
  "right of `:` is the type, `(...)` the argument types, right of `->` the return type," *plus* a
  side-note that `->` collides with Ruby's lambda arrow. That's three new things stacked on the one
  moment I most need to go slow. I get it on a second read, but the first pass is a gulp.
- *The fix:* Give it room. Split into two beats: first just `def to_s: () -> String` ("no args,
  returns String") as the simplest possible shape, *then* `def +: (Integer) -> Integer`. The
  lambda-arrow collision is worth a callout but should be its own aside, not crammed into the same
  breath as the first reading. Even a one-line "read it like a Ruby `def` header with the types
  penciled in, nothing more" *before* the symbol-by-symbol breakdown would settle me first.

**F3 · little/part4-union.md §4-1 (lead blockquote) · "untagged union / tagged variants" lands before I can use it**
- *The break (reader-voice):* The chapter's opening blockquote says the Union we build is an
  "**untagged union**" that the reference books "deliberately avoided" because they hold "*tagged*
  variants — values labeled with a tag to tell them apart." I've just barely accepted "a type can
  be data"; now, before I've even seen `Union` defined, I'm told there's a tagged-vs-untagged axis
  and that ours is the untagged kind. I have no hook for "tagged variant" yet. It's positioned as
  context ("for the difference see appendix a5-4"), so I can skip it — but it sits *above* the
  definition, so it reads as if I'm supposed to hold it going in.
- *The fix:* Move this blockquote *below* §4-1's `Union` definition and the `1 | "a"` example, so
  I meet the thing first and the "by the way, theorists call ours the untagged kind" framing lands
  on something concrete. Or soften the lead so it's unmistakably a "you'll appreciate this later"
  aside, not a prerequisite.

**F4 · little/part6-hash-and-tuple.md §6-3 · "width subtyping" leans on "subtype" one chapter early**
- *The break (reader-voice):* §6-3 is the climax and it's built on "**subtyping**" and "**width
  subtyping**" — "you can pass `{name:, admin:}` where `{name:}` is wanted … the side with *more*
  keys is the subtype." But "subtype" isn't taught until Part 7 (the text even says so: "'subtype'
  is taken head-on in the next chapter"). So at the climax of Part 6 I'm asked to reason with a
  relation that's explicitly deferred. The inline gloss ("the side with more keys passes more
  demands, so it's the subtype") is a genuine help and the *policy* itself ("allow extras, blame
  only what's missing") I fully grasp from the Ruby options-hash story — so I recover. But the word
  "subtype," used as if load-bearing a chapter before its lesson, makes §6-3 feel harder than the
  idea actually is.
- *The fix:* In §6-3, lead with the plain-Ruby conclusion (which already works without the term):
  "as long as it has *at least* these keys it's fine; extras don't matter; only a *missing* key is
  a problem." Then introduce "subtype" purely as a *forward-pointer* ("type theory has a name for
  'more keys = fits where fewer is wanted' — *width subtyping* — we meet 'subtype' properly next
  chapter"), so the term decorates the idea instead of carrying it. The idea should stand on the
  options-hash motive alone, which it can.

**F5 · little/part2-method-dispatch.md §2-2 · "the type version of dispatch" packs two ideas into one move**
- *The break (reader-voice):* "Where Ruby picks the method body *at run time*, we pick 'what does
  that method return (its return type)' *at type-check time*." Two notions I've never separated —
  *runtime* vs. *type-check time*, and "a method *has* a return type you can look up without
  running it" — arrive fused in one sentence. As a Rubyist I only ever *run* code to see what it
  returns; the idea that you can know the return type *without running* is the actual leap, and
  it's stated as if obvious. The table example right below (`(Integer, +)` → `Integer`) rescues me
  by *showing* it, so it's friction, not a block.
- *The fix:* One sentence of scaffolding before the table: "Normally you'd *run* `1.+(2)` to see it
  gives `3`. A type checker can't run the code — so instead it *looks up* 'an `Integer` plus an
  `Integer` gives an `Integer`' in a table, without computing anything." That names the leap
  ("can't run it / look it up instead") I'm being asked to make.

### nitpick

**N1 · little/part0-introduction.md §0.1 · "synthesis" parenthetical**
- "building a type upward from an expression (synthesis)" — the word *synthesis* is dropped in
  parentheses and never needed again in the Little volume (it's a Seasoned term). For my profile it
  reads as a vocabulary tax with no payoff in this volume. Fine to keep as a forward-pointer, but
  it's the kind of bare term that makes a Ruby-only reader feel they've missed something. Consider
  cutting it here or marking it explicitly "(a Seasoned-volume name; ignore for now)."

**N2 · little/part0-introduction.md §0.4 · "三題噺 / three-voice piece"**
- The English keeps the Japanese "三題噺" alongside "three-voice piece." Harmless, but for an
  English-edition reader the untranslated term is a tiny speed bump every time it recurs (P0
  README, P0 §0.4). The gloss "three-voice piece" already does the work; the raw 三題噺 could be a
  one-time etymology note rather than a recurring label.

**N3 · little/part7-accepts-and-trinary.md §7-3a · "union-subtyping" in the ① cell**
- The three-voice table's ① cell says "take the weakest conclusion = union-subtyping." That hyphen-
  compound term appears nowhere else and isn't glossed. The body already explained the *idea*
  ("safe only when all members pass → take the weakest answer") perfectly without it. The bare term
  in the summary cell adds nothing for me and reads as jargon I should recognize. Drop it or gloss.

**N4 · little/part1-literals-and-arithmetic.md §1-1 column · "top type" used to deny a confusion I don't have yet**
- The "two opposite schools" column ends: "Why `untyped` is so easily confused with the 'holds
  anything' top type (they're actually different) … is in appendix a1-1." I haven't met "top type"
  and have no confusion to disambiguate yet, so the sentence warns me off a mistake I can't make.
  Minor — it's a clearly-optional appendix pointer — but the phrasing "so easily confused"
  presumes a familiarity I lack. Could read "(a distinction that matters later; appendix a1-1)."

**N5 · little/part4-union.md §4-1x note · note is long and front-loads `dispatch_union` internals**
- The "method sends to a Union receiver" note is nearly as long as the chapter's main line and
  dives into `dispatch_union`, `merge_member_diagnostics`, `const_combinations`, and a real-Rigor
  spec quote. It's explicitly fenced as a "note set aside from the main line," so I *can* skip it —
  but it's heavy enough that a tired reader might mistake it for required material and stall. The
  fence is good; consider a one-line "(skip on a first read)" at its head to make the off-ramp
  louder. Purely a pacing nit.

---

## Notes on what is *not* a problem (so a later editor doesn't "fix" it)

- **`Const[1]`, `Union`, `HashShape`, `Tuple` as non-class types** are all introduced *after* the
  P1 §1-1 "a type is data, not a class" switch, each with a Ruby `Data.define` shown. The "type ≠
  class" footing is established before it's tested. Correct order — don't add more caveats.
- **Every TypeScript / Java / Kotlin / Go comparison** I hit (P1 `unknown`/`any` column, P5 null-
  safety, P6 Hack/PHPStan, P7 structural subtyping, the a5 appendix) is fenced as an optional
  bonus *and* paired with a Ruby restatement. None is a foothold I must already get. The book's
  promise ("presupposing only Ruby … complete without these languages") holds in practice.
- **`untyped` / `Dynamic` / `:maybe`** are motivated from Ruby's reality (unknown method, no
  annotations) every time, never assumed. Good.
- The deferrals to the Seasoned volume (variance, bidirectional, soundness, `Top`/`Bot` lattice)
  are correctly signposted as out-of-scope and never relied on. Not my job to ask for them here.
