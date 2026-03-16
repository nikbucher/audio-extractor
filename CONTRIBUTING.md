# Contributing

Contributions are welcome! Please open an issue or pull request on [GitHub](https://github.com/nikbucher/audio-snip).

## Development Methodology

[AIUP](https://aiup.dev) — requirements-driven, iterative. Specs, code, and tests improve together in short cycles.

Phases: Inception → Elaboration → Construction → Transition.

## Code Style

- Follow [`.editorconfig`](.editorconfig) / [rustfmt](https://github.com/rust-lang/rustfmt) (`cargo fmt`)
- Run `cargo fmt --check` before committing
- Run `cargo clippy -- -D warnings` for lint checks

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/).

Format: `<type>[optional scope]: <description>`

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `build`

Examples:

- `feat: add draggable range handles for time selection`
- `fix: handle missing FFmpeg gracefully`
- `docs: update vision with success criteria`
- `ci: add commitlint to PR checks`

## Test–Use Case Traceability

Each test is linked to its use case via a naming convention and a doc comment:

```rust
/// UC-002 | BR-008: Date Input Normalization
#[test]
fn uc002_normalize_plain_date_from() { ... }
```

- **Test name:** prefix with `uc{nr}_` (e.g. `uc001_`, `uc002_`) for filtering with `cargo test uc001`
- **Doc comment, line 1:** `UC-{nr} | {Scenario or Business Rule}` — references the scenario (Main Success, A1, A2, …) or business rule (BR-001, BR-002, …) from `docs/use_cases/`
- **Doc comment, line 2 (optional):** `Business Rules: BR-001, BR-002` — only when additional business rules apply beyond what line 1 already states

Omit the doc comment only if the use case mapping is already fully expressed by the test name.

## Releasing

1. Update version in `src-tauri/Cargo.toml` (single source of truth) and `CHANGELOG.md`
2. Commit: `chore: release vX.Y.Z`
3. Tag and push:
   ```sh
   git tag -a vX.Y.Z -m "vX.Y.Z"
   git push origin main vX.Y.Z
   ```

## CI Checks

Make sure CI passes before submitting:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```
