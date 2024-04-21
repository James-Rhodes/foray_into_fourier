use macroquad::prelude::*;
use std::ops::Range;

pub mod animation;
use animation::Animation;

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

    loop {
        animation.set_camera();
        let _mouse = animation.get_world_mouse();

        let graph = Graph::new(
            vec2(0., 0.),
            vec2(WINDOW_WIDTH - 100., WINDOW_HEIGHT - 100.),
            -1.0..1.,
            -1.0..1.,
        )
        .style(GraphStyle {
            x_style: AxisStyle {
                tick_step: 0.1,
                tick_style: TickStyle::LabelAndLine {
                    label_style: LabelStyle {
                        pos: LabelPos::Below,
                    },
                },
                end_point_style: GraphEndPointStyle::Arrow { thickness: 7. },
                line_thickness: 3.,
                line_color: WHITE,
            },
            y_style: AxisStyle {
                tick_step: 0.1,
                tick_style: TickStyle::LabelAndLine {
                    label_style: LabelStyle {
                        pos: LabelPos::Below,
                    },
                },
                end_point_style: GraphEndPointStyle::Arrow { thickness: 7. },
                line_thickness: 3.,
                line_color: WHITE,
            },
        });
        graph.draw_axes();

        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

#[derive(Copy, Clone)]
enum LabelPos {
    Above,
    Below,
}

#[derive(Copy, Clone)]
struct LabelStyle {
    pos: LabelPos,
}

#[derive(Copy, Clone)]
enum TickStyle {
    LabelAndLine { label_style: LabelStyle },
    Line,
    Label { label_style: LabelStyle },
    Nothing,
}

enum GraphEndPointStyle {
    Arrow { thickness: f32 },
    Nothing,
}

struct AxisStyle {
    pub tick_step: f32,
    pub tick_style: TickStyle,
    pub end_point_style: GraphEndPointStyle,
    pub line_thickness: f32,
    pub line_color: Color,
}
struct GraphStyle {
    pub x_style: AxisStyle,
    pub y_style: AxisStyle,
}

struct Graph {
    world_center_pos: Vec2,
    world_size: Vec2,
    x_range: Range<f32>,
    y_range: Range<f32>,
    style: GraphStyle,
    world_min_coords: Vec2,
    world_max_coords: Vec2,
    axes_pos: Vec2,
}

enum Orientation {
    Horizontal,
    Vertical,
}

impl Graph {
    pub fn new(
        world_center_pos: Vec2,
        world_size: Vec2,
        x_range: Range<f32>,
        y_range: Range<f32>,
    ) -> Self {
        let tick_style = TickStyle::LabelAndLine {
            label_style: LabelStyle {
                pos: LabelPos::Below,
            },
        };

        let style = GraphStyle {
            x_style: AxisStyle {
                tick_step: (x_range.end - x_range.start) / 10.,
                tick_style,
                end_point_style: GraphEndPointStyle::Nothing,
                line_thickness: 2.,
                line_color: WHITE,
            },
            y_style: AxisStyle {
                tick_step: (y_range.end - y_range.start) / 10.,
                tick_style,
                end_point_style: GraphEndPointStyle::Nothing,
                line_thickness: 2.,
                line_color: WHITE,
            },
        };

        let mut graph = Self {
            world_center_pos,
            world_size,
            x_range,
            y_range,
            style,
            world_min_coords: vec2(
                world_center_pos.x - world_size.x / 2.,
                world_center_pos.y - world_size.y / 2.,
            ),
            world_max_coords: vec2(
                world_center_pos.x + world_size.x / 2.,
                world_center_pos.y + world_size.y / 2.,
            ),
            axes_pos: vec2(0., 0.),
        };

        graph.axes_pos = graph.graph_to_world(vec2(0.0, 0.0));
        if !graph.y_range.contains(&0.) {
            graph.axes_pos.y = graph.world_min_coords.y;
        }
        if !graph.x_range.contains(&0.) {
            graph.axes_pos.x = graph.world_min_coords.x;
        }

        graph
    }
    pub fn style(mut self, style: GraphStyle) -> Self {
        self.style = style;
        self
    }

    pub fn draw_axes(&self) {
        // Draw X Axis
        draw_line(
            self.world_min_coords.x,
            self.axes_pos.y,
            self.world_max_coords.x,
            self.axes_pos.y,
            self.style.y_style.line_thickness,
            self.style.y_style.line_color,
        );

        // Draw Y Axis
        draw_line(
            self.axes_pos.x,
            self.world_min_coords.y,
            self.axes_pos.x,
            self.world_max_coords.y,
            self.style.x_style.line_thickness,
            self.style.x_style.line_color,
        );

        self.draw_axes_end_pts();
        self.draw_ticks();
    }
    fn draw_ticks(&self) {
        if matches!(self.style.x_style.tick_style, TickStyle::Nothing)
            && matches!(self.style.y_style.tick_style, TickStyle::Nothing)
        {
            return;
        }

        // X axis
        let mut curr_step = 0.;
        loop {
            curr_step += self.style.x_style.tick_step;
            // TODO: This needs to also account for graphs where 0,0 isn't in the graph. So we
            // can't always just walk outwards from the 0 point. We need to start at the axis pts
            // and then work from there while also making it evenly spaced if it does contain 0,0
            todo!()
        }
    }

    fn draw_tick(&self, orientation: Orientation, style: TickStyle) {
        todo!()
    }

    fn draw_axes_end_pts(&self) {
        let zero_position = self.graph_to_world(vec2(0.0, 0.0));
        match self.style.x_style.end_point_style {
            GraphEndPointStyle::Arrow { thickness } => {
                let min_x = self.world_min_coords.x - thickness;
                let max_x = self.world_max_coords.x + thickness;
                let y = zero_position.y;
                draw_triangle(
                    vec2(min_x, y),
                    vec2(min_x + thickness, y + thickness),
                    vec2(min_x + thickness, y - thickness),
                    self.style.x_style.line_color,
                );
                draw_triangle(
                    vec2(max_x, y),
                    vec2(max_x - thickness, y + thickness),
                    vec2(max_x - thickness, y - thickness),
                    self.style.x_style.line_color,
                );
            }
            GraphEndPointStyle::Nothing => (),
        };
        match self.style.y_style.end_point_style {
            GraphEndPointStyle::Arrow { thickness } => {
                let min_y = self.world_min_coords.y - thickness;
                let max_y = self.world_max_coords.y + thickness;
                let x = zero_position.x;
                draw_triangle(
                    vec2(x, min_y),
                    vec2(x + thickness, min_y + thickness),
                    vec2(x - thickness, min_y + thickness),
                    self.style.x_style.line_color,
                );
                draw_triangle(
                    vec2(x, max_y),
                    vec2(x + thickness, max_y - thickness),
                    vec2(x - thickness, max_y - thickness),
                    self.style.x_style.line_color,
                );
            }
            GraphEndPointStyle::Nothing => (),
        };
    }
    pub fn graph_to_world(&self, pt: Vec2) -> Vec2 {
        vec2(
            animation::map(
                pt.x,
                self.x_range.start,
                self.x_range.end,
                self.world_min_coords.x,
                self.world_max_coords.x,
            ),
            animation::map(
                pt.y,
                self.y_range.start,
                self.y_range.end,
                self.world_min_coords.y,
                self.world_max_coords.y,
            ),
        )
    }
}
