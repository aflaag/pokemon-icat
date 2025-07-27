{ pkgs ? import <nixpkgs> {} }:
let
    pythonWithDeps = pkgs.python3.withPackages (ps: with ps; [
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
    pokemon-icons = pkgs.stdenv.mkDerivation {
        pname = "pokemon-icons";
        version = "1.2.0";

        outputHashMode = "recursive";
        outputHashAlgo = "sha256";
        # outputHash = "sha256-Py3Sfci1625ZYVRoWVyaOQtP9qXpNl6afOqwRkFXH7E=";
        outputHash = "sha256-E8ev6zMB7Dbsw9YksPisSDwV3gQTckemYHG3lj4eYyU=";

        src = ./.;

        buildInputs = [
            pythonWithDeps
            pkgs.cacert
        ];

        buildPhase = ''
            export POKEMON_ICAT_DATA=$TMPDIR
            mkdir -p $POKEMON_ICAT_DATA
            python3 setup_icons.py
        '';

        installPhase = ''
            mkdir -p $out
            cp -r $POKEMON_ICAT_DATA/pokemon-icons $out
        '';

    };
    # pokemon-icat = pkgs.rustPlatform.buildRustPackage rec {
    pokemon-icat = pkgs.stdenv.mkDerivation {
        pname = "pokemon-icat";
        version = "1.2.0";

        src = ./.;
        # src = pkgs.lib.cleanSourceWith {
        #     src = ./.;
        #     filter = path: type:
        #         !(pkgs.lib.hasInfix "/.git/" path);
        # };

        cargoVendorDir = ./vendor;
        # cargoVendorDir = pkgs.lib.cleanSource ./vendor;;
        # cargoSha256 = pkgs.lib.fakeSha256;

        nativeBuildInputs = [
            pkgs.cargo
            pkgs.rustc
            pkgs.pkg-config
            pkgs.makeWrapper
        ];

        buildPhase = ''
            cargo build --release
        '';

        installPhase = ''
            mkdir -p $out/bin
            cp target/release/pokemon-icat $out/bin

            mkdir -p $out/share/pokemon-icat
            cp -r bin/* $out/share/pokemon-icat

            cp -r ${pokemon-icons}/pokemon-icons $out/share/pokemon-icat

            wrapProgram $out/bin/pokemon-icat --set POKEMON_ICAT_DATA $out/share/pokemon-icat
        '';
    };
in
{
    inherit pokemon-icons pokemon-icat;

    # defaultPackages = pkgs.linkFarm "pokemon-full" [
    #     { name = "pokemon-icat"; path = pokemon-icat; }
    #     { name = "pokemon-icons"; path = pokemon-icons; }
    # ];
    defaultPackages = pokemon-icat;
}
