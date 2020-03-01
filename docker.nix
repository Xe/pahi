{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  dhall = import ./nix/dhall.nix;
  callPackage = pkgs.lib.callPackageWith pkgs;
  pahi = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildLayeredImage {
      name = "xena/pahi";
      tag = "latest";

      contents = [
        pkg
        pkgs.bash
        pkgs.cacert
        pkgs.coreutils
        pkgs.hyperfine
        dhall.dhall-json-simple
      ];

      config = {
        Cmd = [ "/bin/bash" ];
        WorkingDir = "/";
        Env = [ "NIX_SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt" ];
      };
    };

in dockerImage pahi
