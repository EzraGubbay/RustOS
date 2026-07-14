# ADR-0006: In-Emulator Test Harness — `isa-debug-exit` + Serial Reporting

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

`os_guide.md` treats bare-metal testing as a first-class, cross-cutting concern: Phase 1 calls for "a bare-metal test harness: running assertions inside the emulator and reporting results back out," and the testing thread asks for "unit and integration tests that run inside the emulator." A `no_std` kernel ([ADR-0005](0005-freestanding-build-custom-target.md)) cannot use Rust's standard test harness, which depends on `std`. Two mechanisms are needed: a way for kernel-side test code to **signal a pass/fail exit status to the host**, and a way to **stream test output** off the machine.

The repo already configures this: `Cargo.toml` sets `test = true` on the binary and passes `-device isa-debug-exit,iobase=0xf4,iosize=0x04` as `test-args`.

## Decision

Run tests **inside QEMU** ([ADR-0003](0003-emulator-qemu.md)) using a custom test framework, with results reported two ways:

- **Exit status via the QEMU `isa-debug-exit` device** at `iobase=0xf4`. Writing a value to that port causes QEMU to exit with a status derived from the value, which the host test runner interprets as pass/fail. (A distinct success code is chosen so it can't be confused with QEMU's own exit codes.)
- **Test output via the serial port** ([ADR-0007](0007-diagnostics-lifeline.md)), captured by the host.

The custom framework is wired through Rust's `custom_test_frameworks` feature so `cargo test` drives the in-emulator run via the `bootimage` runner ([ADR-0004](0004-bootloader-crate.md)).

## Alternatives Considered

- **Rust's built-in `#[test]` harness.** Not available: it requires `std`. Rejected by necessity.
- **Triple-faulting / plain QEMU shutdown to signal exit.** Works, but yields a single opaque outcome and can't distinguish "tests passed" from "kernel crashed." `isa-debug-exit` gives an explicit, chosen status code and is the standard approach.
- **QEMU semihosting for exit/output.** A viable alternative channel (and more portable across architectures), but heavier to set up than `isa-debug-exit` + a 16550 UART on the PC platform, and less aligned with the phil-opp tutorials the project tracks. Reasonable to revisit if/when an AArch64 port needs a portable harness.
- **Testing only on real hardware.** Rejected: no automation, no fast feedback, and no clean status-reporting channel.

## Consequences

- **Automated `cargo test` on bare metal** — assertions run in the real target environment (long mode, real MMU behavior) rather than on the host, which catches bugs a host-side test never could.
- **A dedicated success exit code must be reserved** and kept distinct from QEMU's native codes so the runner interprets results correctly.
- **Coupling to QEMU and `bootimage`.** The harness depends on the `isa-debug-exit` device and the `bootimage` runner; a move to a different emulator or boot path (see [ADR-0004](0004-bootloader-crate.md)) would require re-plumbing it.
- **Serial is load-bearing for test output**, reinforcing the diagnostics decision in [ADR-0007](0007-diagnostics-lifeline.md).
- **`x86`-specific.** `isa-debug-exit` is a PC-platform device; a portable harness (e.g. semihosting) would be needed for other architectures.

## References

- `os_guide.md` — Phase 1, "A bare-metal test harness"; cross-cutting "Testing and debugging on bare metal".
- Existing repo artifacts: `Cargo.toml` (`test = true`, `test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]`).
