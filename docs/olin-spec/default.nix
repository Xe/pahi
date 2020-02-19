{ sources ? import ../../nix/sources.nix, pkgs ? import sources.nixpkgs { }
, dhall-lang ? sources.dhall-lang, dhall ? import ../../nix/dhall.nix }:
pkgs.stdenv.mkDerivation rec {
  name = "olin-spec";
  version = "latest";
  src = ./.;
  phases = "buildPhase installPhase";

  buildInputs = [ dhall.dhall-simple dhall-lang ];

  buildPhase = ''
    (cd $src/errors && dhall text < renderErrorMD.dhall) > errors.md
  '';

  installPhase = ''
    mkdir -p $out/docs/olin-spec
    cp -rf $src/README.md $out/docs/olin-spec/README.md
    cp -rf errors.md $out/docs/olin-spec/errors.md
  '';
}
