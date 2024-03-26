{
  description = "Mermaid LSP flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-23.11";
    systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixvim = {
      # If you are not running an unstable channel of nixpkgs,
      # select the corresponding branch of nixvim.
      # url = "github:nix-community/nixvim/nixos-23.05";
      url = "github:nix-community/nixvim";

      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = {
    self,
    nixpkgs,
    systems,
    rust-overlay,
    devenv,
    nixvim,
    ...
  } @ inputs: let
    overlays = [(import rust-overlay)];
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    packages = forEachSystem (system: let
      pkgs = import nixpkgs {inherit system overlays;};
      rustVersion = pkgs.rust-bin.stable.latest.default;
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustVersion;
        rustc = rustVersion;
      };
    in {
      # For setting up devenv
      devenv-up = self.devShells.${system}.default.config.procfileScript;

      nvim = nixvim.legacyPackages.${system}.makeNixvim {
        extraConfigLua = ''
          # TODO: Add extra config of vim here!
        '';
      };
    });

    devShells = forEachSystem (system: let
      pkgs = import nixpkgs {inherit system overlays;};
    in {
      default = devenv.lib.mkShell {
        inherit pkgs inputs;
        modules = [
          {
            packages = with pkgs; [
              # Latest stable release
              (rust-bin.stable.latest.default)
            ];
          }
        ];
      };
    });
  };
}
