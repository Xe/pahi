{ sources ? import ../nix/sources.nix, pkgs ? import sources.nixpkgs { }}:
with pkgs;
stdenv.mkDerivation {
  name = "pahi-docs";
  version = "latest";
  phases = "installPhase";
  src = ./.;

  installPhase = ''
    mkdir -p $out/docs
    cp -rf $src/README.md $out/docs
    cp -rf $src/CODE_OF_CONDUCT.md $out/docs
  '';
}
