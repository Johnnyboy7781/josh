{
    description = "A basic shell written in Rust";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils, ... }: 
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                };
            in
            {
                packages.josh = pkgs.rustPlatform.buildRustPackage {
                    pname = "josh";
                    version = "0.0.1";
                    src = ./.;

                    cargoLock = {
                        lockFile = ./Cargo.lock;
                    };

                    meta = with pkgs.lib; {
                        description = "A basic shell written in Rust";
                        license = licenses.mit;
                        platforms = platforms.linux;
                        mainProgram = "josh";
                        shell = true;
                    };
                };

                defaultPackage = self.packages.${system}.josh;
            }
        );
}
