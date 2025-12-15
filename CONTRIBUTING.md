# Contributing to eq2c-rs

Thank you for your interest in contributing to eq2c-rs! 🦀

We welcome contributions from everyone—whether it's fixing a bug, adding support for a new mathematical function, improving documentation, or spotting typos.

## 🛠️ Getting Started

1. Fork the repository on GitHub.

2. Clone your fork locally:

```bash
git clone [https://github.com/YOUR_USERNAME/eq2c-rs.git](https://github.com/YOUR_USERNAME/eq2c-rs.git)
cd eq2c-rs
```

3. Create a branch for your specific change:

```bash
git checkout -b feat/add-trig-functions
```

## 🧪 Development Workflow

Since this is a Rust project, we rely on standard Cargo tooling.

### Prerequisites

- Rust & Cargo: Ensure you have the latest stable version installed via rustup.

### Running Tests

Before submitting any changes, please ensure the tests pass. Since this is a transpiler, accuracy is critical.

```bash
cargo test
```

## Code Style & Linting

We use rustfmt for code formatting and clippy for linting. Please run these before committing:

### Format code

```bash
cargo fmt
```

Check for common mistakes

```bash
cargo clippy -- -D warnings
```

### Max Line Length

| Type                 | Max line length |
| :------------------- | --------------: |
| Code                 |             100 |
| Comments in the code |             120 |
| Documentation        |             120 |

100 is the [`max_width`](https://github.com/rust-lang/rustfmt/blob/master/Configurations.md#max_width)
default value.

120 is because of the GitHub. The editor & viewer width there is +- 123 characters.

## ⚠️ Warnings & Code Quality

### Warnings

The code must be warning free. It's quite hard to find an error if the build logs are polluted with warnings.
If you decide to silent a warning with (`#[allow(...)]`), please add a comment why it's required.

<!-- Always consult the [Travis CI](https://travis-ci.org/crossterm-rs/crossterm/pull_requests) build logs. -->

### Forbidden Warnings

Search for `#![deny(...)]` in the code:

- `unused_must_use`
- `unused_imports`

## 📝 Commit Messages (Important!)

To automatically generate our changelogs, we follow the Conventional Commits specification. Please format your commit messages like this:

```
type(scope): description
```

### Types

- feat: A new feature (e.g., adding sin() or cos() support).

- fix: A bug fix (e.g., fixing operator precedence).

- docs: Documentation only changes.

- style: Formatting, missing semi-colons, etc.

- refactor: A code change that neither fixes a bug nor adds a feature.

- perf: A code change that improves performance.

- test: Adding missing tests or correcting existing tests.

- chore: Changes to the build process or auxiliary tools.

### Examples

- `feat(parser): support logarithmic functions`

- `fix(codegen): add missing semicolon in C output`

- `docs: update README with installation steps`

## 📮 Submitting a Pull Request

1. Push your branch to your fork:

```git
git push origin your-branch-name
```

2. Open a Pull Request against the main branch of eq2c-rs.

3. Describe your changes clearly. If your PR fixes an issue, link it (e.g., "Closes #12").

4. Wait for a review! We will do our best to review your code quickly.

## ⚖️ License

By contributing to eq2c-rs, you agree that your contributions will be licensed under the project's LICENSE (MIT or Apache-2.0).
