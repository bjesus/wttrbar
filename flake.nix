{
  description = "A simple but detailed weather indicator for Waybar using wttr.in";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      genSystems =
        func:
        lib.genAttrs
          [
            "aarch64-darwin"
            "aarch64-linux"
            "x86_64-darwin"
            "x86_64-linux"
          ]
          (
            system:
            func (
              import nixpkgs {
                inherit system;
                overlays = with self.overlays; [ wttrbar ];
              }
            )
          );

      mkDate =
        longDate:
        (lib.concatStringsSep "-" [
          (builtins.substring 0 4 longDate)
          (builtins.substring 4 2 longDate)
          (builtins.substring 6 2 longDate)
        ]);
    in
    {
      devShells = genSystems (pkgs: {
        default = pkgs.mkShell {
          name = "wttrbar-shell";

          # inherit attributes from upstream nixpkgs derivation
          inherit (pkgs.wttrbar)
            depsBuildBuild
            depsBuildBuildPropagated
            depsBuildTarget
            depsBuildTargetPropagated
            depsHostHost
            depsHostHostPropagated
            depsTargetTarget
            depsTargetTargetPropagated
            propagatedBuildInputs
            propagatedNativeBuildInputs
            strictDeps
            ;

          # overrides for local development
          buildInputs = (
            with pkgs;
            wttrbar.buildInputs
            ++ [
              cargo
              rustfmt
              clippy
            ]
          );

          nativeBuildInputs = (
            with pkgs; wttrbar.nativeBuildInputs ++ [ lldb ] ++ lib.optionals stdenv.isLinux [ gdb ]
          );
        };
      });

      overlays = {
        default = self.overlays.wttrbar;
        wttrbar = final: prev: {
          wttrbar = final.callPackage ./nix/default.nix {
            wttrbar = prev.wttrbar;
            version =
              let
                version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
              in
              version
              + "+date="
              + (mkDate (self.lastModifiedDate or "19700101"))
              + "_"
              + (self.shortRev or "dirty");
          };
        };
      };

      packages = genSystems (pkgs: {
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.wttrbar;
        inherit (pkgs) wttrbar;
      });
    };
}
