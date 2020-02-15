{ sources ? import ./sources.nix }:

let
  pkgs = import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  date = "2020-02-14";
in
  pkgs.rustChannelOf { inherit date; channel = "nightly"; }
