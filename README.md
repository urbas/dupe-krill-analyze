# dupe-krill-analyze [![builder](https://github.com/urbas/dupe-krill-analyze/actions/workflows/build.yml/badge.svg)](https://github.com/urbas/dupe-krill-analyze/actions/workflows/build.yml)

Helps you analyze [dupe-krill](https://github.com/kornelski/dupe-krill)'s
reports.

## Installation

Use nix (see [nix installation instructions](https://nixos.org/download/)):

```bash
# Run directly:
nix run github:urbas/dupe-krill-analyze# -- help

# Or add it to your PATH:
nix shell github:urbas/dupe-krill-analyze#
dupe-krill-analyze related-dirs help
```

## Usage

```bash
# Set the dupe-krill report you'd like to analyze:
export DUPE_KRILL_ANALYZER_INPUT_REPORT=<dupe-krill report path>

# Add

# Now run a bunch of analyses, for example:
dupe-krill-analyze related-dirs <some dir>
```
