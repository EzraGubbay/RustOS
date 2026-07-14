# ADR-0003: Primary Emulator — QEMU (`qemu-system-x86_64`)

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

`os_guide.md` (Phase 0) prescribes "an emulator as your primary development machine" together with "a debugging lifeline: the emulator's GDB stub, plus serial/semihosting output." A kernel cannot be iterated on productively via reboots of real hardware; the tight edit-build-run loop lives in an emulator. The project also needs a bare-metal test harness that runs assertions inside the emulator and reports results back out (Phase 1, and the cross-cutting testing thread).

Because the target ISA is x86-64 ([ADR-0002](0002-cpu-architecture-x86-64.md)), the emulator must faithfully model the PC platform: the `q35` chipset, a 16550-family UART for serial output, and a mechanism to exit with a status code so tests can report pass/fail.

## Decision

Use **QEMU**, specifically **`qemu-system-x86_64`**, as the primary development and test machine. Run against the **`q35`** machine type. The existing project configuration already reflects this:

- The kernel is launched via the `bootimage runner` (`.cargo/config.toml`).
- QEMU is run with `-monitor stdio` for interactive control (`Cargo.toml` → `package.metadata.bootimage.run-args`).
- The in-emulator test harness uses the `isa-debug-exit` device (`iobase=0xf4,iosize=0x04`) to return a status code to the host (`test-args`).
- Serial output over the emulated UART is the default diagnostic channel.

## Alternatives Considered

- **Bochs.** A meticulously accurate x86 emulator with strong introspection. Rejected as the *primary* loop: slower iteration and a smaller share of the Rust-OS tutorial ecosystem than QEMU. It remains a useful secondary tool for hard-to-diagnose x86-accuracy bugs.
- **A hardware-accelerated hypervisor (KVM / Hypervisor.framework / WHPX).** Faster execution, but virtualization accelerators diverge from a plain emulated PC in ways that can mask bugs and complicate deterministic debugging. QEMU's software emulation with its GDB stub is the better teaching and debugging substrate; acceleration can be enabled later purely for speed if needed.
- **Running only on real hardware.** Rejected as a primary loop for the obvious reason — no fast iteration, and, per the guide, "the only symptom" of many failures "is a silent reset," which is far cheaper to diagnose against QEMU's GDB stub than on a board. Real hardware still matters as a periodic reality check, but the choice of a physical board is deferred and not yet recorded as an ADR.

## Consequences

- **Fast, scriptable iteration** with a built-in GDB stub, which the guide names as the debugging lifeline.
- **A working automated test path** already exists: `isa-debug-exit` lets in-emulator assertions report a status code to the host, enabling `cargo test`-style workflows on bare metal.
- **Serial-first diagnostics** are the natural default, matching the guide's Phase 1 emphasis on UART output as "your single most important early tool."
- **Over-fitting risk.** QEMU is forgiving; behavior that works under emulation can still fail on silicon. The guide explicitly warns against over-fitting the emulator's forgiving behavior — mitigated by periodic runs on real hardware.
- **Version and machine-type coupling.** Test and run behavior depend on the QEMU version and the `q35` machine model; these should be recorded so results stay reproducible.
- **`qemu-system-x86_64` is the correct binary** given the x86-64 target; an AArch64 port would instead use `qemu-system-aarch64` with the `virt` machine, which is out of scope here.

## References

- `os_guide.md` — Phase 0, "Toolchain and environment"; Phase 1, "first serial output" and "bare-metal test harness"; cross-cutting "Testing and debugging on bare metal".
- Existing repo artifacts: `.cargo/config.toml` (`bootimage runner`), `Cargo.toml` (`run-args`, `test-args`).
