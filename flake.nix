{
  description = "A Nix-flake-based rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in
      pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          nix
          pkg-config
          openssl.dev
        ];

        shellHook = ''
          cargo -V
          exec zsh
        '';
      };
  };
}
