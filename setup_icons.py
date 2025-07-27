import argparse
import asyncio
import re
import os
from dataclasses import dataclass
from io import BytesIO
from pathlib import Path
from typing import Dict, List, Literal, Optional

from aiohttp import ClientSession
from PIL import Image
from rich.progress import Progress, TaskID

from bin.converter import POKEMONS_METADATA

# consts
BATCH_SIZE = 50


@dataclass
class PokemonPath:
    path: str
    folder: Path


# urls
HEADERS = {"Accept": "application/vnd.github+json", "X-GitHub-Api-Version": "2022-11-28"}
TREE_ROOT_URL = "https://api.github.com/repos/PokeAPI/sprites/git/trees/6127a37944160e603c1a707ac0c5f8e367b4050a"

URL_TREE = "https://api.github.com/repos/PokeAPI/sprites/git/trees/c87f4ced89853ad94e3a474306c07d329a28d59c"
URL_POINT_BASE = "https://raw.githubusercontent.com/PokeAPI/sprites/master"

# dirs
# CACHE_DIR = Path.home() / ".cache"
# CACHE_DIR = Path("$POKEMON_ICAT_DATA")
# POKEMON_ICAT_DIR = CACHE_DIR / "pokemon-icat"
# POKEMON_ICAT_DIR = Path("$POKEMON_ICAT_DATA")
POKEMON_ICAT_DIR = Path(os.environ["POKEMON_ICAT_DATA"])
POKEMON_ICAT_DIR.mkdir(exist_ok=True, parents=True)

POKEMON_ICONS_DIR = POKEMON_ICAT_DIR / "pokemon-icons"
POKEMON_ICONS_DIR.mkdir(exist_ok=True, parents=True)


POKEMON_PATHS = [
    PokemonPath(path="/sprites/pokemon", folder=POKEMON_ICONS_DIR / "normal"),
    PokemonPath(path="/sprites/pokemon/shiny", folder=POKEMON_ICONS_DIR / "shiny"),
]

for pokemon_path in POKEMON_PATHS:
    pokemon_path.folder.mkdir(exist_ok=True)


@dataclass
class GitObject:
    path: Path
    mode: str
    type: Literal["blob"]
    sha: str
    size: int
    url: str


# 3D sprites to be ignored (yes, i found them manually)
RANGE_3D = {
    "10093.png",
    "10094.png",
    "10095.png",
    "10096.png",
    "10097.png",
    "10098.png",
    "10099.png",
    "10121.png",
    "10122.png",
    "10130.png",
    "10131.png",
    "10132.png",
    "10133.png",
    "10134.png",
    "10135.png",
    "10144.png",
    "10145.png",
    "10148.png",
    "10149.png",
    "10150.png",
    "10151.png",
    "414-plant.png",
    "664-icy-snow.png",
    "665-icy-snow.png",
    "666-archipelago.png",
    "666-continental.png",
    "666-elegant.png",
    "666-fancy.png",
    "666-garden.png",
    "666-high-plains.png",
    "666-icy-snow.png",
    "666-jungle.png",
    "666-marine.png",
    "666-meadow.png",
    "666-modern.png",
    "666-monsoon.png",
    "666-ocean.png",
    "666-poke-ball.png",
    "666-polar.png",
    "666-river.png",
    "666-sandstorm.png",
    "666-savanna.png",
    "666-sun.png",
    "666-tundra.png",
    "669-blue.png",
    "669-orange.png",
    "669-white.png",
    "669-yellow.png",
    "670-blue.png",
    "670-orange.png",
    "670-white.png",
    "670-yellow.png",
    "671-blue.png",
    "671-orange.png",
    "671-white.png",
    "671-yellow.png",
    "676-dandy.png",
    "676-debutante.png",
    "676-diamond.png",
    "676-heart.png",
    "676-kabuki.png",
    "676-la-reine.png",
    "676-matron.png",
    "676-pharaoh.png",
    "676-star.png",
    "710-large.png",
    "710-small.png",
    "710-super.png",
    "716-neutral.png",
    "720-unbound.png",
    "741-baile.png",
    "745-midday.png",
    "746-solo.png",
    "773-bug.png",
    "773-dark.png",
    "773-dragon.png",
    "773-electric.png",
    "773-fairy.png",
    "773-fighting.png",
    "773-fire.png",
    "773-flying.png",
    "773-ghost.png",
    "773-grass.png",
    "773-ground.png",
    "773-ice.png",
    "773-normal.png",
    "773-poison.png",
    "773-psychic.png",
    "773-rock.png",
    "773-steel.png",
    "773-water.png",
    "774-red-meteor.png",
    "775-form-1.png",
    "778-disguised.png",
    "784-totem.png",
}

ignored: List[GitObject] = []

upscale_factor: float = 3


def build_new_image(png_idx_img: Image.Image) -> Image.Image:
    # convert the image into RGBA format
    rgba_img = png_idx_img.convert("RGBA")

    # Get the bounding box of non-transparent pixels
    bbox = rgba_img.getbbox()

    # If there's no bounding box (i.e., image is fully transparent), raise an error
    if bbox is None:
        raise ValueError("Image is fully transparent")

    # Crop the image to the bounding box
    cropped_img = rgba_img.crop(bbox)

    return cropped_img


async def gather_pokemons(tree: "TreeNode") -> None:
    async with ClientSession() as session:
        with Progress(transient=True) as progress:
            progress_tasks: Dict[str, TaskID] = {}

            @dataclass
            class Entry:
                pokemon_path: PokemonPath
                files: List[GitObject]
                task: TaskID

            pokemons_to_download: Dict[str, Entry] = {}
            for pokemon_path in POKEMON_PATHS:
                folder_node = tree.get_folder_by_path(pokemon_path.path)
                if folder_node is None:
                    print(f"Warning: missing folder for {pokemon_path.path!r}")
                    continue

                files = folder_node.get_filtered_pokemon_files()
                this_task = progress_tasks[folder_node.name] = progress.add_task(
                    f"[red]Downloading {pokemon_path.folder.name} pokemons...", total=len(files)
                )

                pokemons_to_download[folder_node.name] = Entry(
                    files=files,
                    pokemon_path=pokemon_path,
                    task=this_task,
                )

            async def inner(entry: Entry, session: ClientSession):
                while entry.files:
                    batch = entry.files[:BATCH_SIZE]
                    del entry.files[:BATCH_SIZE]

                    results = await asyncio.gather(
                        *[
                            download_single_pokemon(
                                pokemon_obj=pokemon_obj,
                                session=session,
                                url=URL_POINT_BASE + entry.pokemon_path.path + "/" + f"{pokemon_obj.path}",
                                pokemon_path=entry.pokemon_path,
                            )
                            for pokemon_obj in batch
                        ],
                        return_exceptions=True,
                    )
                    progress.update(entry.task, advance=len(results))

            await asyncio.gather(*[inner(entry=entry, session=session) for entry in pokemons_to_download.values()])


async def download_single_pokemon(
    url: str,
    session: ClientSession,
    pokemon_path: PokemonPath,
    pokemon_obj: GitObject,
) -> None:
    pokemon_id = get_pokemon_id(pokemon_obj)
    if pokemon_id is None:
        raise ValueError(f"Invalid pokemon passed: {pokemon_obj.path.name}")

    metadata = POKEMONS_METADATA.get(pokemon_id)
    if metadata is None:
        raise ValueError(f"Invalid pokemon id: {pokemon_id}")

    filename = f"{metadata.name}.png"
    destination = pokemon_path.folder / filename

    # DOESN'T WORK: the saved images are cropped
    # if destination.exists():
    #     stat = destination.stat()
    #     print(stat.st_size, pokemon.size)
    #     if stat.st_size == pokemon.size:
    #         # Skipping: already downloaded
    #         return

    async with session.get(url) as response:
        try:
            # load the image in PNG indexed format
            buffer = BytesIO(await response.read())
            buffer.seek(0)
            png_idx_img = Image.open(buffer, formats=["png"])
        except Exception as e:
            # ignore images that can't be loaded
            # TODO: currently, only 10186.png does not work, due to unknown reasons
            print("---")
            print(f"ERROR: An error occurred while trying to dump '{filename}'; it will be ignored:")
            print(f"{type(e).__name__}: {e}")
            print(f"URL: {url}")

            ignored.append(pokemon_obj)
            return

    # return the new image from the new RGBA array
    loop = asyncio.get_event_loop()
    new_rgba_img = await loop.run_in_executor(None, build_new_image, (png_idx_img))

    # upscale the RGBA image with the upscaling factor
    img = new_rgba_img.resize(
        (int(new_rgba_img.width * upscale_factor), int(new_rgba_img.height * upscale_factor)), Image.BOX
    )

    # save the processed RGBA image
    img.save(destination)


def get_pokemon_id(p: GitObject) -> Optional[str]:
    found = re.search(r"([0-9]+)", p.path.stem)
    if found is None:
        return None
    return found.group()


def image_number(p: GitObject) -> int:
    if p.path.stem in ["substitute", "egg", "egg-manaphy"]:
        return -1

    # extract the number of the pokemon
    filtered_name = get_pokemon_id(p)
    if filtered_name is None:
        return -1

    # return image number
    return int(filtered_name)


class TreeNode:
    def __init__(self, name: str, url: str) -> None:
        self.name = name
        self.url = url
        self.files: List[GitObject] = []
        self.directories: Dict[str, TreeNode] = {}

    def print(self, indent: str = "") -> None:
        """
        Print the tree structure of the node and its children.

        Args:
            indent (str): The indentation string for the current level.
        """
        # Print the current node
        print(f"{indent}- {self.name}/")

        # Print files
        for file in self.files:
            print(f"{indent}  - {file.path.name}")

        # Print directories recursively
        for dir_name, dir_node in self.directories.items():
            dir_node.print(indent + "  ")

    def get_filtered_pokemon_files(self) -> List[GitObject]:
        """
        Get a filtered and sorted list of Pokemon GitObjects.

        Returns:
            List[GitObject]: Filtered and sorted list of Pokemon GitObjects.
        """
        filtered_pokemon_objs: List[GitObject] = []

        for pokemon_obj in self.files:
            png_name = pokemon_obj.path.name
            if pokemon_obj.path.suffix != ".png":
                continue
            if "-mega" in png_name or "-primal" in png_name:
                continue
            if png_name in RANGE_3D:  # Assuming RANGE_3D is defined elsewhere
                continue
            filtered_pokemon_objs.append(pokemon_obj)

        # Sort the filtered list
        filtered_pokemon_objs.sort(key=lambda p: (image_number(p), len(p.path.stem), p.path.stem))

        return filtered_pokemon_objs

    def get_folder_by_path(self, path: str) -> Optional["TreeNode"]:
        """
        Retrieve a folder (TreeNode) by its path in the tree structure.

        Args:
            path (str): The path to the folder, using '/' as separator.

        Returns:
            Optional[TreeNode]: The TreeNode if found, None otherwise.
        """
        parts = list(filter(bool, path.split("/")))

        if len(parts) == 0:
            # We've reached the end of the path, return the current node
            return self

        next_dir = parts[0]
        if next_dir in self.directories:
            # Recursively search in the next subdirectory
            return self.directories[next_dir].get_folder_by_path("/".join(parts[1:]))

        # If we can't find the next directory, return None
        return None


async def build_tree(node: TreeNode, paths: List[str], session: ClientSession) -> None:
    if not paths:
        return

    current = paths.pop(0)

    if current not in node.directories:
        async with session.get(node.url) as response:
            data = await response.json()
            if "message" in data:
                print("ERROR:", data["message"])
                print("ABORTING...")
                exit(1)
                return

            for git_obj in data["tree"]:
                if git_obj["type"] == "blob":
                    node.files.append(
                        GitObject(
                            path=Path(git_obj["path"]),
                            mode=git_obj["mode"],
                            sha=git_obj["sha"],
                            type=git_obj["type"],
                            size=git_obj["size"],
                            url=git_obj["url"],
                        )
                    )
                elif git_obj["type"] == "tree":
                    node.directories[git_obj["path"]] = TreeNode(
                        name=git_obj["path"],
                        url=git_obj["url"],
                    )

    if current in node.directories:
        await build_tree(node=node.directories[current], paths=paths, session=session)
    else:
        if current != "":
            print(f"Warning: Directory '{current}' not found in {node.name}")


async def build_tree_root() -> TreeNode:
    node = TreeNode("root", url=TREE_ROOT_URL)

    async with ClientSession() as session:
        for pokemon_path in POKEMON_PATHS:
            path = pokemon_path.path
            print(f"Scanning {path!r}...")
            split = list(filter(bool, path.split("/"))) + [""]
            await build_tree(node=node, paths=split, session=session)

    return node


async def main() -> None:
    parser = argparse.ArgumentParser(description="Image processing tool")
    parser.add_argument(
        "-u", "--upscale", type=float, metavar="FACTOR", default=3, help="Upscale the image by the specified factor"
    )

    # Add more arguments here as needed

    args = parser.parse_args()

    global upscale_factor
    upscale_factor = args.upscale

    tree = await build_tree_root()
    # tree.print()

    await gather_pokemons(tree)

    if len(ignored) != 0:
        print("These images were ignored due tue errors: " + ", ".join(repr(p.path.name) for p in ignored))


if __name__ == "__main__":
    asyncio.run(main())
