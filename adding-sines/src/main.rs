use std::f32::consts::PI;

use macroquad::prelude::*;
use mqanim::{
    plot::{Graph, GraphStyle, LabelStyle, MarkerStyle},
    Animation,
};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Template".to_owned(),
        sample_count: 16,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

enum EnabledFreqs {
    OneHz,
    TwoHz,
    ThreeHz,
    FourHz,
    FiveHz,
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
            tick_step: 1.,
            tick_style: mqanim::plot::TickStyle::LabelAndMarker {
                label_style: LabelStyle::default(),
                marker_style: MarkerStyle::default(),
            },
            ..Default::default()
        },
        y_style: mqanim::plot::AxisStyle {
            tick_step: 1.,
            tick_style: mqanim::plot::TickStyle::LabelAndMarker {
                label_style: LabelStyle::default(),
                marker_style: MarkerStyle::default(),
            },
            ..Default::default()
        },
    });

    let plot_to_copy: Vec<Vec2> = (0..RESOLUTION)
        .map(|val| {
            let t = mqanim::map(val as f32, 0., RESOLUTION as f32, -1., 1.);
            let v = f32::sin(2. * PI * 1. * t)
                + f32::sin(2. * PI * 3. * t)
                + f32::sin(2. * PI * 4. * t);

            vec2(t, v)
        })
        .collect();

    loop {
        animation.set_camera();
        graph.draw_axes();
        graph.plot_line_vec(&plot_to_copy, 2., ORANGE);

        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

