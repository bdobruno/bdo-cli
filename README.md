# bdo-cli <img src="./assets/bdo-cli.svg" alt="Logo" width="20" />

`bdo-cli` is a small Rust CLI for common Git, GitHub, and repository workflow commands.

## Installation

Install the CLI with Cargo:

```bash
cargo install --git https://github.com/hellobdo/bdo-cli --tag v1.2.0
```

This installs the `bdo` binary into Cargo's bin directory, usually `~/.cargo/bin`.
Make sure that directory is available in your `PATH`.

## Requirements

`bdo-cli` shells out to existing developer tools. Before using it, install:

- **Rust and Cargo**: for installing and updating the CLI
- **Git**: for repository operations
- **GitHub CLI**: for creating GitHub repositories

You should also be authenticated with the GitHub CLI before using `bdo create`:

```bash
gh auth login
```

## Available Commands

- `bdo help` - Show all available commands
- `bdo create <repo-name>` - Create a local repository, make the initial commit, and create the GitHub repository
- `bdo push` - Add, commit, and push changes
- `bdo branch <branch-name>` - Create and push a new branch
- `bdo checkout` - Switch back to `main`, delete the previous branch, and reset `main` to `origin/main`

## Development

Run the CLI locally with Cargo:

```bash
cargo run -- help
```

Run the Rust checks used by CI:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release --locked
```

## Uninstallation

If you installed with Cargo, uninstall with:

```bash
cargo uninstall bdo-cli
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
