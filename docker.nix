{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  callPackage = pkgs.lib.callPackageWith pkgs;
  pahi = callPackage ./default.nix { };

  dockerImage = pkg:
    pkgs.dockerTools.buildLayeredImage {
      name = "xena/pahi";
      tag = "latest";

      contents = [ pkg pkgs.bash pkgs.coreutils ];

      config = {
        Cmd = [ "/bin/bash" ];
        WorkingDir = "/";
      };
    };

in dockerImage pahi
