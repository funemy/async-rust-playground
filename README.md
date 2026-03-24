# Async Rust Playground

To run each file under `src/bin`, pass their filenames to `--bin`, e.g.,

```bash
cargo run --bin case0
```

The list of executables are `case0`, `case1`, `case1_stm`, `timer`.

The shell scripts under `scripts` can be used for generating HIR/MIR/CFGs for executables.

For example, to generate the MIR for `src/bin/case1.rs`, you can:

```bash
./mir.sh case1
```
