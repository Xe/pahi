{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = null;
  targets = [ "wasm32-unknown-unknown" "wasm32-wasi" ];
in pkgs.rustChannelOfTargets channel date targets
