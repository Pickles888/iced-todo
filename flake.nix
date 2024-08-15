{
  description = "Todo list app made in iced";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

	naersk' = pkgs.callPackage naersk {};
      in {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShell = pkgs.mkShell {
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
	  ]}";
      };
    });
}
