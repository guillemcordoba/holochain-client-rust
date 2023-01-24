let
  holonixPath = (import ./nix/sources.nix).holonix;
  holonix = import (holonixPath) {
    holochainVersionId = "v0_1_0-beta-rc_4";
    include = {
      node = false;
      happs = false;
      scaffolding = false;
      launcher = false;
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # niv
    nodejs
  ];
}