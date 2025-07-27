# { pkgs ? import <nixpkgs> {}, src ? ./. }:
{ pkgs ? import <nixpkgs> {} }:
let
    python = pkgs.python;
    py = python.pkgs;
    pokemon-icons = py.buildPythonApplication {
        pname = "pokemon-icons";
        version = "1.2.0";
        format = "other"; # no setup.py/pyproject.toml

        src = ./.;

        propagatedBuildInputs = with py; [
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
        ];

        nativeBuildInputs = [
          pkgs.cacert
        ];

        buildPhase = ''
          export POKEMON_ICAT_DATA=$TMPDIR
          mkdir -p "$POKEMON_ICAT_DATA"
          ${python.interpreter} setup_icons.py
        '';
        src = ./.;
        # src = src;

        buildInputs = [
            # pythonWithDeps
            pkgs.cacert
        ];

        # buildPhase = ''
        #     export POKEMON_ICAT_DATA=$TMPDIR
        #     mkdir -p $POKEMON_ICAT_DATA
        #     python3 setup_icons.py
        # '';

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
