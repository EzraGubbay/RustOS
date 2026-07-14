# ADR-0005: Freestanding Build — `#![no_std]`, Custom Target Spec, `build-std`

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

A kernel runs with no OS beneath it, so it cannot use Rust's standard library or any hosted assumptions (`os_guide.md`, Phase 0, "The bare-metal language dialect"). It must build against a target that has no host OS, define its own entry point, and abandon runtime services like stack unwinding. This bundles several tightly coupled build decisions that together define "how a compiled artifact becomes a bootable image" — the thing Phase 0 says to master before moving on.

The repo already encodes these choices in `x86_64-rust_os.json`, `.cargo/config.toml`, and `Cargo.toml`.

## Decision

Build RustOS as a **freestanding `#![no_std]`** binary against a **custom JSON target specification** (`x86_64-rust_os.json`), compiling `core` and `compiler_builtins` from source via **`build-std`**. Specifically:

- **`llvm-target: x86_64-unknown-none`**, `os: none` — no host OS.
- **`panic-strategy: abort`** — no stack unwinding (`panic = "abort"`), which removes the need for unwinding machinery in a context that has no runtime to unwind into.
- **`disable-redzone: true`** — the red zone is unsafe in code that takes interrupts, since an interrupt handler can clobber it.
- **`features: -mmx,-sse,+soft-float`** with **`rustc-abi: softfloat`** — floating-point/SIMD units are disabled and float ops are done in software, because using the FPU/SSE in the kernel requires saving that state on every interrupt/context switch, which is deferred until the project explicitly handles vector state (guide Phase 8).
- **`build-std = ["core", "compiler_builtins"]`** with **`build-std-features = ["compiler-builtins-mem"]`** — the standard prebuilt libraries don't exist for a custom bare-metal target, so the sysroot crates are rebuilt from source.
- **Edition 2024**, and unstable toolchain features (`json-target-spec`, `build-std`), i.e. a **nightly** toolchain.

## Alternatives Considered

- **An existing built-in bare-metal target (e.g. `x86_64-unknown-none`) instead of a custom JSON spec.** Viable and simpler, but a custom spec gives explicit control over red zone, SIMD features, and panic strategy, and makes those decisions visible and reviewable in-tree. The extra control is worth the small maintenance cost for a project whose whole point is understanding the machine.
- **Keeping SSE/FPU enabled.** Rejected for now: it forces early, careful management of vector state on every context switch and interrupt — a Phase 8 concern the project deliberately defers. Soft-float keeps the early kernel simple.
- **Leaving the red zone enabled.** Rejected: the red zone and interrupt handlers are fundamentally incompatible; disabling it is standard kernel practice.
- **`panic = "unwind"`.** Rejected: there is no runtime to unwind through, and unwinding pulls in machinery a freestanding kernel shouldn't carry. `abort` is the correct semantics for a kernel panic.
- **A stable toolchain.** Not currently possible: `build-std` and JSON target specs are unstable. The nightly dependency is accepted; pinning via a `rust-toolchain` file is advisable to keep builds reproducible.

## Consequences

- **Full control over the emitted artifact**, at the cost of a nightly toolchain and rebuilding sysroot crates (slower clean builds).
- **No `std`** — only `core` (and later `alloc`, once a global allocator exists, guide Phase 3). Every dependency must be `no_std`-compatible; the current deps (`volatile`, `spin`, `lazy_static` with `spin_no_std`) already are.
- **No hardware floating point in the kernel** until vector-state handling is deliberately introduced. Kernel code that needs floats pays the soft-float cost; this is an accepted early trade.
- **Interrupt-safe stack usage** from the red-zone decision.
- **Reproducibility depends on toolchain pinning.** Because unstable features drift, the nightly version should be pinned and recorded.
- **A linker script / higher-half layout interacts with this** as the address-space design firms up (see [ADR-0008](0008-higher-half-kernel-layout.md)).

## References

- `os_guide.md` — Phase 0, "Toolchain and environment" and "The bare-metal language dialect".
- Existing repo artifacts: `x86_64-rust_os.json`, `.cargo/config.toml` (`build-std`, `json-target-spec`), `Cargo.toml` (edition 2024, `no_std` deps).
