use macroquad::prelude::*;
use mqanim::{ui::draw_text_centered, Animation};

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

#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    // animation.enable_fxaa();

    let mut pts = vec![];
    let mut world_pts = vec![];

    let texture: Texture2D = load_texture("assets/henry.png").await.unwrap();
    let henry_size = texture.size();
    loop {
        animation.set_camera();

        let mouse = animation.get_world_mouse();

        draw_text_centered(&format!("{}", pts.len()), -400., 350., 20, WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            let x = mqanim::map(mouse.x, -WINDOW_WIDTH / 2., WINDOW_WIDTH / 2., -1., 1.);
            let y = mqanim::map(mouse.y, -WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2., -1., 1.);
            pts.push(vec2(x, y));
            world_pts.push(mouse);
        }

        if is_key_pressed(KeyCode::Backspace) {
            pts.pop();
            world_pts.pop();
        }

        if is_key_pressed(KeyCode::P) {
            println!("{:?}", pts);
        }

        let scale = 1.3;
        draw_texture_ex(
            &texture,
            -henry_size.x * scale / 2.,
            -henry_size.y * scale / 2.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                dest_size: Some(vec2(henry_size.x * scale, henry_size.y * scale)),
                ..Default::default() // dest_size: todo!(),
                                     // source: todo!(),
                                     // rotation: todo!(),
                                     // flip_x: todo!(),
                                     // pivot: todo!(),
            },
        );

        world_pts
            .iter()
            .for_each(|pt| draw_circle(pt.x, pt.y, 2., PURPLE));
        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}
