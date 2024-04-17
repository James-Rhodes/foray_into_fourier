use macroquad::prelude::*;

pub mod animation;
use animation::Animation;
use macroquad::ui::hash;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "One Dimensional Similarity".to_owned(),
        sample_count: 16,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

const SIM_BAR_POS: (f32, f32) = (-500., -200.);
const SIM_BAR_SIZE: (f32, f32) = (1000., 25.);
const INDICATOR_SIZE: (f32, f32) = (5., SIM_BAR_SIZE.1);

#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: SIMILARITY_FRAG,
        },
        MaterialParams {
            ..Default::default()
        },
    )
    .unwrap();

    let mut a = -100.0;
    let mut b = 1.0;
    let mut res = 100.0;
    loop {
        animation.set_camera();
        draw_similarity_bar(&material, res);
        let mouse = animation.get_world_mouse();
        draw_circle(mouse.x, mouse.y, 10.0, ORANGE);

        animation::ui::Slider::new(vec2(0., 0.), vec2(100., 10.), -100.0..100.)
            .set_mouse_pos(mouse)
            .draw(&mut a);

        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

fn draw_similarity_bar(material: &Material, curr: f32) {
    gl_use_material(material);
    draw_rectangle(
        SIM_BAR_POS.0,
        SIM_BAR_POS.1,
        SIM_BAR_SIZE.0,
        SIM_BAR_SIZE.1,
        RED,
    );
    gl_use_default_material();

    let text_center_y = SIM_BAR_POS.1 - SIM_BAR_SIZE.1;
    animation::ui::draw_text_centered(
        "Similarity",
        0.,
        text_center_y + 3. * SIM_BAR_SIZE.1,
        30,
        WHITE,
    );

    // let sim_bar_x_max = SIM_BAR_POS.0 + SIM_BAR_SIZE.0;
    let indicator_x = curr * (-SIM_BAR_POS.0 / 100.) - INDICATOR_SIZE.0 / 2.;
    let indicator_y = SIM_BAR_POS.1;
    draw_rectangle(
        indicator_x,
        indicator_y,
        INDICATOR_SIZE.0,
        INDICATOR_SIZE.1,
        BLACK,
    );
    animation::ui::draw_text_centered("100", SIM_BAR_SIZE.0 / 2., text_center_y, 30, WHITE);
    animation::ui::draw_text_centered("0", 0., text_center_y, 30, WHITE);
    animation::ui::draw_text_centered("-100", -SIM_BAR_SIZE.0 / 2., text_center_y, 30, WHITE);
}

const DEFAULT_VERTEX_SHADER: &str = "
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying vec2 uv;
varying vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";

const SIMILARITY_FRAG: &str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec3 out_color = mix(vec3(1.0, 0.0,0.0), vec3(0.0,1.0,0.0), uv.x);
    gl_FragColor = vec4(out_color, 1.0);
}
"#;
