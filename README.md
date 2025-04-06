# System-Manager
Manages the nixos configuration for the system & home-manager.
I got tired of using complicated nix commands & cobbeled together scripts so i sat down one evening & made system-manager!

In less words, it makes managing the nix configurations less tedious, with some quality of life options.
A notable benefit is that it makes relocating the nix configuration directory easy, as the directory only needs setting
once & all future invocations will point to the set configuration path.

For a high-level usage overview see the [README](https://github.com/tye-exe/nixos-config/blob/main/README.md) for my configuration. For other curiousites indulge in the help menu.

## Build Insructions
There are multiple provided options for building system-manager:

#### With My Nix Config
If you use my nix configuration, system-manager will get added to the path & is usable directly from the shell. This is
the only supported way to manage a nix system with my configuration.

#### Adding Flake
You can add system-manager to your flake by adding it as an input.
```nix
system-manager = {
  url = "github:tye-exe/system-manager";
  inputs.nixpkgs.follows = "nixpkgs";
};
```
And then add it as a package with
```nix
inputs.system-manager.packages.${system}.system-manager
```

#### Cargo
Just run `cargo build --release` in this dir & the binary will be in `./target/releases/`. There's nothing that fancy 
here.
