{
  description = "neucars — macroquad/miniquad dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      runtimeLibs = with pkgs; [
        libx11
        libxi
        libxcursor
        libxrandr
        libxinerama
        libxkbcommon
        libGL
        alsa-lib
      ];
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "boids";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        doCheck = false;

        nativeBuildInputs = with pkgs; [
          binaryen
          lld
        ];

        buildPhase = ''
          runHook preBuild
          cargo build --release --target wasm32-unknown-unknown --offline
          runHook postBuild
        '';

        installPhase = ''
          runHook preInstall
          mkdir -p $out
          cp web/index.html web/mq_js_bundle.js $out/
          wasm-opt -Oz \
            target/wasm32-unknown-unknown/release/boids.wasm \
            -o $out/boids.wasm
          runHook postInstall
        '';
      };

      apps.${system} = {
        serve = {
          type = "app";
          program = pkgs.lib.getExe (
            pkgs.writeShellApplication {
              name = "serve";
              runtimeInputs = with pkgs; [
                cargo
                rustc
                lld
                stdenv.cc
                python3
              ];
              text = ''
                cargo build --release --target wasm32-unknown-unknown
                cp target/wasm32-unknown-unknown/release/boids.wasm web/
                echo "serving on http://localhost:8080 (ctrl-c to stop)"
                python3 -m http.server -d web 8080
              '';
            }
          );
        };

        clean = {
          type = "app";
          program = pkgs.lib.getExe (
            pkgs.writeShellApplication {
              name = "clean";
              runtimeInputs = [ pkgs.cargo ];
              text = ''
                cargo clean
                rm -rf result public web/boids.wasm
                echo "cleaned: target/ result public web/boids.wasm"
              '';
            }
          );
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          pkg-config
          binaryen
          lld
        ];

        buildInputs = runtimeLibs;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
      };
    };
}
