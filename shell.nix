let
  sources = import ./nix/sources.nix;
  niv = import sources.niv { };
  pkgs = import sources.nixpkgs { };
  olin = import sources.olin { inherit pkgs; };
  rust = import ./nix/rust.nix { inherit sources; };
in pkgs.mkShell {
  buildInputs = [ rust.rust niv.niv olin pkgs.wasmer ];
  nativeBuildInputs = [ pkgs.removeReferencesTo ];
}
