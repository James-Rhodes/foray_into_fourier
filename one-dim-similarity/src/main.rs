use macroquad::prelude::*;

pub mod animation;
use animation::Animation;

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
const INDICATOR_SIZE: (f32, f32) = (10., SIM_BAR_SIZE.1);

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

    let mut a = 0.0;
    let mut b = 0.0;
    let mut res;
    loop {
        animation.set_camera();
        res = a * b;

        draw_similarity_bar(&material, res);
        let mouse = animation.get_world_mouse();

        let slider_size = vec2(600., 40.);
        let slider_style = animation::ui::SliderStyle {
            bar_height: 30.,
            ..Default::default()
        };
        animation::ui::draw_text_centered(
            &format!("a = {:.2}", a),
            -slider_size.x / 2. - 125.,
            200.,
            60,
            WHITE,
        );
        animation::ui::Slider::new(vec2(0., 200.), slider_size, -10.0..10.)
            .mouse_pos(mouse)
            .style(slider_style)
            .draw(&mut a);

        animation::ui::draw_text_centered(
            &format!("b = {:.2}", b),
            -slider_size.x / 2. - 125.,
            100.,
            60,
            WHITE,
        );
        animation::ui::Slider::new(vec2(0., 100.), slider_size, -10.0..10.)
            .mouse_pos(mouse)
            .style(slider_style)
            .draw(&mut b);

        animation::ui::draw_text_centered(&format!("a x b = {res:.2}"), 0., 0., 70, WHITE);

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
        text_center_y + 4. * SIM_BAR_SIZE.1,
        70,
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
        WHITE,
    );
    animation::ui::draw_text_centered("100", SIM_BAR_SIZE.0 / 2., text_center_y, 50, WHITE);
    animation::ui::draw_text_centered("0", 0., text_center_y, 50, WHITE);
    animation::ui::draw_text_centered("-100", -SIM_BAR_SIZE.0 / 2., text_center_y, 50, WHITE);
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
    // vec3 out_color = mix(vec3(1.0, 0.0,0.0), vec3(0.0,1.0,0.0), uv.x);
    float diff = 0.003;
    if(uv.x < 0.5 - diff) {
        gl_FragColor = vec4(1.0,0.0,0.0, 1.0);
    }
    else if(uv.x > 0.5 + diff) {
        gl_FragColor = vec4(0.0,1.0,0.0, 1.0);
    }
    else{
        gl_FragColor = vec4(0.0,0.0,0.0, 1.0);
    }
    
}
"#;
