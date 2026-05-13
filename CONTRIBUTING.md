# Contributing

Thanks for your interest in contributing! This project is actively looking for contributors across all skill levels.

## Getting Started

1. Fork the repo and clone it locally.
2. Install prerequisites:
   ```bash
   rustup target add wasm32v1-none
   cargo install --locked stellar-cli
   ```
3. Build:
   ```bash
   stellar contract build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## How to Contribute

- Browse open issues — anything labelled `good first issue` is a great starting point.
- Comment on an issue before starting work to avoid duplication.
- Open a PR against `develop` with a clear description of what you changed and why.

## Issue Labels

| Label | Meaning |
|---|---|
| `good first issue` | Self-contained, well-scoped, beginner-friendly |
| `help wanted` | Needs a contributor, no prior context required |
| `core` | Touches proposal/voting/treasury logic |
| `testing` | Adds or improves test coverage |
| `docs` | Documentation improvements |
| `sdk` | TypeScript SDK work |

## Code Style

- Follow standard Rust formatting (`cargo fmt`).
- Keep functions small and focused.
- Add a doc comment (`///`) to every public function.
- Mark incomplete work with `// TODO:` comments.

## PR Checklist

- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes with no warnings
- [ ] `cargo test` passes
- [ ] New public functions have doc comments
- [ ] ROADMAP.md updated if a roadmap item is completed

## License

By contributing you agree that your contributions will be licensed under MIT.
