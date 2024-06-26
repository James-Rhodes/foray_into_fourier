use std::f32::consts::PI;

use macroquad::prelude::*;
use mqanim::{
    plot::{Graph, GraphStyle, LabelStyle, MarkerStyle},
    ui::{draw_text_centered, Button, ButtonShape},
    Animation,
};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Adding Sines".to_owned(),
        sample_count: 16,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

const RESOLUTION: u8 = 200;
#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    animation.enable_fxaa();

    let graph = Graph::new(
        vec2(0., 0.),
        vec2(WINDOW_WIDTH - 100., WINDOW_HEIGHT - 100.),
        -1.0..1.,
        -3.0..3.,
    )
    .style(GraphStyle {
        x_style: mqanim::plot::AxisStyle {
            tick_style: mqanim::plot::TickStyle::Nothing,
            line_thickness: 5.,
            ..Default::default()
        },
        y_style: mqanim::plot::AxisStyle {
            tick_style: mqanim::plot::TickStyle::Nothing,
            line_thickness: 5.,
            ..Default::default()
        },
    });

    let mut time = Vec::with_capacity(RESOLUTION as usize);
    let plot_to_copy: Vec<f32> = (0..RESOLUTION)
        .map(|val| {
            let t = mqanim::map(val as f32, 0., RESOLUTION as f32, -1., 1.);
            time.push(t);

            f32::sin(2. * PI * 1. * t) + f32::sin(2. * PI * 3. * t) + f32::sin(2. * PI * 4. * t)
        })
        .collect();

    let mut freqs = [true, false, false, false, false];
    let mut user_plot: Vec<f32> = vec![0.; plot_to_copy.len()];

    loop {
        animation.set_camera();
        graph.draw_axes();
        graph.plot_line_xy(&time, &plot_to_copy, 5., ORANGE);

        user_plot.iter_mut().zip(&time).for_each(|(val, t)| {
            *val = (freqs[0] as u32 as f32) * f32::sin(2. * PI * 1. * t);
            *val += (freqs[1] as u32 as f32) * f32::sin(2. * PI * 2. * t);
            *val += (freqs[2] as u32 as f32) * f32::sin(2. * PI * 3. * t);
            *val += (freqs[3] as u32 as f32) * f32::sin(2. * PI * 4. * t);
            *val += (freqs[4] as u32 as f32) * f32::sin(2. * PI * 5. * t);
        });

        let mouse_pos = animation.get_world_mouse();

        graph.plot_line_xy(&time, &user_plot, 5., BLUE);

        draw_all_buttons(&mut freqs, mouse_pos, vec2(320., 300.), 125.);

        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

fn draw_all_buttons(freqs: &mut [bool; 5], mouse_pos: Vec2, center_pos: Vec2, spacing: f32) {
    draw_toggle(
        "1Hz",
        vec2(center_pos.x - 2. * spacing, center_pos.y),
        mouse_pos,
        &mut freqs[0],
    );
    draw_toggle(
        "2Hz",
        vec2(center_pos.x - spacing, center_pos.y),
        mouse_pos,
        &mut freqs[1],
    );
    draw_toggle(
        "3Hz",
        vec2(center_pos.x, center_pos.y),
        mouse_pos,
        &mut freqs[2],
    );
    draw_toggle(
        "4Hz",
        vec2(center_pos.x + spacing, center_pos.y),
        mouse_pos,
        &mut freqs[3],
    );
    draw_toggle(
        "5Hz",
        vec2(center_pos.x + 2. * spacing, center_pos.y),
        mouse_pos,
        &mut freqs[4],
    );
}

fn draw_toggle(label: &str, pos: Vec2, mouse_pos: Vec2, data: &mut bool) {
    let width = 100.;
    let height = 85.;
    Button::new(pos, ButtonShape::Rectangle { width, height })
        .mouse_pos(mouse_pos)
        .draw(data);
    draw_text_centered(label, pos.x, pos.y, 70, WHITE);
}
