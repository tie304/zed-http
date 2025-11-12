{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustup

    # Development tools
    git
    curl
    wget
    jq

    # Build dependencies
    pkg-config
    openssl
    openssl.dev

    # System libraries that might be needed
    gcc
    glibc

    # Optional but useful tools
    bacon # Background rust code checker
    cargo-watch # Auto-rebuild on file changes
    cargo-edit # Cargo subcommands for editing Cargo.toml
    cargo-outdated # Check for outdated dependencies

    # For HTTP/networking projects
    netcat
    nmap
    wireshark-cli

    nil
    nixfmt-rfc-style
  ];

  # Environment variables
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  OPENSSL_DIR = "${pkgs.openssl.dev}";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

  shellHook = ''
    echo "🦀 Rust development environment loaded!"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
    echo "Available tools:"
    echo "  - rust-analyzer (LSP server)"
    echo "  - clippy (linter)"
    echo "  - rustfmt (formatter)"
    echo "  - bacon (background checker)"
    echo "  - cargo-watch (file watcher)"
    echo ""
    echo "Run 'cargo --help' to see available commands"
  '';
}
