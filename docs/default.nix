{ pkgs }:
let
  src = builtins.filterSource
    (path: type: type != "directory" || builtins.baseNameOf path != "olin-spec")
    ./.;
in
with pkgs;
stdenv.mkDerivation {
  name = "pahi-docs";
  version = "latest";
  phases = "installPhase";
  inherit src;

  installPhase = ''
    mkdir -p $out/docs
    cp -rf $src/* $out/docs
    rm $out/docs/default.nix
  '';
}
