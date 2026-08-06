{
  description = "A flake for packaging the dailies-r Rust project";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    flake-utils,
    naersk,
    nixpkgs,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};
      in {
        # `nix build` & `nix run`:
        packages.default = naersk'.buildPackage {
          nativeBuildInputs = [
            pkgs.dbus.dev
            pkgs.pkg-config
          ];
          pname = "dailies-r";
          src = ./.;
        };
      }
    );
}
