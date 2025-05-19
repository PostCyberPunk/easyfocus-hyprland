{
  description = "easyfocus-hyprland flake with package and devShell";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {self, ...}: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

    forEachSupportedSystem = f:
      inputs.nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          inherit system;
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
              inputs.self.overlays.default
            ];
          };
        });
    mkPkg = pkgs:
      pkgs.rustPlatform.buildRustPackage {
        pname = "easyfocus-hyprland";
        version = "dev";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
        buildInputs = with pkgs; [
          gtk3
          gtk-layer-shell
        ];
        nativeBuildInputs = with pkgs; [
          cairo
          pkg-config
        ];
      };
  in {
    overlays.default = final: prev: {
      rustToolchain = let
        rust = prev.rust-bin;
      in
        if builtins.pathExists ./rust-toolchain.toml
        then rust.fromRustupToolchainFile ./rust-toolchain.toml
        else if builtins.pathExists ./rust-toolchain
        then rust.fromRustupToolchainFile ./rust-toolchain
        else
          rust.stable.latest.default.override {
            extensions = ["rust-src" "rustfmt"];
          };
    };
    packages = forEachSupportedSystem (args: {
      easyfocus-hyprland = mkPkg args.pkgs;
      default = self.packages.${args.system}.easyfocus-hyprland;
    });

    devShells = forEachSupportedSystem ({pkgs, ...}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer
          #############dev
          cairo
          gtk3
          gtk-layer-shell
        ];

        env = {
          # Required by rust-analyzer
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    });
  };
}
