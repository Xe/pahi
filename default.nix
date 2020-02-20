{ sources ? import ./nix/sources.nix }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };
  name = "pahi";
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "target")
    ./.;

  pahi = naersk.buildPackage { inherit name src; };

  olin = naersk.buildPackage {
    name = "olin";
    src = ./wasm;

    buildInputs = [ (import sources.olin { }) ];
    doCheck = false;
  };

  olin-spec = import ./docs/olin-spec { inherit pkgs sources; };
  docs = import ./docs { inherit pkgs sources; };

  composite = pkgs.stdenv.mkDerivation {
    version = "latest";
    phases = "installPhase";
    inherit name src;

    installPhase = ''
      mkdir -p $out/docs/olin-spec
      cp -rf ${docs}/docs $out
      cp -rf ${olin-spec}/docs $out
      cp -rf ${pahi}/bin $out/bin
      mkdir -p $out/wasm

      for f in ${olin}/bin/*
      do
        cp "$f" "$out/wasm/$(basename $f)".wasm
      done

      cp $src/README.md $out/README.md
      cp $src/LICENSE $out/LICENSE
    '';
  };
in composite
