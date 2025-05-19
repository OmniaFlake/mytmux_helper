{
  description = "My Rust package flake with overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlay ];
        };
      in {
        packages.mytmux_helper = pkgs.mytmux_helper;

        overlays = {
          default = self.overlay;
        };

        overlay = self.overlay;

        # Other outputs like devShells, apps, etc.
      });
}

