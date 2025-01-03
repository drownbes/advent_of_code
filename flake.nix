{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }: 
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.stable.completeToolchain; in {
      devShells.default = pkgs.mkShell {
        shellHook = ''
          export SHELL="${pkgs.bashInteractive}/bin/bash"
          source "${toolchain}/etc/bash_completion.d/cargo"
        '';
        buildInputs = with pkgs;[
          toolchain
          aoc-cli
          just
          mermaid-cli
        ];
      };
    });
}
