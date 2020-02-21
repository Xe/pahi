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

      contents = [ pkg pkgs.bash pkgs.coreutils dhall.dhall-json-simple ];

      config = {
        Cmd = [ "/bin/bash" ];
        WorkingDir = "/";
      };
    };

in dockerImage pahi
