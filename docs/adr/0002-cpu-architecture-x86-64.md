# ADR-0002: CPU Architecture — x86-64

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

As `os_guide.md` (Phase 0) states, the target instruction-set architecture "colours everything" — it touches boot and initial CPU state, the privilege model, the interrupt controller, page-table format, the hardware memory model, and device discovery. The choice is between **x86-64** and **AArch64**.

Relevant inputs specific to this project:

- The author already reads x86-64 AT&T assembly from prior coursework, which lowers the cost of the x86 path.
- The project is preparation for a parallel-systems-programming course; the author expects x86-64 to be the ISA assumed there (noted as a soft, uncertain reason).
- x86-64 has a far denser beginner tutorial ecosystem (notably the phil-opp "Writing an OS in Rust" series), which the project already leans on — the existing repo uses the `bootloader` crate and a custom `x86_64-unknown-none`-derived target.
- AArch64 is a cleaner ISA and closer to the metal the author personally owns (Apple Silicon), and its weak memory model is pedagogically valuable — but these are outweighed by the familiarity and ecosystem advantages for a first implementation.

## Decision

Target **x86-64** for the initial implementation, using QEMU's `q35` machine as the reference platform (see [ADR-0003](0003-emulator-qemu.md)).

## Alternatives Considered

- **AArch64.** Rejected for the *first* implementation, not on merit but on cost. It would mean learning a new ISA concurrently with OS mechanism, and forgoing the denser x86 Rust tutorial base. Its advantages (cleaner initial CPU state, clean `TTBR0`/`TTBR1` user/kernel split, weak memory model as a teaching tool, proximity to the author's own hardware) are acknowledged and deferred, not dismissed.
- **Start on one architecture, then port to the other.** The guide flags this as an "underrated" route — porting is a fast way to learn what is genuinely architecture-specific versus accidental coupling, and it forces a clean arch-abstraction boundary. This is **not rejected**; it is deferred. A future AArch64 port remains an explicit possibility, and the codebase should keep the arch boundary clean to preserve that option.

## Consequences

- **Legacy boot complexity is inherited.** x86-64 wakes in a primitive mode; the machine must be walked into long mode, and segmentation and the GDT come along whether wanted or not. The `bootloader` crate absorbs much of this initially.
- **Interrupt and paging mechanisms are fixed to the x86 world:** IDT + PIC/APIC for interrupts; `CR3`/`CR4` and 4-/5-level page tables for the MMU; `invlpg` for TLB invalidation.
- **The memory model is x86-TSO (strongly ordered).** This makes early concurrency more forgiving — but it also means the project will *not* naturally expose the weak-ordering hazards that AArch64 would. This is a known pedagogical gap; the guide suggests deliberately writing reordering-sensitive code (Phase 8) to compensate, and a later AArch64 port would surface it for real.
- **Existing assembly fluency is leveraged**, lowering the cost of context-switch and boot-path work.
- **Hardware discovery is the PC model:** PCI/PCIe enumeration and MSI/MSI-X, rather than device-tree parsing.
- **A clean architecture-abstraction boundary should be maintained** from early on to keep the future AArch64 port cheap. This is a design constraint that follows directly from leaving the port option open.

## References

- `os_guide.md` — Phase 0, "Instruction-set architecture: x86-64 vs AArch64".
- `docs/dev-journal/001-init.md` — decision 1, "Instruction Set Architecture: x86-64".
- Existing repo artifacts: `x86_64-rust_os.json`, `Cargo.toml` (`bootloader` crate).
