use std::{collections::VecDeque, f32::consts::PI};

use macroquad::prelude::*;
use mqanim::{ui::draw_text_centered, Animation};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Rotating Phasor".to_owned(),
        sample_count: 16,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    animation.enable_fxaa();

    let mut t = 0.;
    let r = 150.;
    let center = vec2(-300., 150.);

    let max_cap = 300;
    let t_step = 2.;
    let mut prev_y: VecDeque<f32> = VecDeque::with_capacity(max_cap);
    let y_start_pos = vec2(100., 200.);
    let mut prev_x: VecDeque<f32> = VecDeque::with_capacity(max_cap);
    let x_start_pos = vec2(100., -100.);
    loop {
        t += 0.005;
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
                    line_thickness: 5.,
                    ..Default::default()
                },
                y_style: mqanim::plot::AxisStyle {
                    line_thickness: 5.,
                    ..Default::default()
                },
            })
            .draw_axes();
        draw_line(
            0. + center.x,
            0. + center.y,
            x + center.x,
            y + center.y,
            5.,
            ORANGE,
        );
        draw_line(
            x + center.x,
            y + center.y,
            x + center.x,
            x_start_pos.y,
            5.,
            BLUE,
        );
        draw_line(
            x + center.x,
            y + center.y,
            y_start_pos.x,
            y + center.y,
            5.,
            PURPLE,
        );
        draw_circle(x + center.x, y + center.y, 5., ORANGE);

        let mut prev_pt = prev_y[0];
        prev_y.iter().enumerate().skip(1).for_each(|(idx, y)| {
            draw_line(
                y_start_pos.x + t_step * (idx - 1) as f32,
                prev_pt + center.y,
                y_start_pos.x + t_step * (idx as f32),
                *y + center.y,
                5.,
                WHITE,
            );

            prev_pt = *y;
        });

        let mut prev_pt = prev_x[0];
        prev_x.iter().enumerate().skip(1).for_each(|(idx, x)| {
            draw_line(
                prev_pt + center.x,
                x_start_pos.y - t_step * (idx - 1) as f32,
                *x + center.x,
                x_start_pos.y - t_step * (idx as f32),
                5.,
                WHITE,
            );

            prev_pt = *x;
        });

        draw_text_centered("Phasor Position", 250., -150., 60, WHITE);
        draw_text_centered(
            &format!("cos({t:.2}) + j x sin({t:.2})"),
            250.,
            -200.,
            60,
            WHITE,
        );
        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}
