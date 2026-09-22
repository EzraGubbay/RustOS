# RustOS

RustOS is an educational x86-64 kernel, written from scratch in Rust and developed under QEMU. I am building it for three reasons: to work through operating-systems internals by hand, putting the theory from a university OS course into practice; to learn Rust at the bare-metal level, where there is no runtime or heap and the ownership model has to be reconciled with memory-mapped hardware and global mutable state; and to explore data-oriented design, which the project treats as a constraint on every subsystem rather than a late optimization. It is not a usable operating system: today it boots and prints one line of text. It is a study project, worked through with documentation and LLM assistance; under the rules in [`AGENTS.md`](AGENTS.md), the assistant explains, plans and drafts documents, and the kernel code is written by hand.

## Current state

The kernel boots under QEMU, writes `Hello World!` to the VGA text console and then spins in an empty `loop {}`. It has no exception or interrupt handling, no serial output, no memory management of its own and no heap. The bring-up so far follows the early chapters of Philipp Oppermann's *Writing an OS in Rust*, which the ADRs adopt as the project's x86 reference.

- **Target.** [`x86_64-rust_os.json`](x86_64-rust_os.json) is a custom target spec on LLVM's `x86_64-unknown-none`: `os: none`, linked by `rust-lld`, `panic-strategy: abort`, `disable-redzone: true`, and `features: -mmx,-sse,+soft-float` with `rustc-abi: softfloat`, so the compiler emits no MMX or SSE code and does floating point in software.
- **Build.** [`.cargo/config.toml`](.cargo/config.toml) makes that file the default target, enables `json-target-spec` (the nightly opt-in that lets Cargo accept a `.json` target at all), and uses `build-std` to compile `core` and `compiler_builtins` from source, with `compiler-builtins-mem` supplying `memcpy`, `memset` and the other memory routines a C library would normally provide.
- **Boot.** `bootloader` 0.9 (0.9.35 in `Cargo.lock`) supplies a BIOS boot path, and `bootimage` builds it around the kernel ELF into a raw disk image. The bootloader walks the CPU from real mode into 64-bit long mode, so paging is already on, using the bootloader's page tables, when it jumps to `_start`. I borrowed this path because writing the real-to-long-mode transition by hand is a detour from the OS theory the project is for, and the 0.9 line keeps the tutorial applicable; the cost is BIOS-only boot and an older API ([ADR-0004](docs/adr/0004-bootloader-crate.md)).
- **Kernel.** [`src/main.rs`](src/main.rs) is `#![no_std]` and `#![no_main]`. Its entry point, `_start`, is exported unmangled with the C calling convention, prints one line and spins; it takes no arguments, so the memory map the bootloader provides goes unread for now. The panic handler prints the `PanicInfo` and spins. There is no linker script yet, so the kernel is linked at the linker's default address in the lower half, not in the higher half proposed in [ADR-0008](docs/adr/0008-higher-half-kernel-layout.md).
- **Console.** [`src/vga_buffer.rs`](src/vga_buffer.rs) writes the 80×25 VGA text buffer at `0xb8000` through `Volatile` cells, each a `#[repr(C)]` pair of an ASCII byte and a color attribute. Text goes on the bottom row, which scrolls up on newline or wrap, and bytes outside printable ASCII appear as `■`. A global `WRITER`, a `spin::Mutex` initialized with `lazy_static!`, backs crate-level `print!` and `println!` macros.
- **Tests.** `custom_test_frameworks` stands in for the `std` test harness: under `cargo test`, `_start` calls the generated `test_main`, which passes every `#[test_case]` function to the minimal `test_runner` in `src/main.rs` (there is one test, `trivial_assertion`). Test runs attach QEMU's `isa-debug-exit` device at I/O port `0xf4`, but nothing writes to that port yet and results appear only on the VGA screen, so QEMU never exits by itself: bootimage kills it at its 300-second test timeout and `cargo test` reports a failure. Exit codes and serial output, as [ADR-0006](docs/adr/0006-in-emulator-test-harness.md) describes, are the next step.

## Building and running

Four things come from outside the repository:

- a nightly Rust toolchain; nothing in the repo selects one (there is no `rust-toolchain` file), so set a directory override;
- the `rust-src` component, which `build-std` compiles `core` from, and `llvm-tools`, which the bootloader's build script uses;
- `bootimage`, which assembles the disk image and acts as Cargo's runner;
- QEMU, with `qemu-system-x86_64` on your `PATH` (`brew install qemu` on macOS, `apt install qemu-system-x86` on Debian or Ubuntu).

```sh
rustup toolchain install nightly
rustup override set nightly        # run in the repository root
rustup component add rust-src llvm-tools --toolchain nightly
cargo install bootimage
```

Then boot the kernel:

```sh
cargo run
```

Cargo builds the kernel for `x86_64-rust_os.json` and, because that target's `os` is `none`, hands the ELF to the `bootimage runner` configured in `.cargo/config.toml`. The runner builds the bootloader around it, writes the disk image and starts `qemu-system-x86_64 -drive format=raw,file=<image> -monitor stdio`, which is bootimage's default command plus the `run-args` from `Cargo.toml`. The VGA console appears in the QEMU window. The terminal holds QEMU's monitor, not a serial console; type `quit` there to exit.

The individual steps are also available:

```sh
cargo build       # kernel ELF: target/x86_64-rust_os/debug/RustOS
cargo bootimage   # disk image: target/x86_64-rust_os/debug/bootimage-RustOS.bin
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os/debug/bootimage-RustOS.bin
cargo test        # boots the test build; ends at bootimage's 300 s timeout (see Tests above)
```

No nightly is pinned yet, although [ADR-0005](docs/adr/0005-freestanding-build-custom-target.md) says one should be. Custom target specs and `build-std` are unstable and drift between nightlies, so a much newer nightly may need changes to `x86_64-rust_os.json`.

## Design notes

### A custom target spec, and what `no_std` costs

The obvious alternative is rustc's built-in `x86_64-unknown-none` target, which [ADR-0005](docs/adr/0005-freestanding-build-custom-target.md) records as viable and simpler. I kept a spec of my own so that every property the kernel relies on is written down in the repository, where a diff shows it, rather than inside the compiler. The lines that matter each answer a concrete problem:

- **`disable-redzone: true`.** The System V x86-64 ABI lets a leaf function keep up to 128 bytes of data below `rsp` without moving `rsp`. An interrupt or exception taken in ring 0 pushes its frame onto that same stack, straight over the red zone. The kernel takes no interrupts yet, but it will, so the red zone is off from the start.
- **`-mmx,-sse,+soft-float` with `rustc-abi: softfloat`.** If kernel code used vector registers, every interrupt and context switch would have to save and restore that state. I've deferred that machinery to the guide's SIMD phase, so the compiler uses only general-purpose registers, does floating point in software and passes nothing in FPU or vector registers.
- **`panic-strategy: abort`.** There is no runtime to unwind into, so a panic stops in the panic handler instead of unwinding the stack.

The custom spec needs a nightly toolchain, because JSON targets and `build-std` are unstable, and it makes clean builds slower, because no prebuilt `core` exists for a target defined in this repository.

Building without `std` gives up more than the standard library: there is no `main` and no runtime to call it, so the kernel provides `_start`; no C library, so `compiler-builtins-mem` supplies the memory routines LLVM emits calls to; no panic machinery, so the kernel defines its own panic handler and aborts; no heap, so no `alloc` until a global allocator exists; no `std` test harness, so tests rely on the unstable `custom_test_frameworks`; and no hardware floating point. Every dependency has to be `no_std` as well, which is why `lazy_static` is built with its `spin_no_std` feature.

### One `unsafe` block, behind a spin lock

The console writer is where Rust's ownership rules first met the hardware. `print!` has to work anywhere, including the panic handler, so the writer is global. A `static` has to be `Sync`, and with no OS to block on, the lock is a spinlock, `spin::Mutex`. The writer holds a `&'static mut` made from the raw address `0xb8000`, which compile-time evaluation cannot create, so `lazy_static!` builds it on first use. The compiler would be entitled to delete stores to memory that is never read back, so every cell goes through `Volatile`. The result is that the crate's source has one `unsafe` block, that cast, and everything else reaches the screen through `WRITER.lock()`: the discipline [ADR-0001](docs/adr/0001-programming-language-rust.md) asks for, with `unsafe` kept small and wrapped in a safe interface.

The same lock is the first concurrency bug waiting to happen. Once interrupts exist, a handler that prints while the code it interrupted holds `WRITER` will spin forever, which is the handler-versus-mainline hazard the guide raises in Phase 2.

### Data layout before code

[ADR-0009](docs/adr/0009-data-oriented-design-constraint.md) makes data-oriented design a constraint on every subsystem: its tables, keys, hot and cold fields and dominant access pattern are written down before any code. None of the subsystems this is aimed at exist yet, so the work so far is on paper. [Journal entry 002](docs/dev-journal/002-studying-dod.md) designs the data plane for threads and the objects they block on as normalized tables. Waitable objects are named by 64-bit generational handles (32-bit index, 32-bit generation), so a stale handle fails a comparison instead of reaching whatever reused its slot. Threads live in one array per scheduling state, so picking the next runnable thread reads two columns instead of scanning every thread. The runnable set is split per CPU, so cores pick from disjoint arrays.

## Roadmap

In dependency order, taken from the capability checkpoints in [`docs/os_guide.md`](docs/os_guide.md) and from the ADRs:

1. **Serial output and a real test result.** A 16550 UART driver as the channel of record ([ADR-0007](docs/adr/0007-diagnostics-lifeline.md)), then test output over serial and a pass/fail exit through `isa-debug-exit`.
2. **Measurement.** Cycle counters, cache- and TLB-miss counters and a micro-benchmark harness, which ADR-0009 wants in place before any data-layout claim is made.
3. **Exceptions and interrupts.** An IDT, CPU exception handlers and a separate known-good stack so a double fault is caught instead of becoming a triple-fault reset, then the interrupt controller, the timer and keyboard input.
4. **Physical memory.** Read the bootloader's memory map and hand out frames. The frame-allocator design (bitmap, free list or buddy) is still open.
5. **Paging.** Map and unmap pages, move the kernel into the higher half (ADR-0008), and choose between a physical-memory offset map and recursive mapping for reaching frames.
6. **Kernel heap.** Bump, then free-list, then slab allocation behind Rust's global-allocator interface, which makes `alloc` available.

After that come kernel threads and scheduling, synchronization and SMP, IPC, userspace and system calls, storage and a filesystem, and SIMD with vector-state handling. A GUI is an optional capstone at the very end ([ADR-0010](docs/adr/0010-cli-first-gui-deferred.md)).

## References

- Philipp Oppermann, *Writing an OS in Rust*, <https://os.phil-opp.com>. The bring-up in `src/` follows its early chapters.
- [`docs/os-syllabus.pdf`](docs/os-syllabus.pdf): the syllabus of Bar-Ilan University's Operating Systems course (89231), whose theory this project puts into practice. Its textbook is Silberschatz, Galvin and Gagne, *Operating System Concepts*, 10th edition.
- [`docs/os_guide.md`](docs/os_guide.md): the study guide that orders the work into phases and capability checkpoints, also rendered as [`docs/Data-Oriented-OS-Study-Guide.pdf`](docs/Data-Oriented-OS-Study-Guide.pdf). Its Resource Atlas is the wider reading list, from the Intel SDM and the OSDev wiki to OSTEP, xv6 and Richard Fabian's *Data-Oriented Design*.
- [`docs/dod-study-plan.md`](docs/dod-study-plan.md) and Andrew Kelley's lecture on data-oriented design, <https://youtu.be/IroPQ150F6c>, which journal entry 002 records studying.
- Cutler et al., "The benefits and costs of writing a POSIX kernel in a high-level language" (Biscuit, OSDI '18), <https://pdos.csail.mit.edu/papers/biscuit.pdf>, cited in ADR-0001 on the cost of garbage collection in a kernel.
- [`docs/adr/`](docs/adr/README.md) and [`docs/dev-journal/`](docs/dev-journal/): the decisions behind the project and the reasoning that led to them.
