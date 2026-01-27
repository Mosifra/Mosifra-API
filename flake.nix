{
  description = "Rust dev flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };
  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer
        sqls
        tmux
      ];
      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      shellHook = ''
        SESSION_NAME="rust-dev"

        if ! tmux has-session -t $SESSION_NAME 2>/dev/null; then
          tmux new-session -d -s $SESSION_NAME -n "editor"
          tmux send-keys -t $SESSION_NAME:0 "neovide" C-m

          tmux new-window -t $SESSION_NAME:1 -n "shell"

          tmux select-window -t $SESSION_NAME:1
        fi

        tmux attach-session -t $SESSION_NAME
      '';
    };
  };
}
