# ADR-0009: Data-Oriented Design as a First-Class, Strict Constraint

- Status: Accepted
- Date: 2026-07-14
- Deciders: Ezra

## Context

Data-oriented design (DOD) is not an optimization the project intends to apply late — it is one of the project's founding commitments. `os_guide.md` states DOD is "treated as a first-class constraint on every subsystem, not a late optimization pass," and warns that "retrofitting DOD is painful," so the pattern vocabulary is front-loaded. The dev journal names DOD as "an important aspect of this project" and records that Zig was seriously considered specifically for its native DOD support before Rust was chosen ([ADR-0001](0001-programming-language-rust.md)).

The guide's key insight is that a kernel is *already* relational: the process table, page tables, file-descriptor table, and inode cache are tables, and a file-descriptor number is already a handle. So DOD is the native idiom of kernel data, not an imposition.

## Decision

Adopt **strict, design-first data-oriented design as a cross-cutting constraint on every subsystem**, applied from the first line rather than retrofitted. Concretely, this means:

- **Design the data before the code, every time.** For each subsystem, the tables, per-row contents, keying, hot-vs-cold field split, and dominant access pattern are written down (in the journal) *before* any implementation.
- **Reach for a named pattern** rather than reasoning from scratch: struct-of-arrays over array-of-structures; **handles/indices instead of pointers** (with generational indices where use-after-free matters); **hot/cold splitting**; **existence-based processing** (set membership over per-object boolean flags); out-of-band booleans; **encodings over polymorphism** (tag + switch instead of a vtable); and a **relational, normalized** view of kernel state.
- **Measure, don't guess.** Stand up cycle counters and cache-/TLB-miss counters early; a DOD claim that isn't measured is a guess.
- **Know when *not* to bother.** DOD is for the hot path; genuinely cold, rarely-touched code does not earn the added complexity. This is a discipline, not a religion.

## Alternatives Considered

- **Conventional object-oriented / pointer-graph design.** The default in most kernels and most Rust code. Rejected as the governing style because it fights the project's performance goals and, notably, fights Rust's ownership model — pointer-graph designs are exactly what the borrow checker pushes back on, so DOD's handle/index layouts and the language's safety discipline pull in the same direction.
- **Apply DOD later as an optimization pass.** Explicitly rejected by the guide: retrofitting DOD is painful and tends not to happen. Front-loading the discipline is the whole point.
- **Adopt Zig for native struct-of-arrays support (`MultiArrayList`).** Considered and rejected in [ADR-0001](0001-programming-language-rust.md); the consequence is that DOD patterns are built by hand in Rust, which this ADR commits to doing deliberately.

## Consequences

- **Every subsystem starts from its data.** ADRs and journal entries for new components should lead with the data-layout design; this is a review expectation, not a nicety.
- **Manual pattern implementation.** Without Zig's built-ins, SoA containers, generational handle tables, and hot/cold splits are hand-rolled and maintained — accepted, and itself instructive.
- **Measurement infrastructure is a prerequisite**, not an afterthought (guide Phase 0). Performance counters and a micro-benchmark harness should exist early.
- **Alignment with safety and with the domain.** The relational framing maps directly onto kernel tables, and the handle/index style is both DOD-idiomatic and borrow-checker-friendly.
- **A guardrail against overuse.** Cold paths are explicitly exempt; the constraint is scoped to where it pays off.
- **This constraint interacts with essentially every other ADR** — allocators, page tables, the console ring buffer, scheduler run queues, and the process/descriptor tables are all to be modeled as tables.

## References

- `os_guide.md` — Phase 0, "Data-oriented grounding"; the cross-cutting "Data-oriented design as a through-line"; Resource Atlas, "Data-oriented design".
- `docs/dev-journal/001-init.md` — decision 2 (DOD as motivation; Zig considered for it).
