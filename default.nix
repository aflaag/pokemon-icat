{ pkgs ? import <nixpkgs> {} }:
let
    python = pkgs.python3;
    pythonWithDeps = python.withPackages (ps: with ps; [
        aiohappyeyeballs
        aiohttp
        aiosignal
        "async-timeout"
        attrs
        frozenlist
        idna
        "markdown-it-py"
        mdurl
        multidict
        pillow
        propcache
        pygments
        "python-slugify"
        rich
        "text-unidecode"
        "typing-extensions"
        yarl
    ]);
in
pkgs.stdenv.mkDerivation {
    pname = "pokemon-icat"; # diff with name?
    version = "unstable";

    src = ./.;

    cargoVendorDir = ./vendor;

    nativeBuildInputs = [
        pkgs.cargo
        pkgs.rustc
        pkgs.pkg-config
        pkgs.makeWrapper # what is this?
    ];

    buildInputs = [
        pythonWithDeps
    ];

    buildPhase = ''
        cargo build --release
    '';

    # USE /usr/share/ INSTEAD OF ~/.cache/ !!
    # CHANGE THE BINARY TO LOOK FOR POKEMON_ICAT_DATA INSTEAD OF THE PATH DIRECTLY
    installPhase = ''
        mkdir -p $out/bin
        cp target/release/pokemon-icat $out/bin

        mkdir -p $out/share/pokemon-icat
        cp -r bin/* $out/share/pokemon-icat

        wrapProgram $out/bin/pokemon-icat --set POKEMON_ICAT_DATA $out/share/pokemon-icat --prefix PATH : ${pythonWithDeps}/bin
    '';
}
