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

Using [Nix](https://nixos.org/), e.g., can open `nix-shell`, then
development can be done with:

```
nix-shell$ cargo build
```

For just getting a binary, with Nix Flakes:

```
nix build
```

then run e.g. `./result/bin/action-game-3d`.

### Non-nix Setup

Otherwise, dependencies as per
[amethyst/amethyst](https://github.com/amethyst/amethyst#dependencies).

