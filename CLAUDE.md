# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Educational side-by-side comparisons of Rust and C++ demonstrating core systems programming concepts. Each topic is implemented as a pair of self-contained binaries — one Rust, one C++ — that print annotated output explaining the concept as they run. No external crates or third-party headers; `std` only in both languages.

## Build Commands

```bash
make all          # build every Rust and C++ binary
make run-rust     # run Rust ownership demo
make run-cpp      # run C++ ownership demo
make run-rust-mem # run Rust stack/heap memory demo
make run-cpp-mem  # run C++ stack/heap memory demo
make clean        # remove all build artifacts
```

Build individual Rust binaries:
```bash
cd rust && cargo build --release --bin ownership_demo
cd rust && cargo build --release --bin memory_demo
```

Build individual C++ binaries (clang++, C++17):
```bash
clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2 -o cpp/ownership_demo cpp/ownership_demo.cpp
clang++ -std=c++17 -Wall -Wextra -Wpedantic -O2 -o cpp/memory_demo   cpp/memory_demo.cpp
```

Lint Rust:
```bash
cd rust && cargo clippy
```

## Architecture

All binaries follow the same structure: a `main()` that calls one function per section, each printing a `=== Section N: Title ===` banner followed by annotated output. Intentional compile errors (use-after-move, conflicting borrows, etc.) are placed inside block comments with `// COMPILE ERROR:` annotations so programs compile cleanly while still teaching the concept.

### Rust project (`rust/`)

Single Cargo package (`ownership_demo`, edition 2021) with two binaries:
- `src/main.rs` — ownership & borrowing demo (`rust-concepts-tutor` agent scope)
- `src/bin/memory_demo.rs` — stack vs heap demo (`cpp-rust-educator` agent scope)

### C++ source (`cpp/`)

Two standalone `.cpp` files compiled directly with `clang++`; no CMake or build system beyond the top-level Makefile:
- `ownership_demo.cpp` — mirrors `src/main.rs`
- `memory_demo.cpp` — mirrors `src/bin/memory_demo.rs`

### Adding a new topic

1. Add a new `[[bin]]` entry to `rust/Cargo.toml` pointing at `src/bin/<topic>.rs`.
2. Create the matching `cpp/<topic>.cpp`.
3. Add `cpp/<topic>` target and `run-rust-<topic>` / `run-cpp-<topic>` targets to the top-level `Makefile`.

## Agents

Two sub-agents in `.claude/agents/` shape how concept explanations are generated:

- **`rust-concepts-tutor`** — explains Rust-only concepts (ownership, lifetimes, traits, enums, macros) with standard-library-only runnable samples. Uses Claude Opus.
- **`cpp-rust-educator`** — produces side-by-side C++/Rust comparisons. C++ samples annotate which standard (C++11/17/23) a feature belongs to. Uses Claude Opus.

Both agents maintain persistent per-agent memory under `.claude/agent-memory/<agent-name>/`.

## Constraints

- Zero warnings under `cargo clippy` and `clang++ -Wall -Wextra -Wpedantic`.
- No `unsafe` Rust unless a demo explicitly requires it (and labels it clearly).
- C++ must not invoke undefined behaviour; dangerous patterns must be labeled.
- Rust binaries: use `_`-prefixed names for intentionally unused variables to suppress warnings without `#[allow(...)]`.
