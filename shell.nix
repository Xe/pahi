let
  sources = import ./nix/sources.nix;
  niv = import sources.niv { };
  pkgs = import sources.nixpkgs { };
  olin = import sources.olin { inherit sources pkgs; };
  rust = import ./nix/rust.nix { inherit sources; };
  dhall = import ./nix/dhall.nix;
in pkgs.mkShell {
  buildInputs =
    [ rust niv.niv olin pkgs.wasmer dhall.dhall-simple sources.dhall-lang ];
  nativeBuildInputs = [ pkgs.removeReferencesTo ];

  # envvars
  RUST_LOG = "info";
}
