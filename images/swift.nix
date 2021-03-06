let
  nixpkgs =
    # Swift from hash in common nixpkgs is marked as broken
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-unstable";
      rev = "24eb3f87fc610f18de7076aee7c5a84ac5591e3e";
    };

  pkgs =
    import nixpkgs {};

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/swift";
  tag = "latest";
  installedPackages = [
    pkgs.binutils
    pkgs.swift
  ];
}
