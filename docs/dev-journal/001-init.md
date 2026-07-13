# Introduction
I've decided to build a useable OS with many features, to put into practice the theory I learned in the Operating Systems course I took recently as part of my degree.

This project also serves largely as a preliminary preparation for next semester, where I plan to take a course in parallel systems programming.

I decided to keep an engineering journal to document my architectural decisions, thought processes and potentially any challenges and interesting insights I collect along the way.

## Architectural Decisions

Following a discussion with Claude and the production of the [OS Guide](../os_guide.md), I've come to the following decisions regarding the OS I will be creating.

### 1. Instruction Set Architecture: `x86-64`.
While I feel ARM is better in some respects, I have chosen this architecture instead, primarily because I am both already partly familiar with it from previous academic courses, and I think this will be the ISA assumed for the CPU in the parallel programming course. I'm not sure how relevant the latter reason is, but in any case I might implement a comparable ARM version in the future.

### 2. Programming Language: **Rust**.
I have chosen `Rust` for a few reasons:
- Safety - less concerns surrounding memory allocations, use-after-free and the like.
- Relevance to parallel programming and Machine Learning - topics of particular interest to me.
- Allows for significant optimization, and does not have a Garbage Collector.

Another option I considered was Zig for its Data-Oriented Design approach, which is an important aspect of this project.
However, ultimately Rust's memory-safety won me over, and I'm content with trying to implement Data-Oriented Design as best I can without Zig's native support.