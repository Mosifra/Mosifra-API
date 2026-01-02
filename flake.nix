{
  description = "Rust dev environment flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells.default = pkgs.mkShell {
      buildInputs = [
        pkgs.rustup
        pkgs.rust-analyzer
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
      ];

      shellHook = ''
        echo "Shell lancé"
        export CARGO_HOME=$PWD/.cargo
        export RUSTUP_HOME=$PWD/.rustup
      '';
    };
  };
}
