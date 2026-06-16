# Translation-fidelity review — EN transcreation vs. JA source of truth

**Scope of this review:** technical-claim fidelity only (the "③ Rigor's implementation
problem" voice, "in real Rigor… / Rigor does X" statements, numbers, ADR/chapter/TAPL
citations, default values, and load-bearing hedges). Wording differences are NOT flagged —
the English is a deliberate transcreation. The Japanese under `book/v1/ja/` is the source of
truth for content.

## Overall verdict

**Faithful.** Across every file compared — Seasoned Parts 1–8, appendices a1/a2/a3/a5, the
glossary, and the entire Little volume (Parts 1–9) — the English transcreation preserves the
Japanese's technical claims, hedges, numbers, and citations with no detected ERROR or
MISLEADING distortion. No claim about real Rigor was invented, softened, or hardened; no
caveat was dropped; no number or citation drifted. The transcreation is unusually disciplined
about exactly the things this review targets.

No fixes required. The per-file notes below record the load-bearing claims that were checked
and confirmed accurate, so a future editor can see what was verified rather than re-derive it.

---

## Seasoned volume

### seasoned/part1-bidirectional-typing.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 1-5 "`ExpressionTyper` … pure, non-destructive"; "`accepts` at a call site … three-valued + reason"; "`Scope` / `FactStore` … treated in Part 6" | identical (純粋・非破壊／三値＋理由／Part 6) | none | — | — |
| §1-3 two reasons (totalizing synthesis chibirigor-specific; `untyped` passes checking) | identical | none | — | — |
| Pierce & Turner 2000, Dunfield & Krishnaswami 2021 | identical | none | — | — |
| `param:` directive, two jobs, RBS-side only, `%a{rigor:v1:param: …}` | identical | none | — | — |

### seasoned/part2-subtyping-and-variance.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 2-4 / 2-7 "current implementation processes a Nominal's type arguments uniformly covariantly … declaration-site variance is designed, unimplemented (Slice 5 on)"; `lib/rigor/inference/acceptance.rb` | identical incl. the "designed but unimplemented" hedge and the Slice 5 reference | none | — | — |
| 2-7 "function (proc) type … erased to the nominal `Proc` (no first-class function subtyping)"; "S-Arrow … method override-compatibility check (ADR-35)" | identical, ADR-35 preserved | none | — | — |
| 2-7 join/meet via `Combinator.union` | identical | none | — | — |
| Sorbet `T.assert_type!` column incl. the "not Sorbet's official term" disclaimer | identical | none | — | — |

### seasoned/part3-generics-and-substitution.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 3-6 `RbsTypeTranslator.translate(..., type_vars:)`; "capture exposure is small"; bounded quantification via interface/capability role (TAPL ch. 26); `Type#erase_to_rbs` | identical | none | — | — |
| 3-6x generics 5a/5b/5c split; `element_read` in `lib/chibirigor/type_of.rb`; literal index keeps Tuple precision; `[].first` / `foo.first` → untyped | identical | none | — | — |
| 3-5 erasure "may get wider, never narrower (sound approximation)"; "superset of RBS" | identical | none | — | — |

### seasoned/part4-recursive-types.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 4-4 "a sound *reduced version*, weaker than TAPL ch. 21's proper coinduction … the proper test puts post-unfold pairs into `seen`" | identical hedge preserved verbatim in meaning | none | — | — |
| 4-5 `Type::App`, `App[:"json::value", [String]]` arity 1, fuel; column: `symbolize_names: true` → `Const[true]` → `App[…, [Symbol]]`; **fuel default 64** + progress tracking | identical, default 64 preserved | none | — | — |
| 4-5 HKT grounds = TAPL ch. 29 (kinding), not ch. 20/21; "defunctionalized implementation" | identical | none | — | — |
| HKT three-valued `:yes/:no/:maybe`, fuel-out → `:maybe` table | identical | none | — | — |

### seasoned/part5-real-inference.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 5-4 decidability/reachability/precision; rank-3+ undecidable (Kfoury–Wells 1994); Wells 1994/1999 unrestricted System F undecidable; plugins ADR-2,16 | identical incl. all citations and ADR numbers | none | — | — |
| 5-4a TypeProf whole-program vs Rigor local+catalog; `Dynamic[Top]`; `rigor sig-gen` (Chapter 11), ADR-5 "don't make an observed call site the default" | identical | none | — | — |
| 5-6x `type_of_block` in `lib/chibirigor/type_of.rb`; "direct substitution, not unification"; "degenerates to substitution"; 5b/5c | identical | none | — | — |
| occurs-check note ("essential to termination and soundness in real HM") | identical | none | — | — |

### seasoned/part6-fact-store.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 6-1 Fact's six fields: `bucket`, `target`, `predicate`, `payload`, `polarity`, `stability`; chapter minimizes to three | identical | none | — | — |
| 6-2 the six buckets (local_binding / captured_local / object_content / global_storage / dynamic_origin / relational); "bucket names match real Rigor's internal spec (`inference-engine.md`)"; dynamic_origin's separate role | identical, all six names and the spec reference preserved | none | — | — |
| 6-7 next-chapter teaser "four places Rigor makes deliberately unsound" | identical | none | — | — |

### seasoned/part7-soundness.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 7-4 four holes mapped to progress/preservation; "what it lets go of is mainly progress" | identical | none | — | — |
| 7-5 gradual consistency `~` symmetric/non-transitive; "real Rigor spells it `consistent(A, B)`, `~T` reserved for negation/complement"; gradual guarantee | identical | none | — | — |
| 7-5 `assert` directive column ("after returns, caller's `x` is `non-empty-string`"; RBS-side only; TS user-defined type guard) | identical | none | — | — |
| 7-6 coinduction → **HKT fuel ADR-20 (implemented), default 64 steps** → **inference budget ADR-41 (designed, unimplemented; Status: Proposed)** | identical, both ADR numbers, both implementation-status hedges, and the 64-step default preserved | none | — | — |

### seasoned/part8-toward-rigor.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| 8-1 correspondence table: `ExpressionTyper`, `accepts`+`AcceptsResult`, `Scope`+`FactStore` (six buckets, stability), `MethodDispatcher` (multi-tier), type carriers **7 kinds** → many | identical, the "7 kinds" count preserved | none | — | — |
| 8-2 full ADR map: ADR-2/37/9, 16/48, 6/45/46, 19, 51/27, **22 (baseline+onboarding)**, 44/15/50 | identical, every ADR number matches | none | — | — |
| 8-2-a `register_method` / `Plugin.registry[key] \|\| METHODS[key]`; "corresponds to ADR-2's extension API skeleton" | identical | none | — | — |
| 8-3 ADR-0 → ADR-4; ADR-5/22; ADR-20/41; ADR-14/25/32; ADR-46 + `inference-engine.md` | identical, all ADR numbers and the spec file match | none | — | — |

---

## Appendices

### appendix/a1-special-types.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| a1-1 two-axis decomposition (A: lattice position / B: checked-or-not); `Dynamic[Top]`; lowercase `top`/`bot` in real code, uppercased for the book | identical | none | — | — |
| a1-1 cross-language table (any/unknown, mypy Any, Go interface{}, PHPStan mixed, C# dynamic, Dialyzer dynamic()) | identical | none | — | — |
| a1-3 / a1-3x bottom type as **diagnostic not a type carrier**; real Rigor unreachable arm = **ADR-47 (`flow.unreachable-clause`)**; FP envelope excluding loops/blocks/gradual; `unreachable_branch?` in `lib/chibirigor/narrowing.rb`; leaf-class disjointness; ancestor relations not asserted | identical, ADR-47 and the rule id preserved | none | — | — |
| a1-2 `void` top-like but return-position-only marker, folds back to `top` in value position; BC-break contract | identical | none | — | — |

### appendix/a2-narrowing-patterns.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| a2-3 escape patterns (Thread.new/define_method/Proc.new/Fiber.new); immediate-vs-escape inferred from RBS signature annotations; when-in-doubt-erase | identical | none | — | — |
| a2-6 refinement carrier two-layer (ADR-3): point-removal → `Difference` (`String - ""`); predicate-subset → `Refined`; ranged int → `IntegerRange` (`Integer[1..10]`; `int<min,max>` for directive/internal display) | identical, the ADR-3 two-layer split and all three carrier names preserved | none | — | — |
| a2-6 PHPStan correspondence table (all rows, including the "—" no-PHPStan-equivalent rows) | identical | none | — | — |
| a2-6x chibirigor `Tuple` = effective `non-empty-array` "same effect, different origin"; real Rigor generates the carrier from `unless arr.empty?` flow fact | identical | none | — | — |

### appendix/a3-tooling.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| a3-3 the **5-stage dispatch cascade**: ① constant folding → ② shape → ③ RBS → ④ in-source → ⑤ fallback (Dynamic[Top]); fall-through semantics; ③ beats ④ via stage order; `RBS::Extended` directive example | identical, all five stages named and ordered exactly | none | — | — |
| a3-1 / a3-1x `check --explain` maps fail-soft via the persisted `Dynamic` marker; chibirigor's tiny version picks up "just one kind, unknown dispatch" (`lib/chibirigor/dispatch.rb`, signature-nil branch) | identical | none | — | — |
| a3-2 erasure two-layer (internal `Constant<"FOO">` → boundary `String`); `dump_type` needs `include Rigor::Testing` or `Rigor.dump_type`; PHPStan `dumpType()` analogy; `:info` rides §a3-1 mechanism | identical | none | — | — |
| a3-3b/bx `rigor trace` events (bind/union/dispatch); real Rigor has `--line=N`, chibirigor has no line filter ("leafy difference") | identical, the capability gap honestly preserved | none | — | — |

### appendix/a5-other-languages.md

| en location/quote | ja original meaning | distortion | fix | severity |
|---|---|---|---|---|
| a5-3 Hack `shape(...)` → PHPStan/Psalm `array{...}` → Rigor (RBS records), all open | identical | none | — | — |
| a5-4 **untagged union** vs tagged variant; 『しくみ』 deliberately avoided general unions; TAPL holds tagged variants | identical, term and the deliberate-avoidance claim preserved | none | — | — |
| a5-5 missing arm (Java/C#) vs unreachable arm (Rigor), opt-in `check --unreachable` | identical | none | — | — |

---

## Little volume (③ Rigor voices)

All nine Little chapters compared. The ③ voice and the "real Rigor / handed-to-the-sequel"
claims are faithful in every case:

- **part1** — Const literal precision, "when to round," ADR-51 (SARIF/GitHub), `sig-gen` seed,
  two-school "don't know" column → all preserved.
- **part2** — ③ "RBS + inheritance-chain resolution"; **Const fold budget 1,000,000**; the
  5-stage cascade pointer to a3 → all preserved.
- **part3** — immutable Scope ("real Rigor uses the same immutable design"); bare-name =
  `self.x` subtlety → preserved.
- **part4** — **untagged union** term (matches the user-memory note); the `inference-engine.md`
  spec quote "`Union` receivers MUST dispatch each member individually"; all-members-fail →
  angry / partial → `:maybe` / unknown member → untyped → all preserved.
- **part5** — ADR-47 unreachable arm (opt-in); `possible?` guard; the two laws; **Union size
  budget widening** to nominal-class union → all preserved.
- **part6** — open HashShape / width subtyping to a dynamic hash; Hack/PHPStan/Psalm lineage
  column; map vs filter_map widening; `DataClass`/`DataInstance` → all preserved.
- **part7** — three-valued logic; weakest/strongest answer for Union actual/expected;
  structural subtyping column; robustness-principle footnote → all preserved.
- **part8** — RBS as source of truth; `void`/BC-break; sig-gen seed; TypeProf contrast
  (call-site argument inference Rigor deliberately declines) → all preserved.
- **part9** — the **four deliberate misses**; **baseline ADR-22** (matches on rule ID, no line,
  no column; `.rigor-baseline.yml`); `check --explain`; three special types; progress +
  preservation teaser → all preserved.

## Glossary

`book/v1/en/glossary.md` — every TAPL chapter citation (ch. 8 §8.3, 15, 16, 20–21, 22, 23, 29),
the Rigor-internal concepts (`Difference`/`Refined`/`IntegerRange`, refinement carrier, erasure
vs. Java type-erasure, HKT/kind), and the Hack→PHPStan→Rigor `HashShape` lineage are accurate.
