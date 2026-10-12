"""Add localized name columns (name_en, name_fr, name_ja, ...) to bin/pokemon_data.csv.

The `name` column stays the PokeAPI identifier: it is the icon file name, so
the downloaded pictures never need renaming. Localized names come from the
PokeAPI species names table.

Usage: python3 update_names.py
"""

import csv
import io
import urllib.request
from pathlib import Path

POKEAPI_CSV = "https://raw.githubusercontent.com/PokeAPI/pokeapi/master/data/v2/csv/{}.csv"

# column suffix -> PokeAPI local_language_id
LANGUAGES = {
    "en": "9",
    "fr": "5",
    "ja": "1",  # ja-Hrkt (katakana)
}

DATA_CSV = Path(__file__).parent / "bin" / "pokemon_data.csv"


def fetch_csv(table: str) -> list[dict[str, str]]:
    with urllib.request.urlopen(POKEAPI_CSV.format(table)) as response:
        return list(csv.DictReader(io.StringIO(response.read().decode("utf-8"))))


def main() -> None:
    # identifiers in our CSV are either a species ("bulbasaur") or a default form
    # ("nidoran-f", "mr-mime"), so look them up in both tables
    species_ids = {row["identifier"]: row["id"] for row in fetch_csv("pokemon_species")}
    species_ids |= {row["identifier"]: row["species_id"] for row in fetch_csv("pokemon")}

    names: dict[str, dict[str, str]] = {lang: {} for lang in LANGUAGES}
    lang_by_id = {lang_id: lang for lang, lang_id in LANGUAGES.items()}
    for row in fetch_csv("pokemon_species_names"):
        lang = lang_by_id.get(row["local_language_id"])
        if lang is not None:
            names[lang][row["pokemon_species_id"]] = row["name"]

    with DATA_CSV.open(newline="", encoding="utf-8") as f:
        rows = list(csv.DictReader(f))

    for row in rows:
        species_id = species_ids.get(row["name"])
        for lang in LANGUAGES:
            # entries without a species (e.g. "??????????") keep their name as is
            row[f"name_{lang}"] = names[lang].get(species_id, row["name"]) if species_id else row["name"]

    base_fields = ["name", "generation", "height", "typing"]
    fields = base_fields + [f"name_{lang}" for lang in LANGUAGES]
    with DATA_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fields, quoting=csv.QUOTE_NONNUMERIC, lineterminator="\n")
        writer.writeheader()
        for row in rows:
            # numbers unquoted, like in the original file
            writer.writerow({field: int(row[field]) if field == "height" else row[field] for field in fields})

    missing = [row["name"] for row in rows if species_ids.get(row["name"]) is None]
    print(f"updated {len(rows)} pokemons in {DATA_CSV}; without species: {missing}")


if __name__ == "__main__":
    main()
