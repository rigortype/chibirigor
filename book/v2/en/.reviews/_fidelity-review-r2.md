# EN fidelity review r2 (JA ↔ EN)

**Scope.** English changes `b1e59ad..HEAD` under `book/v2/en` (excluding `.reviews/` and
`_handoff-state.md`): 25 files: Little P0–P2, P4–P9; Seasoned P1–P8 + `examples/README.md`;
appendices a1–a5; glossary; `figures/svg/seasoned-6-1.svg`. Checked against the current JA
(`book/v2/ja` at HEAD, i.e. the changes `74f7783..c6e9a6e`), plus a grep over the whole English
edition for stale sentences that contradict the corrected claims ("unif", `seen`, "all :yes",
"weakest", "fuel", "open"/"extras", "Hack", "(and Rigor)", `--params=`, `parameter_inference:`,
"narrow … Dynamic", "marker", "fell soft to", `@`-tokens).

**Verdict.** One ERROR: in the Seasoned P7 rbs-inline example the `@rbs` tokens have been stripped
from all three comment lines. Everything else checks out. The six focus claims match JA in
substance in every place they appear. No stale English sentence elsewhere contradicts them: the
remaining "weakest answer" / "complain only when every member fails" wording is the
chibirigor-side Union rule plus the receiver-side rule from Part 4, the same as in JA. One NIT.

## Focus claims: status

| Claim | EN location(s) | Status |
|---|---|---|
| Union acceptance: any `:no` → complain; otherwise silent even with `:maybe`; real Rigor passes Dynamic as `:yes` but stays silent | little/part7 L111–115, L167–170; seasoned/part7 NOTE L147–150 | matches JA |
| `App` with an unregistered (`untyped`) bound → `:yes`; acceptance defers to `bound` | seasoned/part4 L219–236; seasoned/part7 L233 | matches JA |
| Per-call body evaluation with actual args (ADR-57, ADR-84), not kept as parameter types; `sig-gen --params=observed` is a proposal (ADR-5, ADR-14), default `--params=untyped`; `parameter_inference:` precision-only (ADR-67); join, not unification | little/part0 L83–89; little/part8 L286–296; seasoned/part5 L217–235, 5-4b L249–270, L284–286, L350–351 | matches JA. Note: JA cites "ADR-5、ADR-14" with no WD number, and EN follows it. The "ADR-14 WD2" in the review brief is not in either edition. The only WD2 cited is ADR-67 WD2 (Road A on hold), and both editions have it |
| HKT cycle detection = exact stop (construction side); fuel = approximate safety net; `seen` has no counterpart in acceptance | seasoned/part4 L168–178, L264, L276–278; seasoned/part7 L225–251, L270–272, L287–288; a4 ADR-20 row | matches JA |
| Hack shapes closed by default; `...` opens | little/part6 L156–160; a5 L73–75 | matches JA |
| rbs-inline assert example in `# @rbs` form inside `class Guard` | seasoned/part7 L183–191 | **ERROR (below)** |

## ERROR

| file:line | EN text | JA text | Problem | Exact fix |
|---|---|---|---|---|
| `book/v2/en/seasoned/part7-soundness.md:186-188` | `>   #  %a{rigor:v1:assert x is non-empty-string}` / `>   #  x: String?` / `>   #  return: void` | `>   # @rbs %a{rigor:v1:assert x is non-empty-string}` / `>   # @rbs x: String?` / `>   # @rbs return: void` (`ja/seasoned/part7-soundness.md:136-138`) | The `@rbs` tokens are gone, leaving a double space. Without them these are plain comments, not rbs-inline annotations, so the example no longer shows what its own lead comment (L184, "in the .rb with an rbs-inline comment (ADR-93)") and Seasoned P1 L268 (`# @rbs %a{…}`) describe. This is a wrong fact in a code sample; it was probably eaten by an `@`-mention or formatter step. It is the only file where the `@`-tokens differ between the editions. | Restore each line: `>   # @rbs %a{rigor:v1:assert x is non-empty-string}`, `>   # @rbs x: String?`, `>   # @rbs return: void` |

## MINOR

None.

## NIT

| file:line | EN text | JA text | Problem | Exact fix |
|---|---|---|---|---|
| `book/v2/en/little/part6-hash-and-tuple.md:121` | "chibirigor makes this **a fit.** What differs is the *motive and scope*. It isn't handling statically-written records for soundness." | 「chibirigorはこれを**適合**にします。静的に書いたレコードを健全性のために扱うわけではありません。」(`ja/little/part6-hash-and-tuple.md:101`) | "What differs is the *motive and scope*" has no counterpart in the current JA. It is left over from the older "Rigor, too, …" framing. It reads fine as a contrast with 『しくみ』, so this is harmless, but it is an addition. | Optional: drop the sentence, giving "chibirigor makes this **a fit.** It isn't handling statically-written records for soundness; the counterpart is…" (or keep it as a legitimate transcreation bridge) |

## Checked, no finding (spot list)

- Little P1 (two-layer display / `sig-gen`), P2 (`call.undefined-method` only on proof), P4
  (`call.possible-nil-receiver`), P5 (Rigor narrows `untyped` under `is_a?`;
  `flow.unreachable-branch` warning / `flow.unreachable-clause` info; `is_a?` dead branches not
  reported; `union_size` specified but unimplemented), P6 ③ + summary row (carrier policy, RBS
  record closed), P7 ③ ×3, P9 (`1 | Dynamic[Top]`; points 2 and 4 caveat).
- Seasoned P1 (`param:` via `# @rbs %a{…}`, ADR-93), P2 (width subtyping only for open shapes),
  P3 (erasure literal rules; contravariance and the `sig-gen` default), P4 4-5 (JSON.parse lookup
  table; one-layer reduction; `bound` table), P5 (TypeProf direction/way; handbook positioning;
  ADR-41 not wired; 5-4b; ADR-67 WD6d; ADR-67 WD2), P6 (`dynamic_origin`; ADR-75; ADR-56
  write-back), P7 (row ②; ADR-83 note; gradual NOTE; 7-6 table), P8 (ADR-50 Proposed; ADR-25/32/93;
  ADR-4/56; `PRISM_DISPATCH`).
- a1, a2 (catalog-based escape judgment; ADR-109 alias), a3 (`--explain` records; single output
  line; line-2 note; `dump_type`), a4 (ADR-20/32 rows; 『しくみ』 "left without an answer"), a5
  (Hack/Rigor bullets; unreachable paragraph; table row), glossary (type reconstruction; erasure),
  `seasoned-6-1.svg` labels ("block escaped" / "dynamic provenance").
