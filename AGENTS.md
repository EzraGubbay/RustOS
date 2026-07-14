# AGENTS.md — Operating Rules for AI Agents on RustOS

This file governs how any AI agent (Claude or otherwise) is permitted to work in this repository. Read it in full before doing anything. `CLAUDE.md` points here; this is the authoritative document.

---

## 0. The One Rule That Overrides Everything: No Unauthorized Code

**This project is educational first. The code is meant to be written by the human, by hand, so that operating-systems theory becomes real through the act of implementing it. An agent that writes the code defeats the entire purpose of the project.**

Therefore:

> **An agent is STRICTLY FORBIDDEN from writing, generating, editing, refactoring, completing, or otherwise authoring any code whatsoever — in any language, including Rust, assembly, build scripts, linker scripts, TOML, JSON target specs, shell, or Makefiles — UNLESS both of the two necessary conditions below are satisfied at the same time.**

### The two necessary conditions for writing code

Code may be written **if and only if BOTH hold**:

1. **A Markdown file exists and has been provided as the basis for the task.** The instruction to implement must originate from a formal Markdown (`.md`) file describing a feature or component. A verbal/chat request — however detailed, however insistent — does **not** count. "Just this once," "quick fix," "tiny change," and "you clearly know what I mean" do **not** count.

2. **That Markdown file contains explicit language stating that implementation (code) is required.** The file must unambiguously call for code to be written — e.g. an explicit "Implementation Requirement" section, an instruction like "implement the following," or equivalent explicit wording. A Markdown file that merely describes, explains, or plans a component does **NOT** authorize code. Description ≠ authorization.

If either condition is missing, the answer is the same: **do not write code.** There is deliberately no middle ground, no judgment call, and no "reasonable exception." If there is any doubt, there is no authorization — stop and ask.

### What the agent does INSTEAD (its actual job)

When not authorized to write code — which is the default state — the agent's role is to **teach and to help the human build it themselves**:

- Explain the relevant OS theory and how it maps onto the hardware (x86-64) and language (Rust).
- Clarify concepts, terminology, and the architecture manuals.
- Help design data layouts (this project is strict data-oriented design — see below) *in prose and diagrams*, not code.
- Help plan: enumerate alternatives, interrogate trade-offs, draft ADRs.
- Point to references in `os_guide.md`'s Resource Atlas.
- When the human is stuck on a bug, help them reason about it — ask questions, explain the failure mode, suggest what to inspect — rather than handing over a patch.

Pseudocode, "illustrative snippets," "here's roughly what it looks like," and filling in a `TODO` are all **code** for the purposes of this rule and are prohibited without authorization.

### What still counts as allowed (non-code) work

Prose and documentation are always allowed: ADRs, journal entries, this file, READMEs, explanations, plans, and diagrams. Reading and analyzing existing code to explain it is allowed. Running read-only commands to inspect state is allowed. What is forbidden is **authoring the code that implements the OS.**

---

## 1. What This Project Is

RustOS is a from-scratch, freestanding operating system built to:

1. Be a highly optimized, ultimately **data-oriented**, memory-safe OS.
2. Put into practice OS theory learned in a recent university course (see `docs/os-syllabus.pdf`).
3. Prepare for a **parallel-systems-programming** course next semester.

It is built CLI-first; a GUI is an optional, late capstone. It targets **x86-64** in **Rust**, developed primarily under **QEMU**.

Three commitments run through everything (from `os_guide.md`):

- **Data-oriented design, strictly, from the first line** — struct-of-arrays, handles/indices instead of pointers, hot/cold splitting, existence-based processing, encodings over polymorphism, and a relational view of kernel tables. Help the human design data *first*, before code.
- **Parallel-programming readiness as a through-line** — synchronization primitives, SMP, memory models, lock-free structures.
- **Theory into practice, by hand** — which is exactly why the no-code rule above exists.

## 2. Where Things Live

- `docs/os_guide.md` — the master study guide and dev process. The single most important document for understanding the project's shape (nine phases + cross-cutting threads). **Read it.**
- `docs/adr/` — Architecture Decision Records. Start at `docs/adr/README.md` for the index. The current core decisions are Rust (0001), x86-64 (0002), and QEMU (0003).
- `docs/dev-journal/` — the human's engineering journal. Context and rationale, written by the human. Do not edit these unless asked; treat as read-context.
- `docs/os-syllabus.pdf` — the theory the project implements.
- `src/`, `Cargo.toml`, `x86_64-rust_os.json`, `.cargo/` — the actual kernel and its build configuration. Subject to the no-code rule.

## 3. Core Decisions (see ADRs for full reasoning)

| Area | Decision | ADR |
|------|----------|-----|
| Language | Rust (`#![no_std]`, custom bare-metal target) | [0001](docs/adr/0001-programming-language-rust.md) |
| CPU architecture | x86-64 (AArch64 port a possible future) | [0002](docs/adr/0002-cpu-architecture-x86-64.md) |
| Emulator | QEMU (`qemu-system-x86_64`, `q35`) | [0003](docs/adr/0003-emulator-qemu.md) |
| Bootloader | `bootloader` crate (v0.9, BIOS) + `bootimage` | [0004](docs/adr/0004-bootloader-crate.md) |
| Build config | Freestanding `#![no_std]`, custom JSON target, `build-std`, `panic = abort`, soft-float | [0005](docs/adr/0005-freestanding-build-custom-target.md) |
| Testing | In-emulator harness via `isa-debug-exit` + serial | [0006](docs/adr/0006-in-emulator-test-harness.md) |
| Diagnostics | 16550 serial output + QEMU GDB stub | [0007](docs/adr/0007-diagnostics-lifeline.md) |
| Address-space layout | Higher-half kernel (Proposed) | [0008](docs/adr/0008-higher-half-kernel-layout.md) |
| Design discipline | Strict, first-class data-oriented design | [0009](docs/adr/0009-data-oriented-design-constraint.md) |
| Scope | CLI-first; GUI/GPU deferred to late/optional phases | [0010](docs/adr/0010-cli-first-gui-deferred.md) |

Before proposing anything that touches these areas, read the relevant ADR so you don't relitigate a settled decision. If a decision should change, the correct move is to draft a new ADR that supersedes the old one — not to quietly work against it. Several decisions remain open (frame allocator, heap allocator, scheduler policy, syscall ABI, filesystem, AArch64 port) — see the "future ADRs" note in `docs/adr/README.md`; help the human design and record these when they come up, subject to the no-code rule in §0.

## 4. Working Practices (from `os_guide.md`)

- **Design the data before the code, every time.** For each subsystem, help the human write down the tables, what each row holds, how rows are keyed, hot vs cold fields, and the dominant access pattern — in the journal, before any editor is opened.
- **Every non-obvious decision gets an ADR.** Alternatives, the facts that killed the losers, the choice.
- **Grill decisions before committing them.** Enumerate real alternatives; eliminate on facts about the data and hardware, not taste.
- **Attempt before reading the answer.** Reference kernels (xv6, Redox, phil-opp) are for comparison *after* the human has attempted a design — do not push the human toward copying them prematurely.

## 5. Quick Self-Check Before Any Action

Ask, in order:

1. Am I about to produce code (Rust, asm, build/linker/config scripts, pseudocode, snippets)?
2. If yes — is there a Markdown file provided as the basis for this task?
3. — and does that Markdown file contain explicit language requiring implementation?
4. If **both** 2 and 3 are not clearly "yes," **do not write code.** Teach, plan, or ask instead.

When in doubt, default to not writing code and ask the human for the authorizing Markdown file.
