use macroquad::prelude::*;
enum RenderState {
    CameraRendering,
    ScreenRendering,
}

pub struct Animation {
    pub render_target: RenderTarget,
    camera: Camera2D,
    pub bg_color: Color,
    render_state: RenderState,
    pub draw_size: Vec2,
    material: Option<Material>,
    width: f32,
    height: f32,
    scale: f32,
}

impl Animation {
    pub fn new(width: f32, height: f32, bg_color: Option<Color>) -> Self {
        // Screen dimensions will be:
        //     x: -width/2 -> width/2 (left -> right)
        //     y: -height/2 -> height/2 (bottom -> top)
        let render_target = render_target(width as u32, height as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., width, height));

        camera.render_target = Some(render_target.clone());
        camera.target = vec2(0., 0.);

        let bg_color = if let Some(bg_color) = bg_color {
            bg_color
        } else {
            Color {
                r: 43. / 255.,
                g: 44. / 255.,
                b: 47. / 255.,
                a: 1.,
            }
        };

        Self {
            render_target,
            camera,
            bg_color,
            render_state: RenderState::ScreenRendering,
            draw_size: vec2(width, height),
            material: None,
            width,
            height,
            scale: Self::compute_scale(width, height),
        }
    }

    pub fn get_world_mouse(&self) -> Vec2 {
        let mouse: Vec2 = mouse_position().into();
        self.screen_to_world(mouse)
    }

    pub fn screen_to_world(&self, point: Vec2) -> Vec2 {
        // Mouse position in the virtual screen
        Vec2 {
            x: ((point.x - (screen_width() - (self.width * self.scale)) * 0.5) / self.scale)
                - 0.5 * self.width,
            y: 0.5 * self.height
                - (point.y - (screen_height() - (self.height * self.scale)) * 0.5) / self.scale,
        }
    }

    pub fn enable_fxaa(&mut self) {
        let uniforms = vec![("texture_size".to_string(), UniformType::Float2)];
        let material = load_material(
            ShaderSource::Glsl {
                vertex: FXAA_VERTEX_SHADER,
                fragment: FXAA_FRAGMENT_SHADER,
            },
            MaterialParams {
                uniforms,
                ..Default::default()
            },
        )
        .unwrap();

        self.material = Some(material);
    }

    pub fn set_camera(&mut self) {
        set_camera(&self.camera);
        clear_background(self.bg_color);
        self.render_state = RenderState::CameraRendering;
    }

    pub fn set_default_camera(&mut self) {
        self.render_state = RenderState::ScreenRendering;
        set_default_camera();
    }

    pub fn draw_frame(&mut self) {
        if matches!(self.render_state, RenderState::CameraRendering) {
            panic!("Animation::set_default_camera must be called before you can draw the frame to the screen");
        }

        if let Some(material) = &self.material {
            material.set_uniform("texture_size", self.draw_size);
            gl_use_material(material);
        } else {
            gl_use_default_material();
        }

        clear_background(self.bg_color);

        self.scale = Self::compute_scale(self.width, self.height);

        self.draw_size = vec2(self.width * self.scale, self.height * self.scale);
        // Draw 'render_target' to window screen, porperly scaled and letterboxed
        draw_texture_ex(
            &self.render_target.texture,
            (screen_width() - (self.width * self.scale)) * 0.5,
            (screen_height() - (self.height * self.scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.draw_size),
                ..Default::default()
            },
        );
    }

    fn compute_scale(width: f32, height: f32) -> f32 {
        f32::min(screen_width() / width, screen_height() / height)
    }
}

pub fn map(val: f32, min1: f32, max1: f32, min2: f32, max2: f32) -> f32 {
    ((val - min1) / (max1 - min1)) * (max2 - min2) + min2
}
pub mod ui {
    use super::map;
    use std::ops::Range;

    use macroquad::prelude::*;
    pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
        let text_center = get_text_center(text, None, font_size, 1., 0.);
        draw_text_ex(
            text,
            x - text_center.x,
            y + text_center.y,
            TextParams {
                font_size,
                font_scale: -1.,
                font_scale_aspect: -1.,
                color,
                ..Default::default()
            },
        );
    }

    #[derive(Copy, Clone)]
    pub struct SliderStyle {
        pub bar_height: f32,
        pub bar_color: Color,
        pub marker_color: Color,
    }
    impl Default for SliderStyle {
        fn default() -> Self {
            Self {
                bar_height: 5.,
                bar_color: WHITE,
                marker_color: GRAY,
            }
        }
    }

    pub struct Slider {
        center_pos: Vec2,
        size: Vec2,
        style: SliderStyle,
        mouse_pos: Option<Vec2>,
        range: Range<f32>,
        min_coords: Vec2,
        max_coords: Vec2,
    }
    impl Slider {
        pub fn new(center_pos: Vec2, size: Vec2, range: Range<f32>) -> Self {
            let min_coords = vec2(center_pos.x - size.x / 2., center_pos.y - size.y / 2.);
            let max_coords = vec2(center_pos.x + size.x / 2., center_pos.y + size.y / 2.);
            Self {
                center_pos,
                size,
                style: SliderStyle::default(),
                mouse_pos: None,
                range,
                min_coords,
                max_coords,
            }
        }
        pub fn style(mut self, style: SliderStyle) -> Self {
            self.style = style;
            self
        }
        pub fn mouse_pos(mut self, mouse_pos: Vec2) -> Self {
            self.mouse_pos = Some(mouse_pos);
            self
        }
        pub fn draw(&self, data: &mut f32) {
            let mouse_pos = if let Some(mouse_pos) = self.mouse_pos {
                mouse_pos
            } else {
                mouse_position().into()
            };

            let draw_x = self.center_pos.x - self.size.x / 2.;
            let draw_y = self.center_pos.y - self.style.bar_height / 2.;
            draw_rectangle(
                draw_x,
                draw_y,
                self.size.x,
                self.style.bar_height,
                self.style.bar_color,
            );

            let marker_x = map(
                *data,
                self.range.start,
                self.range.end,
                -self.size.x / 2. + self.center_pos.x,
                self.size.x / 2. + self.center_pos.x,
            );
            let marker_pos = vec2(marker_x, self.center_pos.y);
            let mouse_intersects_bb = mouse_pos.x >= (self.min_coords.x - self.size.y / 2.)
                && mouse_pos.x <= (self.max_coords.x + self.size.y / 2.)
                && mouse_pos.y <= self.max_coords.y
                && mouse_pos.y >= self.min_coords.y;
            if is_mouse_button_down(MouseButton::Left) && mouse_intersects_bb {
                *data = map(
                    mouse_pos.x,
                    self.min_coords.x,
                    self.max_coords.x,
                    self.range.start,
                    self.range.end,
                );
                *data = data.clamp(self.range.start, self.range.end);
            }
            draw_circle(
                marker_pos.x,
                marker_pos.y,
                self.size.y / 2.,
                self.style.marker_color,
            );
        }
    }

    #[derive(Copy, Clone)]
    pub struct LabelStyle {
        pub pos_offset: Vec2,
        pub color: Color,
        pub font_size: u16,
        pub decimal_places: usize,
    }

    impl Default for LabelStyle {
        fn default() -> Self {
            Self {
                pos_offset: vec2(0., 0.),
                color: WHITE,
                font_size: 12,
                decimal_places: 2,
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct MarkerStyle {
        pub length: f32,
        pub thickness: f32,
        pub color: Color,
    }
    impl Default for MarkerStyle {
        fn default() -> Self {
            Self {
                length: 5.,
                thickness: 2.,
                color: WHITE,
            }
        }
    }

    #[derive(Default, Copy, Clone)]
    pub enum TickStyle {
        LabelAndMarker {
            label_style: LabelStyle,
            marker_style: MarkerStyle,
        },
        Marker {
            style: MarkerStyle,
        },
        Label {
            style: LabelStyle,
        },

        #[default]
        Nothing,
    }

    pub enum GraphEndPointStyle {
        Arrow { thickness: f32 },
        Nothing,
    }

    impl Default for GraphEndPointStyle {
        fn default() -> Self {
            GraphEndPointStyle::Arrow { thickness: 7. }
        }
    }

    pub struct AxisStyle {
        pub tick_step: f32,
        pub tick_style: TickStyle,
        pub end_point_style: GraphEndPointStyle,
        pub line_thickness: f32,
        pub line_color: Color,
    }

    impl Default for AxisStyle {
        fn default() -> Self {
            AxisStyle {
                tick_step: 0.5,
                tick_style: TickStyle::default(),
                end_point_style: GraphEndPointStyle::default(),
                line_thickness: 3.,
                line_color: WHITE,
            }
        }
    }

    #[derive(Default)]
    pub struct GraphStyle {
        pub x_style: AxisStyle,
        pub y_style: AxisStyle,
    }

    pub struct Graph {
        world_center_pos: Vec2,
        world_size: Vec2,
        x_range: Range<f32>,
        y_range: Range<f32>,
        style: GraphStyle,
        world_min_coords: Vec2,
        world_max_coords: Vec2,
        axes_pos: Vec2, // The position where the x and y axis cross
    }

    pub enum Orientation {
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

            let mut graph = Self {
                world_center_pos,
                world_size,
                x_range,
                y_range,
                style: GraphStyle::default(),
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
        pub fn world_center_pos(&self) -> Vec2 {
            self.world_center_pos
        }

        pub fn world_size(&self) -> Vec2 {
            self.world_size
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
                    TickStyle::LabelAndMarker {
                        label_style,
                        marker_style,
                    },
                ) => {
                    let label_pos = pos + label_style.pos_offset + hori_offset;
                    (
                        Some((
                            [
                                vec2(pos.x, pos.y + marker_style.length / 2.),
                                vec2(pos.x, pos.y - marker_style.length / 2.),
                            ],
                            marker_style.thickness,
                            marker_style.color,
                        )),
                        Some(LabelStyle {
                            pos_offset: label_pos,
                            ..label_style
                        }),
                    )
                }
                (Orientation::Horizontal, TickStyle::Marker { style }) => (
                    Some((
                        [
                            vec2(pos.x, pos.y + style.length / 2.),
                            vec2(pos.x, pos.y - style.length / 2.),
                        ],
                        style.thickness,
                        style.color,
                    )),
                    None,
                ),
                (Orientation::Horizontal, TickStyle::Label { style }) => {
                    let label_pos = pos + style.pos_offset + hori_offset;
                    (
                        None,
                        Some(LabelStyle {
                            pos_offset: label_pos,
                            ..style
                        }),
                    )
                }
                (
                    Orientation::Vertical,
                    TickStyle::LabelAndMarker {
                        label_style,
                        marker_style,
                    },
                ) => {
                    let label_pos = pos + label_style.pos_offset + vert_offset;
                    (
                        Some((
                            [
                                vec2(pos.x + marker_style.length / 2., pos.y),
                                vec2(pos.x - marker_style.length / 2., pos.y),
                            ],
                            marker_style.thickness,
                            marker_style.color,
                        )),
                        Some(LabelStyle {
                            pos_offset: label_pos,
                            ..label_style
                        }),
                    )
                }
                (Orientation::Vertical, TickStyle::Marker { style }) => (
                    Some((
                        [
                            vec2(pos.x + style.length / 2., pos.y),
                            vec2(pos.x - style.length / 2., pos.y),
                        ],
                        style.thickness,
                        style.color,
                    )),
                    None,
                ),
                (Orientation::Vertical, TickStyle::Label { style }) => {
                    let label_pos = pos + style.pos_offset + vert_offset;
                    (
                        None,
                        Some(LabelStyle {
                            pos_offset: label_pos,
                            ..style
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
                map(
                    pt.x,
                    self.x_range.start,
                    self.x_range.end,
                    self.world_min_coords.x,
                    self.world_max_coords.x,
                ),
                map(
                    pt.y,
                    self.y_range.start,
                    self.y_range.end,
                    self.world_min_coords.y,
                    self.world_max_coords.y,
                ),
            )
        }
        pub fn plot_line_vec(&self, pts: &[Vec2], thickness: f32, color: Color) {
            pts.windows(2).for_each(|slice| {
                let pt_a = self.graph_to_world(slice[0]);
                let pt_b = self.graph_to_world(slice[1]);
                self.plot_line_world(&pt_a, &pt_b, thickness, color);
            });
        }

        pub fn plot_line_xy(&self, x: &[f32], y: &[f32], thickness: f32, color: Color) {
            x.windows(2).zip(y.windows(2)).for_each(|(xs, ys)| {
                let pt_a = self.graph_to_world(vec2(xs[0], ys[0]));
                let pt_b = self.graph_to_world(vec2(xs[1], ys[1]));

                self.plot_line_world(&pt_a, &pt_b, thickness, color);
            })
        }
        pub fn plot_pt_vec(&self, pt: &Vec2, radius: f32, color: Color) {
            let pt = self.graph_to_world(*pt);
            if !self.world_pt_in_world_bb(&pt) {
                return;
            }

            draw_circle(pt.x, pt.y, radius, color);
        }

        pub fn plot_pt_xy(&self, x: f32, y: f32, radius: f32, color: Color) {
            let pt = self.graph_to_world(vec2(x, y));
            if !self.world_pt_in_world_bb(&pt) {
                return;
            }

            draw_circle(pt.x, pt.y, radius, color);
        }
        fn plot_line_world(&self, pt_a: &Vec2, pt_b: &Vec2, thickness: f32, color: Color) {
            if !self.world_pt_in_world_bb(pt_a) && !self.world_pt_in_world_bb(pt_b) {
                // Neither point is on the graph so bail this iteration
                return;
            }
            // TODO: if a is in the graph but b isn't then clamp b at the nearest intersection point with
            // the nearest border. Vice versa for b. If both are off then don't draw anything

            draw_line(pt_a.x, pt_a.y, pt_b.x, pt_b.y, thickness, color);
        }
        fn world_pt_in_world_bb(&self, pt: &Vec2) -> bool {
            pt.x >= self.world_min_coords.x
                && pt.y >= self.world_min_coords.y
                && pt.x <= self.world_max_coords.x
                && pt.y <= self.world_max_coords.y
        }
    }
}

const FXAA_VERTEX_SHADER: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying lowp vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}"#;
const FXAA_FRAGMENT_SHADER: &str = r#"
#version 100
precision lowp float;

// THIS CODE IS THANKS TO: 
// https://blog.simonrodriguez.fr/articles/2016/07/implementing_fxaa.html

varying vec2 uv;

// UNIFORMS
uniform sampler2D Texture;
uniform vec2 texture_size;

// CONSTANTS
const float EDGE_THRESHOLD_MIN = 0.0312;
const float EDGE_THRESHOLD_MAX = 0.125;
const int ITERATIONS = 12;
const float SUBPIXEL_QUALITY = 0.75;

float rgb2luma(vec3 rgb){
    return sqrt(dot(rgb, vec3(0.299, 0.587, 0.114)));
}

float get_quality(int i) {
    float quality = 0.0;
    if(i < 6) {
       quality = 1.0; 
    } else if(i == 6) {
       quality = 1.5; 
    } else if(i > 6 && i < 10) {
       quality = 2.0; 
    } else if(i == 10) {
       quality = 4.0; 
    } else {
        quality = 8.0;
    }

    return quality;
}

void main() {
    vec2 inverseScreenSize = vec2(1.0/texture_size.x, 1.0/texture_size.y);
    vec3 colorCenter = texture2D(Texture,uv).rgb;

    // Luma at the current fragment
    float lumaCenter = rgb2luma(colorCenter);

    // Directions
    vec2 up = vec2(0., -inverseScreenSize.y);
    vec2 down = vec2(0., inverseScreenSize.y);
    vec2 left = vec2(-inverseScreenSize.x, 0.);
    vec2 right = vec2(inverseScreenSize.x, 0.);

    // Luma at the four direct neighbours of the current fragment.
    float lumaDown = rgb2luma(texture2D(Texture, uv + down).rgb);
    float lumaUp = rgb2luma(texture2D(Texture,uv + up).rgb);
    float lumaLeft = rgb2luma(texture2D(Texture,uv + left).rgb);
    float lumaRight = rgb2luma(texture2D(Texture,uv + right).rgb);

    // Find the maximum and minimum luma around the current fragment.
    float lumaMin = min(lumaCenter,min(min(lumaDown,lumaUp),min(lumaLeft,lumaRight)));
    float lumaMax = max(lumaCenter,max(max(lumaDown,lumaUp),max(lumaLeft,lumaRight)));

    // Compute the delta.
    float lumaRange = lumaMax - lumaMin;

    // If the luma variation is lower that a threshold (or if we are in a really dark area), we are not on an edge, don't perform any AA.
    if(lumaRange < max(EDGE_THRESHOLD_MIN,lumaMax*EDGE_THRESHOLD_MAX)){
        gl_FragColor = vec4(colorCenter, 1.0);
        return;
    }

    // Query the 4 remaining corners lumas.
    float lumaDownLeft = rgb2luma(texture2D(Texture,uv + down + left).rgb);
    float lumaUpRight = rgb2luma(texture2D(Texture,uv + up + right).rgb);
    float lumaUpLeft = rgb2luma(texture2D(Texture,uv + up + left).rgb);
    float lumaDownRight = rgb2luma(texture2D(Texture,uv + down + right).rgb);

    // Combine the four edges lumas (using intermediary variables for future computations with the same values).
    float lumaDownUp = lumaDown + lumaUp;
    float lumaLeftRight = lumaLeft + lumaRight;

    // Same for corners
    float lumaLeftCorners = lumaDownLeft + lumaUpLeft;
    float lumaDownCorners = lumaDownLeft + lumaDownRight;
    float lumaRightCorners = lumaDownRight + lumaUpRight;
    float lumaUpCorners = lumaUpRight + lumaUpLeft;

    // Compute an estimation of the gradient along the horizontal and vertical axis.
    float edgeHorizontal =  abs(-2.0 * lumaLeft + lumaLeftCorners)  + abs(-2.0 * lumaCenter + lumaDownUp ) * 2.0    + abs(-2.0 * lumaRight + lumaRightCorners);
    float edgeVertical =    abs(-2.0 * lumaUp + lumaUpCorners)      + abs(-2.0 * lumaCenter + lumaLeftRight) * 2.0  + abs(-2.0 * lumaDown + lumaDownCorners);

    // Is the local edge horizontal or vertical ?
    bool isHorizontal = (edgeHorizontal >= edgeVertical);

    // Select the two neighboring texels lumas in the opposite direction to the local edge.
    float luma1 = isHorizontal ? lumaDown : lumaLeft;
    float luma2 = isHorizontal ? lumaUp : lumaRight;
    // Compute gradients in this direction.
    float gradient1 = luma1 - lumaCenter;
    float gradient2 = luma2 - lumaCenter;

    // Which direction is the steepest ?
    bool is1Steepest = abs(gradient1) >= abs(gradient2);

    // Gradient in the corresponding direction, normalized.
    float gradientScaled = 0.25*max(abs(gradient1),abs(gradient2));

    // Choose the step size (one pixel) according to the edge direction.
    float stepLength = isHorizontal ? inverseScreenSize.y : inverseScreenSize.x;

    // Average luma in the correct direction.
    float lumaLocalAverage = 0.0;

    if(is1Steepest){
        // Switch the direction
        stepLength = - stepLength;
        lumaLocalAverage = 0.5*(luma1 + lumaCenter);
    } else {
        lumaLocalAverage = 0.5*(luma2 + lumaCenter);
    }

    // Shift UV in the correct direction by half a pixel.
    vec2 currentUv = uv;
    if(isHorizontal){
        currentUv.y += stepLength * 0.5;
    } else {
        currentUv.x += stepLength * 0.5;
    }

    // Compute offset (for each iteration step) in the right direction.
    vec2 offset = isHorizontal ? vec2(inverseScreenSize.x,0.0) : vec2(0.0,inverseScreenSize.y);
    // Compute UVs to explore on each side of the edge, orthogonally. The QUALITY allows us to step faster.
    vec2 uv1 = currentUv - offset;
    vec2 uv2 = currentUv + offset;

    // Read the lumas at both current extremities of the exploration segment, and compute the delta wrt to the local average luma.
    float lumaEnd1 = rgb2luma(texture2D(Texture,uv1).rgb);
    float lumaEnd2 = rgb2luma(texture2D(Texture,uv2).rgb);
    lumaEnd1 -= lumaLocalAverage;
    lumaEnd2 -= lumaLocalAverage;

    // If the luma deltas at the current extremities are larger than the local gradient, we have reached the side of the edge.
    bool reached1 = abs(lumaEnd1) >= gradientScaled;
    bool reached2 = abs(lumaEnd2) >= gradientScaled;
    bool reachedBoth = reached1 && reached2;

    // If the side is not reached, we continue to explore in this direction.
    if(!reached1){
        uv1 -= offset;
    }
    if(!reached2){
        uv2 += offset;
    }

    // If both sides have not been reached, continue to explore.
    if(!reachedBoth){

        for(int i = 2; i < ITERATIONS; i++){
            // If needed, read luma in 1st direction, compute delta.
            if(!reached1){
                lumaEnd1 = rgb2luma(texture2D(Texture, uv1).rgb);
                lumaEnd1 = lumaEnd1 - lumaLocalAverage;
            }
            // If needed, read luma in opposite direction, compute delta.
            if(!reached2){
                lumaEnd2 = rgb2luma(texture2D(Texture, uv2).rgb);
                lumaEnd2 = lumaEnd2 - lumaLocalAverage;
            }
            // If the luma deltas at the current extremities is larger than the local gradient, we have reached the side of the edge.
            reached1 = abs(lumaEnd1) >= gradientScaled;
            reached2 = abs(lumaEnd2) >= gradientScaled;
            reachedBoth = reached1 && reached2;

            // If the side is not reached, we continue to explore in this direction, with a variable quality.
            if(!reached1){
                uv1 -= offset * get_quality(i);
            }
            if(!reached2){
                uv2 += offset * get_quality(i);
            }

            // If both sides have been reached, stop the exploration.
            if(reachedBoth){ break;}
        }
    }

    // Compute the distances to each extremity of the edge.
    float distance1 = isHorizontal ? (uv.x - uv1.x) : (uv.y - uv1.y);
    float distance2 = isHorizontal ? (uv2.x - uv.x) : (uv2.y - uv.y);

    // which direction is the extremity of the edge closer ?
    bool isDirection1 = distance1 < distance2;
    float distanceFinal = min(distance1, distance2);

    // Length of the edge.
    float edgeThickness = (distance1 + distance2);

    // UV offset: read in the direction of the closest side of the edge.
    float pixelOffset = - distanceFinal / edgeThickness + 0.5;

    // Is the luma at center smaller than the local average ?
    bool isLumaCenterSmaller = lumaCenter < lumaLocalAverage;

    // If the luma at center is smaller than at its neighbour, the delta luma at each end should be positive (same variation).
    // (in the direction of the closer side of the edge.)
    bool correctVariation = ((isDirection1 ? lumaEnd1 : lumaEnd2) < 0.0) != isLumaCenterSmaller;

    // If the luma variation is incorrect, do not offset.
    float finalOffset = correctVariation ? pixelOffset : 0.0;

    // Sub-pixel shifting
    // Full weighted average of the luma over the 3x3 neighborhood.
    float lumaAverage = (1.0/12.0) * (2.0 * (lumaDownUp + lumaLeftRight) + lumaLeftCorners + lumaRightCorners);
    // Ratio of the delta between the global average and the center luma, over the luma range in the 3x3 neighborhood.
    float subPixelOffset1 = clamp(abs(lumaAverage - lumaCenter)/lumaRange,0.0,1.0);
    float subPixelOffset2 = (-2.0 * subPixelOffset1 + 3.0) * subPixelOffset1 * subPixelOffset1;
    // Compute a sub-pixel offset based on this delta.
    float subPixelOffsetFinal = subPixelOffset2 * subPixelOffset2 * SUBPIXEL_QUALITY;

    // Pick the biggest of the two offsets.
    finalOffset = max(finalOffset,subPixelOffsetFinal);

    // Compute the final UV coordinates.
    vec2 finalUv = uv;
    if(isHorizontal){
        finalUv.y += finalOffset * stepLength;
    } else {
        finalUv.x += finalOffset * stepLength;
    }

    // Read the color at the new UV coordinates, and use it.
    vec3 finalColor = texture2D(Texture,finalUv).rgb;

    gl_FragColor = vec4(finalColor, 1.0);
}"#;
