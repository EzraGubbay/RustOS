# ADR-0004: Bootloader — `bootloader` crate (v0.9, BIOS) + `bootimage`

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

On x86-64 the machine wakes in a primitive state, and something must walk it into 64-bit long mode, set up an initial GDT, and hand control to the kernel (`os_guide.md`, Phase 1). Writing that early boot shim by hand is a substantial detour from the theory the project exists to practice, and the guide explicitly lists "a bootloader crate, Limine, or multiboot2" as acceptable ways to have "a bootloader … hand control to your kernel already in long mode."

The repo already commits to a specific choice: `Cargo.toml` declares `bootloader = "0.9"`, and `.cargo/config.toml` sets the runner to `bootimage runner`.

## Decision

Use the Rust **`bootloader` crate, pinned at the `0.9.x` line**, together with the **`bootimage`** tool to assemble a bootable disk image. This is the classic **BIOS** boot path used by the phil-opp "Writing an OS in Rust" series, which is the project's primary x86 tutorial reference ([ADR-0002](0002-cpu-architecture-x86-64.md)).

## Alternatives Considered

- **Writing a custom bootloader / assembly boot stub.** Maximum understanding, but a large amount of legacy-mode assembly (real → protected → long mode, paging bring-up) that is orthogonal to the OS-theory goals. Deferred; can be revisited later as a deliberate learning exercise if desired.
- **`bootloader` 0.11+.** A significantly redesigned API with UEFI support and a different build model. Rejected for now purely because it diverges from the phil-opp tutorials the project is tracking; adopting it would mean fighting the tutorials during the most fragile early phase. Revisit if UEFI or the newer API becomes necessary.
- **Limine.** A capable, modern boot protocol with UEFI support and a clean handoff. A strong option, but it moves away from the Rust-crate-only, tutorial-aligned path and adds an external dependency to manage. Reasonable future migration target, not the starting point.
- **multiboot2 + GRUB.** The traditional hobby-OS route. Rejected for the same tutorial-alignment reason and the extra toolchain surface.

## Consequences

- **Long-mode handoff is free.** The kernel's first Rust instruction runs in 64-bit long mode with a usable GDT and a firmware memory map already available — exactly the "known-good state" Phase 1 targets.
- **BIOS, not UEFI.** The `0.9` line targets legacy BIOS boot. This is fine under QEMU and simplifies the early path, but it is a constraint to remember if real UEFI hardware ever enters scope — it would force a bootloader change.
- **Version lock-in to the phil-opp era.** Staying on `0.9.x` keeps the tutorials applicable but means the project is on an older, less-maintained API. A future migration to `bootloader` 0.11+ or Limine is a foreseeable, deliberate ADR-worthy step.
- **`bootimage` is a required build dependency**, coupling the run/test workflow to it (see [ADR-0006](0006-in-emulator-test-harness.md)).
- **Some boot detail is abstracted away.** The crate hides parts of the bring-up the project might otherwise learn by hand; the guide's "attempt before you read the answer" ethic suggests reading what the crate does, even while relying on it.

## References

- `os_guide.md` — Phase 1, "The boot path for your ISA".
- Existing repo artifacts: `Cargo.toml` (`bootloader = "0.9"`), `.cargo/config.toml` (`runner = "bootimage runner"`).
