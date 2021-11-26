{
  description = "The next-generation Olin runtime, made with love";

  inputs.dhall-lang.url = "https://github.com/dhall-lang/dhall-lang/archive/ccb9f5d54b0ecba05a6493e84442ce445e411e9e.tar.gz";
  inputs.dhall-lang.flake = false;

  inputs.easy-dhall-nix.url = "https://github.com/justinwoo/easy-dhall-nix/archive/7c22a145fcb8e00b61d29efc7543af0c80d709ed.tar.gz";
  inputs.easy-dhall-nix.flake = false;

  inputs.naersk.url = "github:nix-community/naersk";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  inputs.nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";
  inputs.nixpkgs-mozilla.flake = false;

  inputs.olin.url = "https://github.com/Xe/olin/archive/0e8c8eea725307f8f396919cbf7f965a0d67b0e0.tar.gz";
  inputs.olin.flake = false;

  outputs = { self, dhall-lang, easy-dhall-nix, naersk, nixpkgs, nixpkgs-mozilla, olin }:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
      naersk-lib = naersk.lib."x86_64-linux".override { rustc = rust; };

      mozilla = pkgs.callPackage (nixpkgs-mozilla + "/package-set.nix") {};
      rust = let
          channel = "nightly";
          date = "2020-07-27";
          targets = [ "wasm32-unknown-unknown" "wasm32-wasi" ];
        in (mozilla.rustChannelOf {
          inherit channel date;
          sha256 = "sha256-75eK1CNDEkeVvJ1phWOHZBuujMhalYCxbi5sgAFUlvI=";
        }).rust.override { inherit targets; };

      dhall = import easy-dhall-nix { inherit pkgs; };
      dhall-lang-pkg = import dhall-lang { inherit pkgs; };
      olin-cwa = import olin { inherit pkgs; };

      olin-spec = import ./olin-spec {
        inherit dhall-lang pkgs;
        dhall = import easy-dhall-nix { inherit pkgs; };
      };

      docs = import ./docs { inherit pkgs; };
      pahi-testrunner = import ./tests { inherit pkgs; };

      olin-pkg = naersk-lib.buildPackage {
        name = "olin";
        src = ./wasm;

        buildInputs = [ olin-cwa ];
        doCheck = false;
      };

      pahi = naersk-lib.buildPackage {
        inherit name src;
        buildInputs = [ pkgs.openssl pkgs.pkg-config ];
      };

      name = "pahi";
      src = builtins.filterSource
        (path: type: type != "directory" || builtins.baseNameOf path != "target")
        ./.;

    in {
      defaultPackage.x86_64-linux = pkgs.stdenv.mkDerivation {
        version = "latest";
        phases = "buildPhase installPhase";
        inherit name src;

        installPhase = ''
          mkdir -p $out/docs/olin-spec
          cp -rf ${docs}/docs $out
          cp -rf ${olin-spec}/docs $out
          mkdir -p $out/bin
          cp -rf ${olin-cwa}/bin/cwa $out/bin
          cp -rf ${pahi}/bin/pahi $out/bin
          mkdir -p $out/wasm

          for f in ${olin-pkg}/bin/*
          do
<<<<<<< HEAD
            cp "$f" "$out/wasm/$(basename $f)"
=======
            cp "$f" "$out/wasm/$(basename $f)".wasm
>>>>>>> main
          done

          cp -rf ${olin-cwa}/wasm/zig $out/wasm/zig

          cp -rf ${pahi-testrunner}/bin/tests $out/bin/testrunner
          mkdir -p $out/tests
          cp -rf ${pahi-testrunner}/tests/testdata.dhall $out/tests/testdata.dhall
          cp -rf ${pahi-testrunner}/tests/bench.sh $out/tests/bench.sh

          cp $src/README.md $out/README.md
          cp $src/LICENSE $out/LICENSE
        '';
      };

<<<<<<< HEAD
=======
      devShell.x86_64-linux = pkgs.mkShell {
        buildInputs = [
          rust

          pkgs.hyperfine

          pkgs.openssl
          pkgs.pkg-config

          dhall-lang-pkg
          dhall

          pkgs.go
          olin
        ];
      };

>>>>>>> main
      packages.x86_64-linux.docker = let
        img = pkgs.dockerTools.buildLayeredImage {
          name = "xena/pahi";
          tag = "latest";

          contents = [ self.defaultPackage.x86_64-linux # pahi
                       pkgs.bash pkgs.coreutils pkgs.cacert pkgs.hyperfine
<<<<<<< HEAD
                       dhall.dhall-json-simple dhall-lang ];
=======
                       dhall.dhall-json-simple ];
>>>>>>> main

          config = {
            Cmd = [ "/bin/bash" ];
            WorkingDir = "/";
            Env = [ "NIX_SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt" ];
          };
        };
      in pkgs.stdenv.mkDerivation {
        name = "pahi-docker";
        version = "latest";
        src = ./.; # this is a bit of a lie, but it works

        installPhase = ''
          mkdir -p $out/docker/
          cp ${img} $out/docker/
        '';

        doConfigure = false;
        doBuild = false;
      };
    };
}
