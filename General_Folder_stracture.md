<!-- Y:  -->
<!-- my_project/ -->
<!-- │ -->
<!-- │ -->
<!-- │ -->
<!-- │ -->
<!-- ├── src/ -->
<!-- │ ├── main.rs                     # Binary crate entry point -->
<!-- │ ├── lib.rs                      # Library crate entry point (all modules are imoported here and this file will import to main) -->
<!-- │ │ -->
<!-- │ │ -->
<!-- │ │ -->
<!-- │ │ -->
<!-- │ ├── module1.rs                  # A single module -->
<!-- │ │ -->
<!-- │ └── module2/ -->
<!-- │   │-->
<!-- │   ├── mod.rs                    # A FILE THAT COLLECT ALL THE MODULES IN THE MODULE2 FOLDER -->
<!-- │   ├── submod.rs -->
<!-- │   └── utils.rs -->
<!-- │ -->
<!-- │ -->
<!-- │ -->
<!-- │ -->
<!-- │ -->
<!-- ├── tests/                         # Integration tests (compiled separately) -->
<!-- │ ├── test_a.rs -->
<!-- │ └── test_b.rs -->
<!-- │ -->
<!-- │ -->
<!-- ├── benches/                       # Benchmark files (if you use `cargo bench`) -->
<!-- │ -->
<!-- ├── examples/                      # Example usage files (run with `cargo run --example name`) -->
<!-- │ -->
<!-- │ -->
<!-- ├── Cargo.toml                   # Dependencies, package info -->
<!-- └── Cargo.lock                   # Auto-generated lockfile -->
