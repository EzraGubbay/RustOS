# ADR-0008: Higher-Half Kernel Address-Space Layout

- Status: Proposed
- Date: 2026-07-14
- Deciders: Ezra

## Context

`os_guide.md` (Phase 3) lists "a higher-half kernel layout" among the virtual-memory topics to master, and notes that linker scripts matter "the moment you have a higher-half kernel." The address-space split — where the kernel lives versus where user programs live — is a foundational decision that shapes the linker script, the page-table setup, the physical-frame access strategy, and the eventual user/kernel boundary (Phase 6).

This decision is recorded now as the intended design, but it is **not yet implemented** — the kernel is still in the early phases. Hence status **Proposed**: it captures the committed direction so later work is consistent, and will move to **Accepted** once paging and the higher-half transition are actually built.

## Decision

Lay the kernel out in the **higher half** of the 64-bit virtual address space (the top of the canonical address range), leaving the lower half for user address spaces. Concretely this means:

- The kernel is linked to run at a high virtual base address (via the linker script), separate from where it is physically loaded.
- Early boot survives the transition **from identity-mapped to higher-half execution** (guide Phase 3, "MMU implementation").
- A strategy is chosen for the kernel to **access physical frames it hasn't explicitly mapped** — a physical-memory offset map versus recursive mapping — which is a sub-decision to be recorded when made.

## Alternatives Considered

- **A low-half / identity-only kernel.** Simpler to reach initially (no high-address transition), but it collides with user address spaces, complicates the clean user/kernel split in Phase 6, and is not how real kernels are laid out. Rejected as the end state; identity mapping is used only transiently during boot.
- **Recursive page-table mapping vs a physical-memory offset map** for reaching physical frames. Both are legitimate; this ADR does not yet fix the choice. The offset map (a straightforward linear map of all physical RAM at a fixed high offset) is simpler to reason about; recursive mapping is more compact but trickier. To be decided and recorded as a follow-up when paging is implemented.

## Consequences

- **A linker script is required** to place the kernel at its high virtual base — this interacts with the freestanding build ([ADR-0005](0005-freestanding-build-custom-target.md)).
- **A clean user/kernel address-space split** is set up for Phase 6, where user programs occupy the lower half and the boundary is enforced.
- **The identity → higher-half transition is a known hazard.** Getting it wrong is a classic silent-reset bug; the diagnostics lifeline ([ADR-0007](0007-diagnostics-lifeline.md)) will be essential here.
- **An open sub-decision remains** (physical-frame access strategy), to be resolved and appended when the mechanism is built.
- **Page tables are kernel tables.** Their layout should follow the DOD stance ([ADR-0009](0009-data-oriented-design-constraint.md)) — compact, index-keyed, cache-conscious.

## References

- `os_guide.md` — Phase 0 ("Linker scripts"); Phase 3, "Virtual memory and paging" and "MMU implementation".
