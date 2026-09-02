
## Create new topic-dir 
`cargo new <exer/e001_amstrong> --lib --vcs none` to create a new topic directory with a library crate and no version control system.

**BUT** for solutions use `--name` to alter the package name to avoid conflicts with the exercise crate name. For example, `cargo new --lib --vcs none --name <NAME>_sol` to create a solution crate for the `<NAME>` exercise.

## Test names
use `test_exercise_xx` double digits e.g. `test_exercise_01`, `test_exercise_02`, etc. to avoid running `test_exercise_10` first.

## Disable Copilot auto-completions
In `.vscode/settings.json` add the following line to disable auto-completions from GitHub Copilot:
```json
{
  "github.copilot.enable": {
    "*": false,
    "plaintext": false,
    "markdown": false,
    "scminput": false
  }
}
```

## Solution to problems with `rust-analyzer`

`cargo init` does not have a built-in flag (like `--workspace`) to initialize a Cargo workspace directly. It only initializes individual packages (`--bin` or `--lib`).

To create a workspace manually, use one of the standard approaches below.

---

### Method 1: Virtual Workspace (Recommended)

A virtual workspace has a root `Cargo.toml` that acts solely as a container without compiling its own code.

1. Create a root `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = [
    "crates/*",
]

```

2. Initialize individual crates inside the member directory:

```bash
cargo new crates/my_app --bin
cargo new crates/my_lib --lib

```

---

### Method 2: Root Package Workspace

If you want the root directory to be a runnable crate that also manages child crates:

1. Initialize the root package:

```bash
cargo init --bin

```

2. Open the generated `Cargo.toml` and append the `[workspace]` table:

```toml
[package]
name = "my_root_app"
version = "0.1.0"
edition = "2024"

[dependencies]

[workspace]
members = [
    "crates/*",
]

```

3. Create child crates:

```bash
cargo new crates/helper_lib --lib

```

---

### Method 3: Third-Party CLI Tool

If you prefer a CLI command, you can install the community extension `cargo-workspaces`:

```bash
cargo install cargo-workspaces
cargo ws init

```