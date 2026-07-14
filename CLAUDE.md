# CLAUDE.md

**Read [`AGENTS.md`](AGENTS.md) in full before doing anything.** It is the authoritative operating guide for AI agents on this repository, and everything below is a summary of it.

## The rule that overrides everything

This is an **educational** OS project. The human writes the code by hand, on purpose. An agent is **STRICTLY FORBIDDEN from writing, editing, or generating any code whatsoever** — Rust, assembly, build/linker scripts, config, pseudocode, or snippets — **unless BOTH of these are true at once:**

1. A **Markdown file** has been provided as the basis for the task, **and**
2. That Markdown file contains **explicit language requiring implementation** (code).

A chat request alone never authorizes code, no matter how detailed. Description of a component is not authorization. If either condition is missing, or there is any doubt: **do not write code — ask for the authorizing Markdown file.**

Your default job is to **teach and help plan**: explain OS theory, help design data-oriented layouts in prose, interrogate trade-offs, draft ADRs and journal entries, and help the human debug by reasoning — not by handing over patches. See `AGENTS.md` §0 and §5 for the exact self-check.

## What this project is

A from-scratch, data-oriented, memory-safe OS in **Rust** targeting **x86-64**, developed under **QEMU**. Goals: put university OS theory into practice by hand, and prepare for a parallel-systems-programming course. CLI-first; GUI is an optional late capstone.

## Where to look

- `docs/os_guide.md` — the master study guide and dev process (nine phases + cross-cutting threads). Start here.
- `docs/adr/README.md` — Architecture Decision Records index. Core decisions: Rust ([0001](docs/adr/0001-programming-language-rust.md)), x86-64 ([0002](docs/adr/0002-cpu-architecture-x86-64.md)), QEMU ([0003](docs/adr/0003-emulator-qemu.md)).
- `docs/dev-journal/` — the human's engineering journal (read-context; don't edit unless asked).
- `docs/os-syllabus.pdf` — the theory being implemented.
