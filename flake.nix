{
  description = "A Chip-8 emulator";

  inputs = {
    # Requires unstable in order to build as of 2021-11-19
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachSystem [
      "x86_64-darwin"
      "x86_64-linux"
    ]
      (system:
        let
          name = "chipeyte";
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          checks.format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ rustfmt cargo ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            touch $out # success!
            '';

          apps.${name} = {
            type = "app";
            program = "${self.pkgs.${system}.${name}}/bin/${name}";
          };

          packages.${name} = (pkgs.makeRustPlatform {
            inherit (fenix.packages.${system}.minimal) cargo rustc;
          }).buildRustPackage {
            pname = "chipeyte";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildFeatures = [ "sdl2-ui" "logging" ];
            buildInputs = with pkgs; [ SDL2 SDL2_gfx libiconv ];

            cargoSha256 = "sha256-fw/zUbYynrpeLGQ/uhs3LEq7tnECvatNAuDCJuCQGms=";
          };

          defaultPackage = self.packages.${system}.${name};

          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [ cargo rustc rust-analyzer bat libiconv SDL2 SDL2_gfx ];

            shellHook = ''
              alias cat=bat
            '';
          };
        });
}
