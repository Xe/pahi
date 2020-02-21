{ sources ? import ../nix/sources.nix, pkgs ? import sources.nixpkgs { } }:
with pkgs;
buildGoPackage {
  name = "pahi-tests";
  version = "latest";
  src = ./.;
  goPackagePath = "github.com/Xe/pahi/tests";

  preBuild = ''
    export CGO_ENABLED=0
  '';

  buildFlags = "-tags release";

  postInstall = ''
    mkdir -p $bin/tests
    cp -rf $src/testdata.dhall $bin/tests/testdata.dhall
  '';
}
