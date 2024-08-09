{
  description = "Example Rust development environment for Zero to Nix";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    allSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

    forAllSystems = f:
      nixpkgs.lib.genAttrs allSystems
      (system: f {pkgs = import nixpkgs {inherit system;};});
  in {
    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        packages =
          (with pkgs; [rustup])
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin
          (with pkgs; [libiconv]);
        shellHook = "zsh && exit";
	WINIT_UNIX_BACKEND = "wayland";
	LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${ with pkgs; lib.makeLibraryPath [
	  wayland
	  libGL
	  vulkan-loader
	  vulkan-headers
	  libxkbcommon
	  fontconfig
	] }";
      };
    });
  };
}
