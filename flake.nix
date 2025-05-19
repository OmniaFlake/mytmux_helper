{
  description = "My Rust package flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }: 
    flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs{inherit system;};
  in
  {
    packages.myRustPackage = pkgs.myRustPackage;

    overlays.default  = final: prev:{
      myRustPackages = prev.callPackage ./default.nix {};
      };
  });
}

