# ADR-0007: Diagnostics Lifeline — 16550 Serial + QEMU GDB Stub

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

Before memory management, interrupts, or any display driver exist, the kernel needs a way to say *something* — otherwise the only symptom of a failure is, as `os_guide.md` warns, "a silent reset." Phase 1 names "your first serial output" as "your single most important early tool," and Phase 0 prescribes "a debugging lifeline: the emulator's GDB stub, plus serial/semihosting output." This is the instrumentation every other subsystem is debugged through.

The repo already leans on serial: `Cargo.toml` runs QEMU with `-monitor stdio`, and the test harness ([ADR-0006](0006-in-emulator-test-harness.md)) reports through the serial channel.

## Decision

Adopt two complementary diagnostic channels as the project's default instrumentation:

- **Serial output over a 16550-family UART** as the primary text channel — for `print!`-style logging, panic messages, and test output. On the PC platform under QEMU this is the standard, minimal device that works before almost anything else is initialized. (A VGA text buffer may also exist as a convenience, but serial is the channel of record because it is capturable by the host and works headless.)
- **The QEMU GDB stub** as the interactive debugger — attach GDB to inspect registers, memory, and page tables and to single-step, especially for faults whose only other symptom would be a reset.

Serial is the *default trace instrument*; the GDB stub is the *deep-inspection tool*. Together they are the "lifeline."

## Alternatives Considered

- **VGA text buffer as the primary channel.** Simple and immediate, but not capturable by the host, useless headless, and gone the moment the project moves toward a framebuffer/GUI. Fine as a secondary convenience; not the channel of record.
- **QEMU semihosting for output.** A valid alternative (and more portable to ARM), but on the PC platform a 16550 UART is the conventional, well-documented path and integrates cleanly with `-monitor stdio` and the test harness. Revisit for a portable/AArch64 setup.
- **`printf`-debugging only, no GDB.** Rejected: some failures (triple faults, bad page-table state, early boot) are far cheaper to diagnose by attaching GDB to the stub than by bisecting with print statements.
- **Hardware debugging (JTAG) / real-serial-cable only.** Out of scope for the emulator-first loop ([ADR-0003](0003-emulator-qemu.md)); relevant only if/when real hardware becomes a serious target.

## Consequences

- **A working "hello" path exists from Phase 1**, and every later subsystem inherits a way to log and assert.
- **Headless, host-capturable output** — serial can be piped to a file, which is what makes the automated test harness ([ADR-0006](0006-in-emulator-test-harness.md)) possible.
- **The serial console is itself a data structure.** Per the guide's Phase 1 note, the UART path is a byte ring buffer feeding a device register; its shape and ownership should be designed deliberately, consistent with the DOD stance ([ADR-0009](0009-data-oriented-design-constraint.md)).
- **GDB fluency is a required skill**, not an optional extra — the guide treats reproducing and inspecting failures as its own discipline.
- **Somewhat x86/QEMU-specific.** The 16550 driver and the exact stub workflow would change on other hardware (PL011 UART on typical ARM), though the *approach* ports directly.

## References

- `os_guide.md` — Phase 0, "Toolchain and environment"; Phase 1, "first serial output" and the "Even the console is data" note; cross-cutting "Testing and debugging on bare metal".
- Existing repo artifacts: `Cargo.toml` (`run-args = ["-monitor", "stdio"]`).
