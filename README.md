# AoC 2023

Doing AoC in Rust with a custom AoC lib + template!

## Add a new day

1. Run `cargo aoc` to generate new workspace
2. Add new workspace to `Cargo.toml` as both a workspace memeber a dependecy
3. Import and add DayX to `days!(...)` macro in `src/main.rs`
