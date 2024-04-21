use macroquad::prelude::*;
use std::ops::Range;

pub mod animation;
use animation::{ui::draw_text_centered, Animation};

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

    //TODO: Make the graph have nice defaults for Default so that the mess that is below is
    //contained

    //TODO: Make the graph plot things lol
    loop {
        animation.set_camera();
        let _mouse = animation.get_world_mouse();

        let graph = Graph::new(
            vec2(0., 0.),
            vec2(WINDOW_WIDTH - 100., WINDOW_HEIGHT - 100.),
            -1.0..1.0,
            -1.0..1.,
        )
        .style(GraphStyle {
            x_style: AxisStyle {
                tick_step: 0.2,
                tick_style: TickStyle::LabelAndLine {
                    label_style: LabelStyle {
                        pos_offset: vec2(0., 0.),
                        color: WHITE,
                        font_size: 12,
                        decimal_places: 2,
                    },
                    line_length: 5.,
                    line_thickness: 2.,
                    line_color: WHITE,
                },
                end_point_style: GraphEndPointStyle::Arrow { thickness: 7. },
                line_thickness: 3.,
                line_color: WHITE,
            },
            y_style: AxisStyle {
                tick_step: 0.1,
                tick_style: TickStyle::LabelAndLine {
                    label_style: LabelStyle {
                        pos_offset: vec2(0., 0.),
                        color: WHITE,
                        font_size: 12,
                        decimal_places: 2,
                    },
                    line_length: 5.,
                    line_thickness: 2.,
                    line_color: WHITE,
                },
                end_point_style: GraphEndPointStyle::Arrow { thickness: 7. },
                line_thickness: 3.,
                line_color: WHITE,
            },
        });
        graph.draw_axes();
        let pt = graph.graph_to_world(vec2(-0.5, -0.5));
        draw_circle(pt.x, pt.y, 10., ORANGE);

        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

#[derive(Copy, Clone)]
struct LabelStyle {
    pos_offset: Vec2,
    color: Color,
    font_size: u16,
    decimal_places: usize,
}

#[derive(Copy, Clone)]
enum TickStyle {
    LabelAndLine {
        label_style: LabelStyle,
        line_length: f32,
        line_thickness: f32,
        line_color: Color,
    },
    Line {
        line_length: f32,
        line_thickness: f32,
        line_color: Color,
    },
    Label {
        label_style: LabelStyle,
    },
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
    axes_pos: Vec2, // The position where the x and y axis cross
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
        assert!(
            x_range.start < x_range.end,
            "The x_range must have start smaller than end"
        );
        assert!(
            y_range.start < y_range.end,
            "The y_range must have start smaller than end"
        );

        let tick_style = TickStyle::LabelAndLine {
            label_style: LabelStyle {
                pos_offset: vec2(0., 0.),
                color: WHITE,
                font_size: 12,
                decimal_places: 2,
            },
            line_length: 3.,
            line_thickness: 1.,
            line_color: WHITE,
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
        if !graph.y_range.contains(&0.) && graph.y_range.end != 0. {
            graph.axes_pos.y = graph.world_min_coords.y;
        }
        if !graph.x_range.contains(&0.) && graph.x_range.end != 0. {
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
        let tick_start = if self.x_range.contains(&0.) {
            0.
        } else if self.x_range.start < 0. && self.x_range.end < 0. {
            self.x_range.end
        } else {
            self.x_range.start
        };

        let num_ticks = if self.x_range.start == 0. || self.x_range.end == 0. {
            ((self.x_range.end - self.x_range.start) / self.style.x_style.tick_step) + 1.
        } else {
            (self.x_range.end - self.x_range.start) / self.style.x_style.tick_step
        };
        let num_ticks = num_ticks.round() as usize;
        for tick_num in 0..num_ticks {
            let above = tick_start + tick_num as f32 * self.style.x_style.tick_step;
            let below = tick_start - tick_num as f32 * self.style.x_style.tick_step;

            if above <= self.x_range.end
                && (self.x_range.start == 0. || self.x_range.end == 0. || above != 0.)
            {
                let pt = self.graph_to_world(vec2(above, 0.));
                self.draw_tick(
                    pt,
                    above,
                    Orientation::Horizontal,
                    self.style.x_style.tick_style,
                );
            }
            if below >= self.x_range.start
                && (self.x_range.start == 0. || self.x_range.end == 0. || below != 0.)
            {
                let pt = self.graph_to_world(vec2(below, 0.));
                self.draw_tick(
                    pt,
                    below,
                    Orientation::Horizontal,
                    self.style.x_style.tick_style,
                );
            }
        }

        // Y axis
        let tick_start = if self.y_range.contains(&0.) {
            0.
        } else if self.y_range.start < 0. && self.y_range.end < 0. {
            self.y_range.end
        } else {
            self.y_range.start
        };

        let num_ticks = if self.y_range.start == 0. || self.y_range.end == 0. {
            ((self.y_range.end - self.y_range.start) / self.style.y_style.tick_step) + 1.
        } else {
            (self.y_range.end - self.y_range.start) / self.style.y_style.tick_step
        };
        let num_ticks = num_ticks.round() as usize;
        for tick_num in 0..num_ticks {
            let above = tick_start + tick_num as f32 * self.style.y_style.tick_step;
            let below = tick_start - tick_num as f32 * self.style.y_style.tick_step;

            if above <= self.y_range.end
                && (self.y_range.start == 0. || self.y_range.end == 0. || above != 0.)
            {
                let pt = self.graph_to_world(vec2(0., above));
                self.draw_tick(
                    pt,
                    above,
                    Orientation::Vertical,
                    self.style.y_style.tick_style,
                );
            }
            if below >= self.y_range.start
                && (self.y_range.start == 0. || self.y_range.end == 0. || below != 0.)
            {
                let pt = self.graph_to_world(vec2(0., below));
                self.draw_tick(
                    pt,
                    below,
                    Orientation::Vertical,
                    self.style.y_style.tick_style,
                );
            }
        }
    }

    fn draw_tick(&self, pos: Vec2, value: f32, orientation: Orientation, style: TickStyle) {
        let offset = -20.;
        let vert_offset = vec2(offset, 0.);
        let hori_offset = vec2(0., offset / 2.);
        let (line_params, label_style) = match (&orientation, style) {
            (
                Orientation::Horizontal,
                TickStyle::LabelAndLine {
                    label_style,
                    line_length,
                    line_thickness,
                    line_color,
                },
            ) => {
                let label_pos = pos + label_style.pos_offset + hori_offset;
                (
                    Some((
                        [
                            vec2(pos.x, pos.y + line_length / 2.),
                            vec2(pos.x, pos.y - line_length / 2.),
                        ],
                        line_thickness,
                        line_color,
                    )),
                    Some(LabelStyle {
                        pos_offset: label_pos,
                        ..label_style
                    }),
                )
            }
            (
                Orientation::Horizontal,
                TickStyle::Line {
                    line_length,
                    line_thickness,
                    line_color,
                },
            ) => (
                Some((
                    [
                        vec2(pos.x, pos.y + line_length / 2.),
                        vec2(pos.x, pos.y - line_length / 2.),
                    ],
                    line_thickness,
                    line_color,
                )),
                None,
            ),
            (Orientation::Horizontal, TickStyle::Label { label_style }) => {
                let label_pos = pos + label_style.pos_offset + hori_offset;
                (
                    None,
                    Some(LabelStyle {
                        pos_offset: label_pos,
                        ..label_style
                    }),
                )
            }
            (
                Orientation::Vertical,
                TickStyle::LabelAndLine {
                    label_style,
                    line_length,
                    line_thickness,
                    line_color,
                },
            ) => {
                let label_pos = pos + label_style.pos_offset + vert_offset;
                (
                    Some((
                        [
                            vec2(pos.x + line_length / 2., pos.y),
                            vec2(pos.x - line_length / 2., pos.y),
                        ],
                        line_thickness,
                        line_color,
                    )),
                    Some(LabelStyle {
                        pos_offset: label_pos,
                        ..label_style
                    }),
                )
            }
            (
                Orientation::Vertical,
                TickStyle::Line {
                    line_length,
                    line_thickness,
                    line_color,
                },
            ) => (
                Some((
                    [
                        vec2(pos.x + line_length / 2., pos.y),
                        vec2(pos.x - line_length / 2., pos.y),
                    ],
                    line_thickness,
                    line_color,
                )),
                None,
            ),
            (Orientation::Vertical, TickStyle::Label { label_style }) => {
                let label_pos = pos + label_style.pos_offset + vert_offset;
                (
                    None,
                    Some(LabelStyle {
                        pos_offset: label_pos,
                        ..label_style
                    }),
                )
            }
            (_, TickStyle::Nothing) => (None, None),
        };

        if let Some((line_pts, line_thickness, line_color)) = line_params {
            draw_line(
                line_pts[0].x,
                line_pts[0].y,
                line_pts[1].x,
                line_pts[1].y,
                line_thickness,
                line_color,
            )
        }
        if let Some(label_style) = label_style {
            let dp = label_style.decimal_places;
            draw_text_centered(
                &format!("{value:.dp$}"),
                label_style.pos_offset.x,
                label_style.pos_offset.y,
                label_style.font_size,
                label_style.color,
            )
        }
    }

    fn draw_axes_end_pts(&self) {
        let zero_position = self.graph_to_world(vec2(0.0, 0.0));
        // TODO: Make this into a function rather than the copy pasta below
        match self.style.x_style.end_point_style {
            GraphEndPointStyle::Arrow { thickness } => {
                let min_x = self.world_min_coords.x - thickness;
                let max_x = self.world_max_coords.x + thickness;
                let y = zero_position.y;
                if self.world_min_coords.x != zero_position.x {
                    draw_triangle(
                        vec2(min_x, y),
                        vec2(min_x + thickness, y + thickness),
                        vec2(min_x + thickness, y - thickness),
                        self.style.x_style.line_color,
                    );
                }
                if self.world_max_coords.x != zero_position.x {
                    draw_triangle(
                        vec2(max_x, y),
                        vec2(max_x - thickness, y + thickness),
                        vec2(max_x - thickness, y - thickness),
                        self.style.x_style.line_color,
                    );
                }
            }
            GraphEndPointStyle::Nothing => (),
        };
        match self.style.y_style.end_point_style {
            GraphEndPointStyle::Arrow { thickness } => {
                let min_y = self.world_min_coords.y - thickness;
                let max_y = self.world_max_coords.y + thickness;
                let x = zero_position.x;
                if self.world_min_coords.y != zero_position.y {
                    draw_triangle(
                        vec2(x, min_y),
                        vec2(x + thickness, min_y + thickness),
                        vec2(x - thickness, min_y + thickness),
                        self.style.x_style.line_color,
                    );
                }
                if self.world_max_coords.x != zero_position.y {
                    draw_triangle(
                        vec2(x, max_y),
                        vec2(x + thickness, max_y - thickness),
                        vec2(x - thickness, max_y - thickness),
                        self.style.x_style.line_color,
                    );
                }
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
