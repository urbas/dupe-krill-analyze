{
  description = "Helps you analyze dupe-krill's reports.";

  inputs.nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        rustPkg = pkgs.rustPlatform.buildRustPackage rec {
          nativeBuildInputs = [ pkgs.makeWrapper ];
          pname = cargoToml.package.name;
          src = self;
          version = cargoToml.package.version;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        dev-deps = with pkgs; [
          cargo-watch
          nixfmt-rfc-style
          rustup
        ];

      in
      {
        apps.default.program = "${rustPkg}/bin/${cargoToml.package.name}";
        apps.default.type = "app";

        packages.default = rustPkg;
        packages.${cargoToml.package.name} = rustPkg;

        devShells.default = pkgs.mkShell {
          packages = dev-deps;
        };
      }
    );
}
