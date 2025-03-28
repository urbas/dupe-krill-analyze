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

        binDeps = with pkgs; [
          diffutils
        ];

        rustPkg = pkgs.rustPlatform.buildRustPackage {
          buildInputs = binDeps;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          pname = cargoToml.package.name;
          src = self;
          version = cargoToml.package.version;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          postInstall = ''
            wrapProgram $out/bin/${cargoToml.package.name} --prefix PATH : ${pkgs.lib.makeBinPath binDeps}
          '';
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
          inputsFrom = [ rustPkg ];
        };
      }
    );
}
