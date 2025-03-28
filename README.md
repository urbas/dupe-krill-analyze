# dupe-krill-analyze [![builder](https://github.com/urbas/dupe-krill-analyze/actions/workflows/build.yml/badge.svg)](https://github.com/urbas/dupe-krill-analyze/actions/workflows/build.yml)

Helps you analyze [dupe-krill](https://github.com/kornelski/dupe-krill)'s
reports.

## Installation

Currently the only supported way to run this tool is via nix (see [nix
installation instructions]):

```bash
# Run directly:
nix run github:urbas/dupe-krill-analyze# -- help

# Or add it to your PATH:
nix shell github:urbas/dupe-krill-analyze#
dupe-krill-analyze related-dirs help
```

## Usage

```bash
dupe-krill-analyze -d <dupe-krill JSON report path> related-dirs <some dir>
```

You can also set the dupe-krill report via this environment variable:

```bash
export DUPE_KRILL_ANALYZER_INPUT_REPORT=<dupe-krill JSON report path>

# Now run a bunch of analyses, for example:
dupe-krill-analyze related-dirs <some dir>
dupe-krill-analyze related-dirs <other dir>
```

## Development

Install nix (see [nix installation instructions]) and run:

```bash
# This command will open a new shell where you can run `cargo`:
nix develop

# Now you're in the nix develop shell and can run:
cargo test
cargo run -- help
cargo clippy -- -D warnings
cargo fmt
```

[nix installation instructions]: https://nixos.org/download/
