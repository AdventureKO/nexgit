{
  description = "Nexgit development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];

      forAllSystems =
        function:
        nixpkgs.lib.genAttrs systems (
          system:
          function (
            import nixpkgs {
              inherit system;
            }
          )
        );
    in
    {
      formatter = forAllSystems (pkgs: pkgs.nixfmt);

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              cargo
              clippy
              git
              nixd
              nixfmt
              nodejs_22
              pnpm_10
              openssl
              pkg-config
              rust-analyzer
              rustc
              rustfmt
            ]
            ++ lib.optionals stdenv.isLinux [
              alsa-lib
              at-spi2-atk
              cups
              gtk3
              libdrm
              libxkbcommon
              mesa
              nspr
              nss
              xorg.libX11
              xorg.libXScrnSaver
              xorg.libXcomposite
              xorg.libXdamage
              xorg.libXext
              xorg.libXfixes
              xorg.libXrandr
              xorg.libxcb
              xorg.libxshmfence
            ];

          RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
        };
      });
    };
}
