{
  description = "My tmux helper";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, ... } @input: let
    system  = "x86_64-linux";
    pkgs = import nixpkgs{inherit system;};
  in
  {
    packages.${system}.default =  pkgs.rustPlatform.buildRustPackage
    {
      name = "mytmux_helper"  ;
      cargoLock.lockFile = ./Cargo.lock;
      src = ./. ;
    };
  };
}
