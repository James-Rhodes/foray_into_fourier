use std::{collections::VecDeque, f32::consts::PI};

use macroquad::miniquad::conf::{Platform, WebGLVersion};
use macroquad::prelude::*;
use mqanim::{draw::draw_path, ui::draw_text_centered, Animation};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Rotating Phasor".to_owned(),
        sample_count: 4,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        // Render at the display's physical resolution. Without this the
        // canvas backing store is CSS-pixel sized and phones (with a
        // devicePixelRatio of 2-3) upscale it, which blurs text in particular.
        high_dpi: true,
        // WebGL1 has no multisampled render targets, which mqanim uses for
        // anti-aliasing. WebGL2 is supported by every browser that matters.
        platform: Platform {
            webgl_version: WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    animation.enable_fxaa();

    let mut t = 0.;
    let r = 75.;
    let center = vec2(-150., 75.);

    let max_cap = 300;
    let t_step = 2.;
    let mut prev_y: VecDeque<f32> = VecDeque::with_capacity(max_cap);
    let y_start_pos = vec2(50., 100.);
    let mut prev_x: VecDeque<f32> = VecDeque::with_capacity(max_cap);
    let x_start_pos = vec2(50., -50.);
    let mut y_trace: Vec<Vec2> = Vec::with_capacity(max_cap);
    let mut x_trace: Vec<Vec2> = Vec::with_capacity(max_cap);
    loop {
        t += 0.004;
        if t >= 100.0 {
            t = 0.;
        }
        animation.set_camera();
        let x = r * f32::cos(2. * PI * t);
        let y = r * f32::sin(2. * PI * t);

        prev_y.push_front(y);
        if prev_y.len() > max_cap {
            prev_y.pop_back();
        }
        prev_x.push_front(x);
        if prev_x.len() > max_cap {
            prev_x.pop_back();
        }

        mqanim::plot::Graph::new(center, vec2(2.2 * r, 2.2 * r), -1.0..1., -1.0..1.)
            .style(mqanim::plot::GraphStyle {
                x_style: mqanim::plot::AxisStyle {
                    line_thickness: 3.,
                    ..Default::default()
                },
                y_style: mqanim::plot::AxisStyle {
                    line_thickness: 3.,
                    ..Default::default()
                },
            })
            .draw_axes();
        let tip = vec2(x + center.x, y + center.y);
        draw_path(&[center, tip], 3., ORANGE);
        draw_path(&[tip, vec2(tip.x, x_start_pos.y)], 3., BLUE);
        draw_path(&[tip, vec2(y_start_pos.x, tip.y)], 3., PURPLE);
        draw_circle(tip.x, tip.y, 3., ORANGE);

        y_trace.clear();
        y_trace.extend(
            prev_y
                .iter()
                .enumerate()
                .map(|(idx, y)| vec2(y_start_pos.x + t_step * idx as f32, y + center.y)),
        );
        draw_path(&y_trace, 3., WHITE);

        x_trace.clear();
        x_trace.extend(
            prev_x
                .iter()
                .enumerate()
                .map(|(idx, x)| vec2(x + center.x, x_start_pos.y - t_step * idx as f32)),
        );
        draw_path(&x_trace, 3., WHITE);

        draw_text_centered("Phasor Position", 125., -75., 15, WHITE);
        draw_text_centered(
            &format!("cos({t:.2}) + j x sin({t:.2})"),
            125.,
            -100.,
            15,
            WHITE,
        );
        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}
