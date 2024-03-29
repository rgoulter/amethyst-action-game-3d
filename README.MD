# Toy 3D Action Game using Amethyst Game Engine

![](https://raw.githubusercontent.com/rgoulter/amethyst-action-game-3d/master/docs/screenshot.png)

This is my project for playing with some Rust, an entity-component system (ECS)
library.

- [Amethyst](https://amethyst.rs/) game engine.

  - [This Rustconf keynote](https://kyren.github.io/2018/09/14/rustconf-talk.html)
    was a fascinating introduction to ECS.

- Textured 3D model.

  - The texture can be replaced (e.g. for unit's team colour).

  - The controls can move this model around.

- A spritesheet-tiled plane.

- A very simple main menu.

## Setup

#### Running using Nix Flakes

For running the application, with Nix Flakes:

```
nix run github:rgoulter/amethyst-action-game-3d
```

or from the repository root:

```
nix run
```

#### Running using Nix Flakes on non-NixOS

On non-NixOS, it's somewhat more involved.

[NixGL](https://github.com/guibou/nixGL) can be used to run the program.

So, with a fresh install of Nix the command to run the program is:

```
nix run github:guibou/nixGL#nixVulkanIntel \
  --no-write-lock-file \
  env LD_LIBRARY_PATH="" \
  nix run github:rgoulter/amethyst-action-game-3d
```

#### Compiling

###### Using Direnv + Nix Flakes

With [direnv](https://direnv.net/) and Nix Flakes, the `.envrc` uses
[nix-direnv](https://github.com/nix-community/nix-direnv/) to load
the development dependencies. Then run e.g.:

```
cargo build
```

###### Using `nix-shell`

Using [Nix](https://nixos.org/), e.g., can open `nix-shell`, then
development can be done with:

```
nix-shell$ cargo build
```

###### Non-nix Setup

Otherwise, dependencies as per
[amethyst/amethyst](https://github.com/amethyst/amethyst#dependencies).

