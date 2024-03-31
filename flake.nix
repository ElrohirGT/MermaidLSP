{
  description = "Mermaid LSP flake";
  inputs = {
    # Basic inputs
    nixpkgs.url = "nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";

    # DevEnv setup
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Inputs for custom vim installation with MermaidLSP setup
    rust-overlay.url = "github:oxalica/rust-overlay";
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
      mermaidLspPackage = rustPlatform.buildRustPackage {
        pname = "mermaid_lsp";
        version = "0.1.0";
        src = ./mermaid_lsp;
        cargoLock.lockFile = ./mermaid_lsp/Cargo.lock;
      };
    in {
      # For setting up devenv
      devenv-up = self.devShells.${system}.default.config.procfileScript;

      # Mermaid LSP
      mermaidLSP = mermaidLspPackage;

      nvim = nixvim.legacyPackages.${system}.makeNixvim {
        colorschemes.oxocarbon.enable = true;
        plugins.lsp.enable = true;
        plugins.cmp = {
          enable = true;
          autoEnableSources = true;
          settings = {
            mapping = {
              "<C-d>" = "cmp.mapping.scroll_docs(-4)";
              "<C-f>" = "cmp.mapping.scroll_docs(4)";

              # Manually trigger a completion from nvim-cmp
              "<C-Space>" = "cmp.mapping.complete()";
              "<C-e>" = "cmp.mapping.close()";
              "<C-n>" = "cmp.mapping.select_next_item()";
              "<C-p>" = "cmp.mapping.select_prev_item()";
              "<CR>" = "cmp.mapping.confirm({ select = true })";
            };
            sources = [{name = "nvim_lsp";}];
          };
        };
        extraConfigLua = ''
          -- TODO: Add extra config of vim here!
          local client = vim.lsp.start_client {
          	name = "mermaid_lsp",
          	cmd = {"${mermaidLspPackage}/bin/mermaid_lsp"}
          }

          if not client then
          	vim.notify "Hey! You did an upsie configuring the client for the LSP!"
          	return
          end

          vim.api.nvim_create_autocmd("FileType", {
          	pattern = "mermaid",
          	callback = function()
          		vim.lsp.buf_attach_client(0, client)
          	end
          })
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
            languages.rust = {
              enable = true;
              channel = "stable";
            };

            pre-commit = {
              hooks = {
                # Linters
                clippy.enable = true;
                actionlint.enable = true;
                yamllint.enable = true;
                cargo-check.enable = true;
                commitizen.enable = true;

                # Formatters
                taplo.enable = true;
                alejandra.enable = true;
              };
              settings.rust = {
                cargoManifestPath = "mermaid_lsp/Cargo.toml";
              };
              settings.clippy = {
                allFeatures = true;
              };
            };
          }
        ];
      };
    });
  };
}
