{
  pkgs ? import <nixpkgs> { },
}:
{
  system-manager = import ./install.nix {
    inherit pkgs;
    lib = pkgs.lib;
  };
}
