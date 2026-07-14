# Architecture Decision Records (ADRs)

This folder records the significant, non-obvious decisions behind RustOS: the alternatives that were considered, the facts that eliminated the losers, and the consequences of the choice. The intent, per `os_guide.md`, is that "future-you, returning after a month away" can reconstruct *why* a decision was made rather than just *what* it was.

## Conventions

- One decision per file, named `NNNN-short-title.md` with a zero-padded sequence number.
- Numbers are never reused. A decision that is later reversed is not deleted; a new ADR supersedes it and the old one is marked `Superseded by ADR-XXXX`.
- Each ADR carries a **Status**: `Proposed`, `Accepted`, `Superseded`, or `Deprecated`.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-programming-language-rust.md) | Programming Language: Rust | Accepted |
| [0002](0002-cpu-architecture-x86-64.md) | CPU Architecture: x86-64 | Accepted |
| [0003](0003-emulator-qemu.md) | Primary Emulator: QEMU (`qemu-system-x86_64`) | Accepted |
| [0004](0004-bootloader-crate.md) | Bootloader: `bootloader` crate (v0.9, BIOS) + `bootimage` | Accepted |
| [0005](0005-freestanding-build-custom-target.md) | Freestanding build: `#![no_std]`, custom target, `build-std` | Accepted |
| [0006](0006-in-emulator-test-harness.md) | In-emulator test harness: `isa-debug-exit` + serial | Accepted |
| [0007](0007-diagnostics-lifeline.md) | Diagnostics lifeline: 16550 serial + QEMU GDB stub | Accepted |
| [0008](0008-higher-half-kernel-layout.md) | Higher-half kernel address-space layout | Proposed |
| [0009](0009-data-oriented-design-constraint.md) | Data-oriented design as a first-class constraint | Accepted |
| [0010](0010-cli-first-gui-deferred.md) | CLI-first scope; GUI an optional late capstone | Accepted |

## Decisions not yet recorded (future ADRs)

These are known open questions from `os_guide.md`. They are **not** yet decided, so they have no ADR — an ADR records a made decision. Each should get one when resolved:

- Physical frame-allocator design (bitmap vs free-list vs buddy) — Phase 3.
- Kernel heap allocator progression (bump → free-list → slab) — Phase 3.
- Physical-frame access strategy (recursive mapping vs offset map) — Phase 3, sub-decision of [ADR-0008](0008-higher-half-kernel-layout.md).
- Scheduler policy (round-robin → priority → MLFQ → fair) — Phase 4.
- Syscall ABI and process model (fork/exec vs spawn; COW) — Phase 6.
- Filesystem choice (FAT vs ext2 vs custom log-structured) and block driver (virtio-blk) — Phase 7.
- A future AArch64 port — see [ADR-0002](0002-cpu-architecture-x86-64.md).

## Template

Each ADR uses this structure:

```markdown
# ADR-NNNN: <Title>

- Status: <Proposed | Accepted | Superseded | Deprecated>
- Date: <YYYY-MM-DD>
- Deciders: <who>

## Context
<The forces at play: requirements, constraints, the problem being solved.>

## Decision
<The choice, stated plainly.>

## Alternatives Considered
<Each real alternative and the fact that eliminated it.>

## Consequences
<What becomes easier, what becomes harder, and what risks are accepted.>
```
