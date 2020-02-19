{ sources ? import ./nix/sources.nix }:
let
  rust = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { };
  naersk = pkgs.callPackage sources.naersk {
    rustc = rust;
    cargo = rust;
  };

  pahi = naersk.buildPackage {
    name = "pahi";
    src = builtins.filterSource
      (path: type: type != "directory" || builtins.baseNameOf path != "target")
      ./.;
    remapPathPrefix = true;
  };

  olin = naersk.buildPackage {
    name = "olin";
    src = ./wasm;

    buildInputs = [ (import sources.olin { }) ];
    doCheck = false;
    remapPathPrefix = true;
  };

in { inherit pahi olin; }
