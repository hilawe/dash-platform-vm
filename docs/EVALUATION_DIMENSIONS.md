# Evaluation dimensions, fixed before synthesis

Written 2026-07-19. Sequencing note, recorded rather than smoothed over. One external design had
already been received and read when this was written, and the other two had not returned. So this is
genuinely pre-registered for two of the three external designs, and partially post-hoc for one. Recording that rather than backdating
it, because the playbook's whole point is that the synthesizer is also an author and will otherwise
favor the design resembling its own.

The rubric exists to stop me scoring designs by resemblance to the author's own design. Where a
design differs from the author's, the burden is on the author's design to justify itself, not the
reverse.

## Dimensions, in weight order

1. **Determinism argument quality.** Does the design name the specific divergence sources and the
   specific mechanism that closes each? A design that says "we ensure determinism" without naming
   floating point, iteration order, memory growth, metering-boundary timing, and compiler or
   runtime version pinning has not answered the hardest question. Weighted first because this
   failure mode presents as consensus forks, not bugs.

2. **Provability outcome for program-written state.** Does state written by programs keep
   membership, non-membership, and secondary-index proofs to light clients? A design that
   sacrifices this must say so and justify it. Weighted second because it is the platform's
   distinguishing property and the requirement most likely to be quietly dropped.

3. **Worst-case block execution bound.** Is there a real argument that block time stays inside the
   consensus cadence under adversarial load, including proof-generation cost, not just typical
   load? Vague appeals to metering do not count.

4. **State-write path and its validation.** Where does authority to mutate state actually sit, and
   what re-validates a program's writes? This is the axis where the designs are most likely to
   diverge meaningfully, so it gets its own dimension rather than being folded into provability.

5. **Native-capability integration, especially the asynchronous ones.** Quorum threshold signatures
   and bridge operations cannot plausibly complete synchronously inside one block's execution. A
   design that treats them as synchronous calls has a latent correctness problem; one that models
   them as requests resolved in a later block has thought it through.

6. **Program upgrade and its user-safety consequence.** Mutable program code is a standing power
   over users who relied on the old logic. Whatever the choice, does the design confront that
   consequence explicitly rather than treating upgrade as a convenience feature?

7. **Interoperability decision quality.** This was left an open question deliberately. Score the
   quality of the argument and its honesty about what is given up, NOT whether the conclusion
   matches the author's. A well-argued conclusion opposite to the author's scores higher than a
   matching conclusion asserted without reasoning.

8. **Metering and storage-pricing soundness.** Especially ongoing versus one-time storage pricing,
   since underpricing storage produces irreversible bloat.

9. **Honesty about new trust and about costs.** Does the design confront what it actually
   introduces, including any deployment gate as a real trusted role during the governed phase, and
   are the developer and operator costs stated plainly rather than optimistically?

10. **Build plan ordering.** Is the riskiest assumption genuinely first, and is it the assumption
    whose failure kills the design?

## Anti-bias rules for synthesis

- Convergence is evidence the requirements force a choice, not evidence the choice is correct or
  secure. Shared blind spots are the known failure mode of this method.
- Discount convergence that plausibly traces to the disclosed consensus-engine lineage (see
  `REQUIREMENTS.md` leak-check record). Convergence reasoned from the provability property or from
  determinism is NOT in that category and counts normally.
- Any dimension where all designs including the author's agree gets an explicit "is this a shared
  blind spot?" check before it is written down as forced.
- The author design gets extra skepticism by policy. Where it is weaker, say so plainly in the
  synthesis and in the comparison writeup.
