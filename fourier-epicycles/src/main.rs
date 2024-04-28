use std::{f32::consts::PI, sync::Arc};

use macroquad::prelude::*;
use mqanim::Animation;
use rustfft::{
    num_complex::{Complex, Complex32},
    Fft, FftPlanner,
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

#[derive(Debug)]
struct FourierInfo {
    mag: f32,
    phase: f32,
    freq: f32,
}

const MAX_NUM_POINTS: usize = 1000;
const TWO_PI: f32 = PI * 2.;

enum State {
    BuiltInAnimation,
    DrawingPicture,
    DrawingManualAnimation,
    Paused,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut animation = Animation::new(WINDOW_WIDTH, WINDOW_HEIGHT, None);
    animation.enable_fxaa();

    let pts = generate_points();
    let pts: Vec<Complex32> = pts.into_iter().map(|pt| Complex::new(pt.x, pt.y)).collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(pts.len());

    let mut fft_buffer = pts.clone();
    fft.process(&mut fft_buffer);
    let mut fft_buffer: Vec<FourierInfo> = fft_buffer
        .into_iter()
        .enumerate()
        .map(|(idx, pt)| {
            let (mag, phase) = pt.to_polar();
            let mag = mag / MAX_NUM_POINTS as f32;
            let freq = idx as f32;
            FourierInfo { mag, phase, freq }
        })
        .collect();

    fft_buffer.sort_unstable_by(|a, b| a.mag.total_cmp(&b.mag)); // Sort by the strongest
                                                                 // magnitudes
    fft_buffer.reverse();

    let mut path = Vec::with_capacity(MAX_NUM_POINTS);
    let mut time = 0.;
    let mut frame_count = 0;

    let mut state = State::BuiltInAnimation;
    loop {
        animation.set_camera();

        pts.windows(2).for_each(|slice| {
            let a = slice[0];
            let b = slice[1];
            draw_line(a.re, a.im, b.re, b.im, 2., WHITE)
        });

        let next_path_pt = draw_epicycles(&fft_buffer, time, 3);
        path.push(next_path_pt);

        // Draw the path
        path.windows(2).for_each(|slice| {
            let pt_a = slice[0];
            let pt_b = slice[1];
            draw_line(pt_a.x, pt_a.y, pt_b.x, pt_b.y, 2., PURPLE);
        });

        time += TWO_PI / fft_buffer.len() as f32; // The split of the frequency bins
        frame_count += 1;
        if frame_count == fft_buffer.len() {
            frame_count = 0;
            time = 0.;
            path.clear();
        }
        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
    }
}

fn generate_points() -> Vec<Vec2> {
    let mut result = Vec::with_capacity(MAX_NUM_POINTS);
    for i in 0..MAX_NUM_POINTS {
        let theta = mqanim::map(
            i as f32,
            0.,
            (MAX_NUM_POINTS - 1) as f32,
            0.,
            2. * std::f32::consts::PI,
        );
        let r = 50. * f32::sin(5. * theta) + 100.;
        result.push(vec2(r * f32::cos(theta), r * f32::sin(theta)));
    }

    result
}

fn draw_epicycles(fft_info: &[FourierInfo], time: f32, depth: usize) -> Vec2 {
    let mut start_pt = vec2(0., 0.);
    fft_info.iter().take(depth).for_each(|fi| {
        let x = start_pt.x + fi.mag * f32::cos(fi.freq * time + fi.phase);
        let y = start_pt.y + fi.mag * f32::sin(fi.freq * time + fi.phase);

        draw_line(start_pt.x, start_pt.y, x, y, 1., ORANGE);
        start_pt = vec2(x, y)
    });

    start_pt
}

fn compute_fft(
    fft_calculator: Arc<dyn Fft<f32>>,
    pts: &[Complex32],
    fft_buffer: &mut Vec<Complex32>,
    fi_buffer: &mut Vec<FourierInfo>,
) {
}
