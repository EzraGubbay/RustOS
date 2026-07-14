# ADR-0001: Programming Language — Rust

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

RustOS is committed to strict data-oriented design (DOD) and heavy, measured optimization from the first line of code. As `os_guide.md` (Phase 0) frames it, the language decision governs "how much control you have over memory layout, whether a runtime and garbage collector sit between you and the hardware, how SIMD is expressed, and how much the compiler helps you stay correct." Because the project fixes DOD as a first-class constraint, layout control and the absence of a managed runtime dominate the decision.

The evaluation criteria, drawn from the guide, were: memory-layout control (packing, alignment, struct-of-arrays), freedom from a mandatory GC/runtime, first-class SIMD intrinsics, cross-compilation ergonomics, compile-time safety, maturity of the bare-metal ecosystem, and how naturally the language expresses data-oriented patterns.

The project is also explicitly educational and a runway into a parallel-systems-programming course, so relevance to concurrency and to the author's longer-term interests (parallel programming, machine learning) carried weight.

## Decision

Implement RustOS in **Rust**, using the freestanding (`#![no_std]`) dialect on a custom bare-metal target.

## Alternatives Considered

- **Zig.** The strongest contender. Its data-oriented ergonomics are excellent — `MultiArrayList` makes struct-of-arrays a built-in, which directly serves the project's dominant DOD pattern. It was rejected because Rust's compile-time memory-safety guarantees (no use-after-free, no data races in safe code) were judged more valuable for a long-horizon solo kernel than Zig's native SoA convenience. The author accepts having to express DOD patterns manually rather than relying on language support.
- **C.** The traditional kernel language with the densest OSDev tutorial ecosystem. Rejected: no compile-time memory safety, weaker type system, and it offers no advantage over Rust in layout control while giving up Rust's safety discipline. The guide's own reference material (the Biscuit paper, Redox OS) supports a modern systems language with layout control over C for this purpose.
- **A garbage-collected language (e.g. Go).** Rejected on principle for a DOD-strict kernel: a mandatory runtime and GC sit between the code and the hardware and impose costs on the hot path. The Biscuit paper (OSDI '18) is cited in the guide as the empirical case against GC in a kernel.

## Consequences

- **Safety pays down risk on a long, intermittent project.** The borrow checker eliminates whole classes of bugs that are especially painful to debug on bare metal, where the only symptom is often a silent reset.
- **Layout control is retained without a runtime.** Rust gives the packing, alignment, and `repr` control that DOD requires, with no GC pauses on the hot path.
- **DOD is manual.** Without Zig's `MultiArrayList`, struct-of-arrays, handle/index tables, and hot/cold splitting must be built and maintained by hand. This is accepted, and is arguably itself an instructive exercise.
- **`unsafe` is unavoidable and must be disciplined.** Kernel work (MMIO, page tables, context switches) requires `unsafe`. The safety benefit only holds if `unsafe` blocks are kept small, justified, and wrapped in safe abstractions.
- **Bare-metal ecosystem is mature but nightly-leaning.** The project already relies on unstable features (`build-std`, custom JSON target spec). Toolchain pinning via a `rust-toolchain` file is advisable.
- **Strong alignment with downstream goals.** Rust's ownership and `Send`/`Sync` model map cleanly onto the Phase 4 synchronization work and the parallel-programming through-line.

## References

- `os_guide.md` — Phase 0, "Language selection"; Resource Atlas, "The language-decision references".
- `docs/dev-journal/001-init.md` — decision 2, "Programming Language: Rust".
- Cutler et al., *The benefits and costs of writing a POSIX kernel in a high-level language* (Biscuit, OSDI '18).
