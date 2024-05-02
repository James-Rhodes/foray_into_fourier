use std::f32::consts::PI;

use macroquad::prelude::*;
use mqanim::{
    plot::{Graph, GraphStyle, LabelStyle, MarkerStyle},
    ui::draw_text_centered,
    Animation,
};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Amplitude and Phase".to_owned(),
        sample_count: 16,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

const RESOLUTION: u8 = 200;
const FONT_SIZE: u16 = 40;
#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    animation.enable_fxaa();

    const GRAPH_OFFSET: f32 = 250.;
    let graph = Graph::new(
        vec2(0., 0.),
        vec2(WINDOW_WIDTH - GRAPH_OFFSET, WINDOW_HEIGHT - GRAPH_OFFSET),
        -1.0..1.,
        -1.0..1.,
    )
    .style(GraphStyle {
        x_style: mqanim::plot::AxisStyle {
            tick_step: 0.5,
            line_thickness: 5.,
            tick_style: mqanim::plot::TickStyle::LabelAndMarker {
                label_style: LabelStyle {
                    font_size: FONT_SIZE,
                    pos_offset: vec2(0., -20.),
                    ..Default::default()
                },
                marker_style: MarkerStyle {
                    length: 10.,
                    ..Default::default()
                },
            },
            ..Default::default()
        },
        y_style: mqanim::plot::AxisStyle {
            tick_step: 0.5,
            line_thickness: 5.,
            tick_style: mqanim::plot::TickStyle::LabelAndMarker {
                label_style: LabelStyle {
                    font_size: FONT_SIZE,
                    pos_offset: vec2(-25., 0.),
                    ..Default::default()
                },
                marker_style: MarkerStyle {
                    length: 10.,
                    ..Default::default()
                },
            },
            ..Default::default()
        },
    });

    let mut time = Vec::with_capacity(RESOLUTION as usize);

    let slider_size = vec2(550., 40.);

    let mut amp = 1.;
    let mut phase = 0.;

    loop {
        let sine_wave: Vec<f32> = (0..RESOLUTION)
            .map(|val| {
                let t = mqanim::map(val as f32, 0., RESOLUTION as f32, -1., 1.);
                time.push(t);
                amp * f32::sin(2. * PI * t + phase)
            })
            .collect();

        animation.set_camera();

        graph.draw_axes();
        graph.plot_line_xy(&time, &sine_wave, 5., ORANGE);

        let mouse_pos = animation.get_world_mouse();

        let amp_pos = vec2(-350., 275.);
        draw_text_centered("Amplitude", amp_pos.x, amp_pos.y + 55., 70, WHITE);
        let bar_offset = 20.;
        mqanim::ui::Slider::new(amp_pos, slider_size, -1.0..1.)
            .style(mqanim::ui::SliderStyle {
                bar_height: slider_size.y - bar_offset,
                ..Default::default()
            })
            .mouse_pos(mouse_pos)
            .draw(&mut amp);

        let phase_pos = vec2(350., 275.);
        draw_text_centered("Phase", phase_pos.x, phase_pos.y + 55., 70, WHITE);
        mqanim::ui::Slider::new(phase_pos, slider_size, -2. * PI..2. * PI)
            .style(mqanim::ui::SliderStyle {
                bar_height: slider_size.y - bar_offset,
                ..Default::default()
            })
            .mouse_pos(mouse_pos)
            .draw(&mut phase);

        draw_text_centered(
            &format!("f(t) = {amp:.2} x sin(2 x pi x t + {phase:.2})"),
            0.,
            -300.,
            70,
            WHITE,
        );
        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}
