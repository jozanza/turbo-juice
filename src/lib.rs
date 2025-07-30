use turbo::*;

// Demo of various visual "juice" effects, cycled with Start button
#[turbo::game]
struct GameState {
    effect_idx: usize,
    modal: Tween<Bounds>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            effect_idx: 0,
            modal: Tween::new(bounds::screen())
                .ease(Easing::EaseInOutQuad)
                .duration(20),
        }
    }
    pub fn update(&mut self) {
        let t = time::tick();
        let gp = gamepad::get(0);
        let kb = keyboard::get();
        let wp = pointer::world();

        // Cycle through effects (debounced)
        if gp.right.just_pressed() {
            self.effect_idx = (self.effect_idx + 1) % 11;
            camera::reset();
            camera::remove_shake();
        }
        if gp.left.just_pressed() {
            self.effect_idx = if self.effect_idx == 0 {
                10
            } else {
                self.effect_idx - 1
            };
            camera::reset();
            camera::remove_shake();
        }

        // Clear background
        clear(0x000000ff);
        let screen_bounds = bounds::screen();
        let sprite_bounds = bounds::new(16, 16).anchor_center(&screen_bounds);

        // Draw current effect
        match self.effect_idx {
            // 0: Modal bounds tween
            0 => {
                if self.modal.done() && gp.start.just_pressed() {
                    let sb = bounds::screen();
                    let half = sb
                        .adjust_width_by_fraction(0.5)
                        .adjust_height_by_fraction(0.5)
                        .anchor_center(&sb);
                    let curr = self.modal.get();
                    self.modal.set(if curr == sb { half } else { sb });
                }
                nine_slice!(
                    "nslice_metal_pipes",
                    margins = (8, 8, 8, 8),
                    fixed = true,
                    bounds = self.modal.get()
                );
                text_box!(
                    "Nine-slice text box demo!\n(Press Start)",
                    font = "OldWizard",
                    fixed = true,
                    bounds = self.modal.get().inset(12),
                    end = t / 4,
                );
            }

            // 1: Screen shake toggle on Enter
            1 => {
                if kb.enter().just_pressed() {
                    if camera::is_shaking() {
                        camera::remove_shake();
                    } else {
                        camera::shake(1);
                    }
                }
                sprite!("evil_turbi", bounds = sprite_bounds,);
            }

            // 2: Pointer pan
            2 => {
                if wp.pressed() {
                    camera::pan_xy(wp.xy(), 60, Easing::EaseOutQuad);
                }
                if kb.enter().just_pressed() {
                    camera::reset();
                }
                sprite!("floor_forest", repeat = true, bounds = screen_bounds);
                sprite!("evil_turbi", bounds = sprite_bounds,);
            }

            // 3: Tiled animated background
            3 => {
                sprite!(
                    "twinkle_stars",
                    fixed = true,
                    bounds = bounds::screen(),
                    repeat = true
                );
            }

            // 4: Scrolling tiled background
            4 => {
                sprite!(
                    "gold_coin",
                    fixed = true,
                    bounds = bounds::screen(),
                    repeat = true,
                    tx = t,
                    ty = t,
                );
            }

            // 5: Blinking UI indicator
            5 => {
                let b = bounds::new(4.0, 4.0)
                    .anchor_bottom(&bounds::screen())
                    .anchor_right(&bounds::screen())
                    .translate_x(-12.0)
                    .translate_y(-12.0);
                rect!(
                    fixed = true,
                    bounds = b,
                    opacity = if t % 32 < 16 { 1. } else { 0. }
                );
                sprite!("evil_turbi", bounds = sprite_bounds,);
            }

            // 6: Animation speed change
            6 => {
                sprite!(
                    "evil_turbi",
                    bounds = sprite_bounds,
                    animation_speed = 4.0,
                );
            }

            // 7: Opacity oscillation
            7 => {
                sprite!(
                    "evil_turbi",
                    fixed = true,
                    bounds = sprite_bounds,
                    opacity = smooth_osc(0.0, 1.0, t, 60)
                );
            }

            // 8: Color alpha oscillation
            8 => {
                sprite!("evil_turbi", bounds = sprite_bounds,);
                let c = apply_alpha(0x00ff00ff, smooth_osc(0.5, 1.0, t, 20));
                sprite!("evil_turbi", bounds = sprite_bounds, color = c);
            }

            // 9: Silhouette effect
            9 => {
                sprite!(
                    "evil_turbi",
                    bounds = sprite_bounds,
                    color = 0x000000ff,
                    background_color = 0xff00ffff,
                    border_radius = 6,
                );
            }

            // 10: Bouncing sprite demo
            10 => {
                sprite!(
                    "gold_coin",
                    bounds = sprite_bounds,
                    y = smooth_osc(64.0, 68.0, t, 20)
                );
            }

            _ => {}
        }

        circ!(color = 0x000000ff, d = 8, fixed = true);
        text!("{}", self.effect_idx; fixed = true, font = "small", x= 2, y = 1);
        // Description box at bottom
        let desc = match self.effect_idx {
            0 => "> Tween modal bounds with Start",
            1 => "> Toggle screen shake with Enter",
            2 => "> Pan camera to click/tap",
            3 => "> Animated repeating tiled background",
            4 => "> Scrolling tiled background",
            5 => "> Blinking UI indicator",
            6 => "> Increased animation speed",
            7 => "> Oscillating opacity",
            8 => "> Color+alpha oscillation",
            9 => "> Silhouette overlay",
            10 => "> Vertical bounce animation",
            _ => "",
        };

        let desc_bounds = screen_bounds.height(12).anchor_bottom(&screen_bounds);
        rect!(
            color = 0x000000ff,
            fixed = true,
            bounds = desc_bounds,
            opacity = 0.9
        );
        text_box!(
            desc,
            fixed = true,
            bounds = desc_bounds.inset_top(2).inset_left(2)
        );
    }
}

/// Smooth oscillation between a and b over period ticks
fn smooth_osc(a: f32, b: f32, t: usize, period: usize) -> f32 {
    let phase = (t % period) as f32 / period as f32 * 2.0 * std::f32::consts::PI;
    let mid = (a + b) * 0.5;
    let amp = (b - a) * 0.5;
    mid + amp * phase.sin()
}

/// Apply opacity o to a color (RGBA u32)
fn apply_alpha(color: u32, o: f32) -> u32 {
    let o = o.clamp(0.0, 1.0);
    let r = (((color >> 24) & 0xFF) as f32 * o).round() as u32;
    let g = (((color >> 16) & 0xFF) as f32 * o).round() as u32;
    let b = (((color >> 8) & 0xFF) as f32 * o).round() as u32;
    let a = (o * 255.0).round() as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}
