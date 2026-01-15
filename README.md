# Backend Setup

The backend for Mosifra can be run either using a **Nix Flake** or with a
manually installed Rust toolchain.

## Requirement

You'll need an environment file containing the hash for the jwt and the secret
for Rocket.

## Using Nix Flake

The provided flake sets up all the necessary dependencies, including:

- `cargo`
- `rustc`
- `rustfmt`
- `clippy`
- `rust-analyzer`
- `sqls`

The `sqls` dependency is optional and could prove useful only in development.

Once inside the flake development shell, you can run the API with:

```bash
cargo run
```

To use the flake, you need **Nix** installed with the experimental features
`nix-command` and `flakes` enabled.

## Running Locally Without Flake

If you prefer not to use Nix, you will need a working **Rust toolchain**
installed on your system (including `cargo` and `rustc`).

Once Rust is set up, you can start the API by navigating to the backend folder
and running:

```bash
cargo run
```

## Notes

- The flake also sets `RUST_SRC_PATH` for proper Rust tooling integration.
- You can optionally launch `neovide` automatically inside the dev shell, as
  configured in the flake.
