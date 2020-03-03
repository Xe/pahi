{ sources ? import ./nix/sources.nix }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  dhall = import ./nix/dhall.nix { inherit sources pkgs; };
  olin-cwa = import sources.olin { };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  name = "pahi";
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;

  pahi = naersk.buildPackage {
    inherit name src;
    buildInputs = [ pkgs.openssl pkgs.pkg-config ];
  };

  olin = naersk.buildPackage {
    name = "olin";
    src = ./wasm;

    buildInputs = [ olin-cwa ];
    doCheck = false;
  };

  olin-spec = import ./olin-spec { inherit pkgs sources; };
  docs = import ./docs { inherit pkgs sources; };
  pahi-testrunner = import ./tests { inherit pkgs sources; };

  composite = pkgs.stdenv.mkDerivation {
    version = "latest";
    phases = "installPhase";
    inherit name src;

    installPhase = ''
      mkdir -p $out/docs/olin-spec
      cp -rf ${docs}/docs $out
      cp -rf ${olin-spec}/docs $out
      mkdir -p $out/bin
      cp -rf ${olin-cwa}/bin/cwa $out/bin
      cp -rf ${pahi}/bin/pahi $out/bin
      mkdir -p $out/wasm

      for f in ${olin}/bin/*
      do
        cp "$f" "$out/wasm/$(basename $f)".wasm
      done

      cp -rf ${olin-cwa}/wasm/zig $out/wasm/zig

      cp -rf ${pahi-testrunner}/bin/tests $out/bin/testrunner
      mkdir -p $out/tests
      cp -rf ${pahi-testrunner}/tests/testdata.dhall $out/tests/testdata.dhall
      cp -rf ${pahi-testrunner}/tests/bench.sh $out/tests/bench.sh

      cp $src/README.md $out/README.md
      cp $src/LICENSE $out/LICENSE
    '';
  };
in composite
