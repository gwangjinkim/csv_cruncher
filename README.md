# CSV Cruncher

A fast CSV processor with Python bindings. Beats pandas at its own game.

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
