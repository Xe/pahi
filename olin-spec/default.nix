{ sources ? import ../nix/sources.nix, pkgs ? import sources.nixpkgs { }
, dhall-lang ? sources.dhall-lang, dhall ? import ../nix/dhall.nix }:
pkgs.stdenv.mkDerivation rec {
  name = "olin-spec";
  version = "latest";
  src = ./.;
  phases = "buildPhase installPhase";

  buildInputs = [ dhall.dhall-simple dhall-lang ];

  buildPhase = ''
    export DHALL_LOC=${dhall-lang}/Prelude/package.dhall
    echo $DHALL_LOC
    buildDir=$(pwd)
    (cd $src && dhall text < $src/renderTypesToMD.dhall) > types.md
    (cd $src/errors && dhall text < renderErrorMD.dhall) > errors.md
    mkdir ns
    (cd $src/ns && dhall text < env.dhall > $buildDir/ns/env.md)
    (cd $src/ns && dhall text < io.dhall > $buildDir/ns/io.md)
    (cd $src/ns && dhall text < log.dhall > $buildDir/ns/log.md)
    (cd $src/ns && dhall text < random.dhall > $buildDir/ns/random.md)
    (cd $src/ns && dhall text < runtime.dhall > $buildDir/ns/runtime.md)
    (cd $src/ns && dhall text < startup.dhall > $buildDir/ns/startup.md)
    (cd $src/ns && dhall text < time.dhall > $buildDir/ns/time.md)
  '';

  installPhase = ''
    mkdir -p $out/docs/olin-spec
    cp -rf $src/*.md $out/docs/olin-spec/
    cp -rf *.md $out/docs/olin-spec/
    cp -rf ns $out/docs/olin-spec/
  '';
}
