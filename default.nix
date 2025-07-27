# { pkgs ? import <nixpkgs> {}, src ? ./. }:
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
    # pokemon-icons = pkgs.rustPlatform.buildRustPackage {
        pname = "pokemon-icons";
        version = "1.2.0";

        outputHashMode = "recursive";
        outputHashAlgo = "sha256";
        # outputHash = "sha256-RKve62/khQMo71pYzefiEhi2vIde/r3bNslLhs/00rk=";
        outputHash = "sha256-tWMzXjdpXBTDI6Rygoaac5eYxJPc9xrkD7hRnVkzJz0=";
        # outputHash = "sha256-1TnfKN8Ij+pbK6vLXdvbV1qud2HfDHeIJQTzTK+jJP0=";
        # outputHash = "sha256-hOWfpeQz0or/2G9VzYnuc6AFHvlsS5NjmQmMOC01jFM=";

        src = ./.;
        # src = src;

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

    # pokemon-icat = pkgs.stdenv.mkDerivation {
    pokemon-icat = pkgs.rustPlatform.buildRustPackage {
        pname = "pokemon-icat";
        version = "1.2.0";

        src = ./.;
        # src = src;

        # cargoVendorDir = ./vendor;
        # cargoSha256 = pkgs.lib.fakeSha256;
        cargoLock = {
            lockFile = ./Cargo.lock;
        };
        # doNotUseCargoVendor = true;

        nativeBuildInputs = [
            # pkgs.cargo
            # pkgs.rustc
            # pkgs.pkg-config
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

    defaultPackages = pokemon-icat;
}
