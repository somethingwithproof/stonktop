# Contributing

Bug reports and feature requests go through the
[issue templates](https://github.com/somethingwithproof/stonktop/issues/new/choose).
Report security problems privately as described in [SECURITY.md](SECURITY.md);
do not open a public issue for them.

## Development setup

Stonktop supports Rust 1.89 or later (the `rust-version` in `Cargo.toml`). The
README lists the build, test, lint and format commands.

CI runs the following on every pull request, so run them locally first:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
cargo test --all-targets
```

Formatting follows `rustfmt.toml` and clippy warnings are treated as errors.

## Pull requests

- Keep each pull request to one purpose and fill in the pull request template.
- Sign off every commit with `git commit -s`. The DCO check fails commits
  without a `Signed-off-by` line.
- Use conventional commit subjects such as `fix(app): ...` or `ci: ...`.
- Contributions are licensed under the MIT license in [LICENSE](LICENSE).

## Tests

When you add major functionality or fix a bug, add tests for it in the same
pull request. Unit tests live next to the code in `src/`, and integration and
end-to-end tests live in `tests/`. A change that cannot be tested should say
why in the pull request description.

## Dependencies

Avoid adding a dependency without a clear reason, and explain it in the pull
request. `Cargo.lock` is committed, and CI runs `cargo audit` against it.
