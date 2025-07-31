# Turbo Juice Demo

A small Turbo OS game showcasing a variety of low‑effort "juice" effects you can cycle through in real time.

Live demo: https://magical-cupcake-e8efbd.netlify.app/

## Features

- **Modal Tween**: Smoothly animate a nine‑slice modal box in and out with Start.
- **Screen Shake**: Toggle camera shake on Enter.
- **Pointer Pan**: Pan the camera to click/tap positions with pointer input.
- **Animated Tile**: Static repeating tiled background.
- **Scrolling Tile**: Tiled background that scrolls over time.
- **Blinking UI**: A blinking indicator in the bottom‑right corner.
- **Fast Animation**: Increase sprite animation speed.
- **Opacity Oscillation**: Smooth fade in/out of a sprite.
- **Color+Alpha Oscillation**: Apply oscillating tint and transparency.
- **Silhouette**: Draw sprite as a silhouette with border radius.
- **Bounce**: Vertical bounce animation of a sprite.

## Controls

| Input       | Action                            |
| ----------- | --------------------------------- |
| **Right**   | Cycle to next effect              |
| **Left**    | Cycle to previous effect          |
| **Space**   | Tween dialog box (effect #0)      |
| **Enter**   | Toggle screen shake (effect #1)   |
| **Pointer** | Pan camera to pointer (effect #2) |

> Note: On effect #0 (Modal Tween), press **Start** to trigger the box animation.

## Setup & Run

1. **Clone the repo**

   ```sh
   git clone https://github.com/yourname/turbo-juice-demo.git
   cd turbo-juice-demo
   ```

2. **Install Turbo CLI** (if not already):

   ```sh
   curl -sSfL https://turbo.computer/install.sh | sh
   ```

3. **Build & run**

   ```sh
   turbo run
   ```

4. **Play**

   - Use a gamepad or keyboard:

     - **Left/Right** to cycle effects
     - **Space**, **Enter**, and **Pointer** for context‑specific demos

## Code Overview

- **`GameState`** holds the current effect index and a `Tween<Bounds>` for the modal demo.
- **`update()`** matches on `effect_idx` and runs the corresponding logic.
- Utility functions:

  - `smooth_osc(a, b, t, period)` for smooth oscillation
  - `apply_alpha(color, o)` to blend an alpha channel

Each effect is entirely self‑contained in its `match` arm—feel free to isolate, extend, or repurpose.

**Don't forget to play around with the `shaders/juice.wgsl` file.** There are plenty of effects to uncomment within the `fs_main` function.

---

_Have fun juicing your Turbo OS games!_
