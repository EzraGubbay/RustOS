---
title: "Data-Oriented Design — A Fast, Active Study Plan"
subtitle: "Mastering the DOD pattern vocabulary before Phase 1"
date: "July 2026"
status: learning-aid
---

# Purpose

The `os_guide.md` "Data-oriented grounding" section names seven core moves you want
"at your fingertips" before you write kernel code. This document is the fast path to
getting them there. It is **reading-first and drill-first**: for each concept you get
the single best short source, one no-code design drill you do in your journal, the
kernel table the concept will actually shape, and a one-line mastery check.

The whole thing is designed to be *fast* — the entire vocabulary is roughly a weekend
of focused reading plus the drills, not a course. The drills are what convert reading
into recall; skipping them is the usual reason DOD "doesn't stick."

## How to run this plan (the method)

The fastest durable way to learn a design vocabulary is **retrieval + application**,
not re-reading. So each concept follows the same loop:

1. **Read the one short source** listed (10–30 min each). Read once, actively — stop
   at each pattern and predict the cache/access consequence before the author states it.
2. **Do the drill immediately**, in `docs/dev-journal/`. Every drill is a *no-code*
   design kata: you write down tables, fields, keys, hot-vs-cold, and the dominant
   access pattern in prose. This is exactly the "design the data before the code"
   ritual the guide mandates — so the study *is* the project practice.
3. **Self-test cold** the next day: cover your notes, answer the mastery check from
   memory, and only then reveal the source. If you can't, that concept isn't learned yet.

Do the concepts roughly in the order below — each leans slightly on the previous.

# The Seven Core Concepts

## 1. Structure-of-Arrays vs Array-of-Structures (SoA vs AoS)

**What it is.** AoS stores one struct per element contiguously (`[{a,b,c}, {a,b,c}...]`);
SoA stores each field in its own array (`a[], b[], c[]`). SoA lets a loop that touches
only `a` stream `a` with no wasted cache-line bytes, eliminates per-struct padding, and
feeds the vector unit a clean stride.

**Fastest source.** Andrew Kelley, *A Practical Guide to Applying DOD* — the SoA /
padding-elimination segment. If reading over watching: Fabian's DOD book, the
"component"/existence chapters where the columnar layout is introduced.

**Active drill.** Take a plausible 8-field process control block. Write it out as AoS,
then as SoA. For three real kernel loops (scheduler pick, "reap zombies", "wake timers"),
mark which fields each loop reads and compute — in cache lines, not bytes — how much data
each layout drags through cache per iteration. Write the winner and *why* in one sentence.

**Kernel anchor.** The process/thread table and the run queue (Phase 4/6). The guide is
explicit: store PCB state as SoA.

**Mastery check.** Given an access pattern, state which layout wins and quantify it in
cache lines touched — without hand-waving.

## 2. Handles / Indices Instead of Pointers

**What it is.** Refer to objects by a small integer index (often index + generation
counter) into a table, not by raw pointer. Handles survive reallocation/compaction, are
half the size, are trivially serializable, and a generation field turns use-after-free
into a detectable stale-handle instead of a corruption.

**Fastest source.** floooh, *Handles are the better pointers* — short, complete, and the
exact pattern a memory-managing kernel wants. Reinforce with the indices segment of the
Kelley talk.

**Active drill.** Design a generational handle for one table (say the file-descriptor
table): specify the bit split between index and generation, what a "get" does on a stale
generation, and what allocation/free do to the generation counter. Then answer in prose:
what specific bug does the generation field catch that a bare index does not?

**Kernel anchor.** File-descriptor table, kernel object table, process table (Phases 5/6).
The guide notes a file descriptor number *is already a handle* — you're formalizing what
the OS already does.

**Mastery check.** Explain, without notes, why `fd = 3` plus a generation counter is safer
*and* faster than a `struct file *`.

## 3. Hot / Cold Data Splitting

**What it is.** Split a record's frequently-touched ("hot") fields from its rarely-touched
("cold") ones into separate arrays/tables, so the hot loop packs more live elements per
cache line and never pays to load cold bytes it won't read.

**Fastest source.** IT Hare, *Operation Costs in CPU Clock Cycles* — internalize that a
main-memory miss costs ~100–150 cycles while an L1 hit is a few; that number is the entire
justification. Then the hot/cold discussion in Fabian's book.

**Active drill.** For your PCB, label every field hot or cold against the scheduler's inner
loop specifically (not "in general"). Estimate how many live PCBs fit in a 64-byte line
before vs after the split. Note one field whose classification you're unsure of and what
measurement (from the Phase 0 counters) would settle it.

**Kernel anchor.** Scheduler PCB layout, allocator metadata (the guide calls allocator
metadata "hot data touched on every allocation").

**Mastery check.** Point at any struct and defend a hot/cold split boundary in terms of a
named inner loop and cache-line occupancy.

## 4. Existence-Based Processing

**What it is.** Represent "this object is in state X" by its *membership in a set/array*
rather than a per-object boolean you branch on. You iterate the set that exists; there is
no "is it active?" test and no branch misprediction, because absent things simply aren't in
the array.

**Fastest source.** Fabian's DOD book, the "Existence-Based Processing" chapter — this is
its home turf.

**Active drill.** Take "runnable vs blocked threads." Design it two ways: (a) one array with
a `runnable: bool` per thread that the scheduler branches on; (b) two disjoint sets/lists
where a thread's presence *is* its state. For the scheduler's hot loop, write down what work
and branches each does per tick, and what state-transition (block/wake) costs in each.

**Kernel anchor.** Run queue vs wait queue, ready lists, any "active/dirty/valid" flag
(Phases 4/5/7).

**Mastery check.** Convert a given per-object boolean into a set-membership design and state
what the hot loop stops doing as a result.

## 5. Out-of-Band Booleans

**What it is.** Don't store a bool inside the hot struct (where it wastes a byte + padding
and pollutes the cache line); hoist it into a separate parallel array or a bitset. A packed
bitset puts 64 flags in 8 bytes and is scannable/maskable in bulk.

**Fastest source.** The out-of-band-booleans segment of the Kelley talk — he treats this
directly. (Closely related to #4; learn them as a pair.)

**Active drill.** Pick a subsystem with several parallel flags (page frame: `free`,
`reserved`, `dirty`, `pinned`). Design them as (a) bytes inside the frame struct and (b) four
parallel bitsets. Write how "count all free frames" and "find first free frame" execute in
each, and note where SIMD/word-at-a-time scanning becomes possible.

**Kernel anchor.** Frame allocator bitmap (Phase 3), any per-object status flag. The frame
bitmap is out-of-band booleans by construction — connect the concept to it explicitly.

**Mastery check.** Explain when a bit in a bitset beats a `bool` field, and when it doesn't
(single flag, always read alongside the struct).

## 6. Encodings Over Polymorphism

**What it is.** Replace a vtable/`dyn`/inheritance hierarchy with a small **tag** (enum) plus
a `switch`. This removes the per-object pointer indirection, keeps dispatch data-dense and
predictable, and lets you batch all objects of one tag together.

**Fastest source.** The "encodings instead of polymorphism" segment of the Kelley talk;
Fabian's book for the mindset behind it.

**Active drill.** Model kernel "objects" a user can wait on (pipe, semaphore, socket-like,
timer). Sketch (a) a trait-object / vtable design and (b) a tag + `switch` over columnar
per-type data. Answer: which makes "process every ready pipe" a tight batched loop, and what
does the vtable version cost per object?

**Kernel anchor.** Kernel object / syscall dispatch table, VFS node types (Phases 5/7). Ties
back to the IDT insight — dispatch tables want to be flat arrays indexed by a tag, not chains
of virtual calls.

**Mastery check.** Take a small class hierarchy and re-express it as tag + switch, naming the
indirection you removed.

## 7. The Relational / Normalized Model of State

**What it is.** Treat all kernel state as **tables** you join, index, and query — with
normalization (no duplicated facts, each fact stored once, related by key/handle). This is
the unifying frame: SoA is a column store, handles are foreign keys, existence-based
processing is a `WHERE`, hot/cold is vertical partitioning. Once you see the kernel as a
database, the other six are just its techniques.

**Fastest source.** Fabian's DOD book, the relational-model chapters — the guide singles this
out as mapping "almost directly onto kernel subsystems." Read this last; it retro-fits meaning
onto everything above.

**Active drill.** Draw the kernel's core state as a normalized schema: processes, threads,
open files, and the file/inode they point to. Identify the keys (handles), the foreign-key
relationships, and one place naive design would duplicate a fact (e.g. file offset vs inode
size). Then map each of concepts #1–#6 onto a spot in your schema.

**Kernel anchor.** Everything — process table, page tables, FD table, inode/dentry cache. The
guide's core claim is that a kernel is *already* a pile of tables.

**Mastery check.** For any subsystem, produce the table(s), the keys, and one normalization
you'd enforce — in under two minutes, in your journal.

# Suggested Sequence (fast pass)

1. **Ground the "why" (30 min):** IT Hare operation-costs infographic. Every later decision
   is justified by these latency numbers; read it first so nothing feels arbitrary.
2. **Get the pattern reflexes (60–90 min):** the Kelley talk end-to-end — it covers SoA,
   indices, out-of-band booleans, and encodings in one pass (concepts 1, 2, 5, 6).
3. **Handles deep cut (15 min):** the floooh essay (concept 2).
4. **The two "existence" ideas (reading):** Fabian's existence-based-processing chapter
   (concepts 4, 5 as a pair).
5. **The unifying frame (reading):** Fabian's relational chapters (concepts 7, 3).
6. **Optional mindset capstone:** Mike Acton's CppCon 2014 talk — the philosophy, best
   watched *after* you have the concrete patterns so it lands as synthesis, not intro.

Do the matching drill after each step, not at the end. Total: a focused weekend.

# Active-Recall Test (your exit criteria)

You've internalized the vocabulary when you can, cold and without notes:

- Name all seven moves and give the one-line access-pattern reason each exists.
- Take an arbitrary struct and produce its SoA layout + a hot/cold split justified by a
  named inner loop.
- Design a generational handle and say what bug the generation catches.
- Convert a per-object boolean into either set-membership or a bitset and say what the hot
  loop stops doing.
- Re-express a small class hierarchy as tag + switch.
- Draw any kernel subsystem as a normalized table schema with keyed relationships.

When the *first thing* you reach for on a new subsystem is "what are the tables, keys, and
hot fields?" — the grounding phase has done its job, and you're ready to design Phase 1's
data before writing a line of it.

# Source Quick-Reference

| Concept | Primary short source |
|---|---|
| SoA vs AoS | Kelley talk (SoA/padding segment); Fabian, component chapters |
| Handles/indices | floooh, "Handles are the better pointers"; Kelley (indices) |
| Hot/cold splitting | IT Hare, "Operation Costs in CPU Clock Cycles"; Fabian |
| Existence-based processing | Fabian, "Existence-Based Processing" chapter |
| Out-of-band booleans | Kelley talk (out-of-band booleans segment) |
| Encodings over polymorphism | Kelley talk (encodings segment); Fabian |
| Relational/normalized model | Fabian, relational-model chapters |

Full URLs live in `os_guide.md` → Resource Atlas → "Data-oriented design."
