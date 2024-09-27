# Development Instructions

Ideally, `pre-commit` should be installed along with:
- `ruff` ([installation instructions](https://docs.astral.sh/ruff/installation/))
- `mypy` ([installation instructions](https://mypy.readthedocs.io/en/stable/getting_started.html))
- `poetry` ([installation instructions](https://python-poetry.org/docs/))
- `pipx' ([installation instructions](https://pipx.pypa.io/stable/installation/))
- `cargo-edit` (`cargo install cargo-edit`)

#### Install all (main+dev) dependencies:
```shell
poetry install --no-root
```

#### Mypy:
```shell
poetry run mypy .
```

#### Install pre-commit hooks:
```shell
poetry run pre-commit install
```

#### Run pre-commit hooks manually:
```shell
poetry run pre-commit run --all-files
```

#### Update pre-commit hooks:
```shell
poetry run pre-commit autoupdate
```

#### Extra checks
```shell
ruff check --fix --extend-select="F,I,UP" --line-length=120
```

#### Generating a requirements.txt file
```shell
poetry self add poetry-plugin-export
poetry export -f requirements.txt --output requirements.txt
```
