# CSV Cruncher

A fast CSV processor with Python bindings. Beats pandas at its own game.
(Sounds nice but this is actually just a test package to demonstrate how to use PO3
to package Rust function into Python packages which run Rust under the hood).

## Install
```bash
cargo add csv_cruncher
pip install maturin
uv run maturin develop
```

# Usage

## CLI:

```bash
cargo run -- --input data.csv --output results.json
```

## Python:

```python
from csv_cruncher import crunch_csv
print(crunch_csv("data.csv"))
```
