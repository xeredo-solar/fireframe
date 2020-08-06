# fireframe

A tiny, dead-simple one-window browser-frame for launching web-applications built on servo

(Replaces webkit2-launcher, the api stays the same)

# dev

Setup:

- Install rustup: `nix-env -iA nixpkgs.rustup`
- Setup nightly & rustc-dev: `bash rustup.sh`
- Run the rustfix script: `bash rustfix.sh` (moves a library so rustc ld can find it)
- Enter a dev-shell: `nix-shell`
- Run `cargo run`
