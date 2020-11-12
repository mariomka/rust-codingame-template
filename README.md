Project for creating a rust codingame base with the cargo-generate crate. https://github.com/ashleygwilliams/cargo-generate

## Getting started

### Prerequirements

```
cargo install bundler
```

### VS-code

if running in vscode just run default build task and a single file will be generated into src/bin/singlefile.rs

```
ctrl+shirt+b
```

This will run tests, build bin {{crate_name}}, and generate the singlefile.rs

### Manually

```
cargo test && cargo build --bin joel-test && bundle . > src/bin/singlefile.rs
```
