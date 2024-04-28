use macroquad::prelude::*;
use mqanim::{
    plot::{GraphStyle, LabelStyle},
    ui::draw_text_centered,
    Animation,
};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;
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

    let spacing = 150.;
    let size = vec2(WINDOW_WIDTH - spacing, WINDOW_HEIGHT - spacing);
    let graph =
        mqanim::plot::Graph::new(vec2(0., 0.), size, -5.0..5.0, -5.0..5.0).style(GraphStyle {
            x_style: mqanim::plot::AxisStyle {
                tick_step: 1.0,
                tick_style: mqanim::plot::TickStyle::LabelAndMarker {
                    label_style: LabelStyle {
                        font_size: 20,
                        pos_offset: vec2(0., -10.),
                        ..Default::default() // decimal_places: todo!(),
                                             // color: todo!(),
                    },
                    marker_style: mqanim::plot::MarkerStyle::default(),
                },
                ..Default::default()
            },
            y_style: mqanim::plot::AxisStyle {
                tick_step: 1.0,
                tick_style: mqanim::plot::TickStyle::LabelAndMarker {
                    label_style: LabelStyle {
                        font_size: 20,
                        pos_offset: vec2(-10., 0.),
                        ..Default::default() // decimal_places: todo!(),
                                             // color: todo!(),
                    },
                    marker_style: mqanim::plot::MarkerStyle::default(),
                },
                ..Default::default()
            },
        });
    loop {
        animation.set_camera();
        graph.draw_axes();
        let graph_arrow_pos = vec2(2., 3.);
        let arrow_pos = graph.graph_to_world(graph_arrow_pos);
        draw_arrow(vec2(0., 0.), arrow_pos, 3., 5., ORANGE);

        let mouse = animation.get_world_mouse();

        draw_arrow(vec2(0., 0.), mouse, 3., 5., BLUE);
        let graph_mouse_pos = graph.world_to_graph(mouse);

        let dot = graph_mouse_pos.x * graph_arrow_pos.x + graph_mouse_pos.y * graph_arrow_pos.y;
        let text = format!("Similarity: {:3.2}", dot);
        draw_text_centered(&text, 0., 370., 30, WHITE);
        let text = format!(
            "(2 x {:.2}) + (3 x {:.2}) = {:3.2}",
            graph_mouse_pos.x, graph_mouse_pos.y, dot
        );
        draw_text_centered(&text, 0., -350., 25, WHITE);
        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

fn draw_arrow(from: Vec2, to: Vec2, line_thickness: f32, arrow_head_thickness: f32, color: Color) {
    let new_to = from + 0.99 * (to - from);
    draw_line(from.x, from.y, new_to.x, new_to.y, line_thickness, color);

    // The maths from this is straight up stolen from https://math.stackexchange.com/questions/1314006/drawing-an-arrow

    let triangle_side_length = arrow_head_thickness * 2.;
    let line_length = (to - from).length();
    let length_ratio = triangle_side_length / line_length;

    let angle: f32 = std::f32::consts::PI / 6.;
    let cos_angle = f32::cos(angle);
    let sin_angle = f32::sin(angle);

    let pt_a = vec2(
        to.x + length_ratio * ((from.x - to.x) * cos_angle + (from.y - to.y) * sin_angle),
        to.y + length_ratio * ((from.y - to.y) * cos_angle - (from.x - to.x) * sin_angle),
    );
    let pt_b = to;
    let pt_c = vec2(
        to.x + length_ratio * ((from.x - to.x) * cos_angle - (from.y - to.y) * sin_angle),
        to.y + length_ratio * ((from.y - to.y) * cos_angle + (from.x - to.x) * sin_angle),
    );

    draw_triangle(pt_a, pt_b, pt_c, color);
}
