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
