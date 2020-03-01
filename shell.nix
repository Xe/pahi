let
  sources = import ./nix/sources.nix;
  niv = import sources.niv { };
  pkgs = import sources.nixpkgs { };
  olin = import sources.olin { inherit pkgs; };
  rust = import ./nix/rust.nix { inherit sources; };
  dhall = import ./nix/dhall.nix;
in pkgs.mkShell {
  buildInputs = [
    rust

    # benchmarks
    pkgs.hyperfine

    # dependency management
    niv.niv

    # native dependencies
    pkgs.openssl

    # olin-docs
    dhall.dhall-simple
    sources.dhall-lang

    # tests
    pkgs.go
    olin
  ];
  nativeBuildInputs = [ pkgs.removeReferencesTo ];

  # envvars
  RUST_LOG = "pahi=error,pahi_olin=info";
  MAGIC_CONCH = "yes";
}
