# ADR-0010: CLI-First Scope; GUI as an Optional Late Capstone

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

An OS project can expand without bound, so scope has to be bounded deliberately. `os_guide.md` is explicit: the project is "CLI-first; GUI is an optional, late capstone," and the graphics phase (Phase 9) is described as "the capstone … Large, optional, and best approached only once the system beneath it is solid." The project instructions echo this: "After implementing the OS as a CLI version, an optional GUI may be considered," and GPU/SIMD/parallel features are studied last.

This is a sequencing and scoping decision, recorded so effort stays on the foundational subsystems and neither the GUI nor the GPU track pulls work forward prematurely.

## Decision

Build RustOS **CLI-first**. Treat the windowing/GUI system (guide Phase 9) as an **optional capstone attempted only after the core OS is solid** — i.e. after bring-up, interrupts, memory, scheduling/synchronization, IPC, userspace/processes, and storage/filesystems are working. Likewise, defer the SIMD/GPU/parallel-hardware work (Phase 8) to **after all non-optional OS features**, per the project goals.

The ordering follows the guide's dependency-ordered phases and capability checkpoints; there is **no timeline** — the phases are dependency-ordered, and pace is deliberately left open.

## Alternatives Considered

- **Building a GUI early / in parallel.** Rejected: a display server, compositor, and widget system depend on a working framebuffer path, memory management, scheduling, and input routing. Starting graphics before those exist means building on sand and repeatedly reworking it.
- **Pulling the GPU/SIMD track forward** (the author's special-interest area). Tempting, but the guide scopes it as Phase 8 precisely because vector-state save on context switch, DMA, and per-core scheduling only make sense once the OS beneath them exists. Deferred to protect the foundations; the CUDA/Metal track can be practiced *separately* on real hardware in the meantime.
- **Committing to a fixed timeline.** Explicitly rejected by the guide — this is a long-horizon, intermittent project; phases are ordered by dependency, not calendar.

## Consequences

- **Effort stays on foundations.** The near-term work is serial I/O, interrupts, memory, and scheduling — not pixels.
- **The GUI may never be built, and that's acceptable.** It is explicitly optional; the project's success does not depend on it.
- **Data-oriented graphics is deferred, not forgotten.** When Phase 9 is reached, its ECS/batching/dirty-rect patterns are where the DOD stance ([ADR-0009](0009-data-oriented-design-constraint.md)) most directly applies.
- **The GPU/parallel track runs on two rails.** Systems intuition (scheduling, coherence, DMA) comes from the OS; the SIMT/CUDA or Metal programming practice happens separately and earlier, without blocking OS progress.
- **Checkpoints, not deadlines,** measure progress (guide's Capability Checkpoints).

## References

- `os_guide.md` — Executive Summary and Phase 9 ("The capstone … Large, optional"); Phase 8 (GPU scoped honestly); "Capability Checkpoints".
- Project instructions — CLI-first, optional GUI; GPU/SIMD studied last.
