use std::f32::consts::PI;

use macroquad::prelude::*;
use mqanim::{
    ui::{draw_text_centered, Button, ButtonShape},
    Animation,
};
use rustfft::{num_complex::Complex32, FftPlanner};

pub mod henry;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 360.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Fourier Epicycles".to_owned(),
        sample_count: 4,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[derive(Debug, Copy, Clone)]
struct FourierInfo {
    mag: f32,
    phase: f32,
    freq: f32,
}

const MAX_NUM_POINTS: usize = 600;
const TWO_PI: f32 = PI * 2.;

const BUTTON_WIDTH: f32 = 67.;
const BUTTON_HEIGHT: f32 = 42.;

const HENRY_VERT_SHIFT: f32 = 20.;

#[derive(Debug, Copy, Clone)]
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

    // UI
    let mut is_paused = false;
    let mut draw_manual = false;
    let button_y = 125.;
    let mut number_of_epicycles = 120.;

    let mut pts = Vec::with_capacity(MAX_NUM_POINTS);
    generate_points(&mut pts);

    let mut fft_buffer = vec![Complex32::new(0., 0.); pts.len()];
    let mut fi_buffer = vec![
        FourierInfo {
            mag: 0.,
            phase: 0.,
            freq: 0.
        };
        pts.len()
    ];

    compute_fft(&pts, &mut fft_buffer, &mut fi_buffer);

    let mut epicycle_path = Vec::with_capacity(MAX_NUM_POINTS);
    let mut time = 0.;
    let mut frame_count = 0;

    // NOTE: This code is a little bit of a mess. Definitely could do with a proper Finite State
    // Machine instead of this little thing I cobbled together...
    let mut prev_state = State::BuiltInAnimation;
    let mut state = State::BuiltInAnimation;
    loop {
        animation.set_camera();

        // UI
        let mouse_pos = animation.get_world_mouse();
        draw_toggle(
            "Pause",
            vec2(250., 150.),
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            mouse_pos,
            &mut is_paused,
        );
        if !matches!(state, State::Paused) {
            draw_toggle(
                "Draw",
                vec2(-250., 150.),
                BUTTON_WIDTH,
                BUTTON_HEIGHT,
                mouse_pos,
                &mut draw_manual,
            );

            draw_text_centered(
                &format!(
                    "Number of Epicycles: {}",
                    f32::floor(number_of_epicycles) as usize
                ),
                0.,
                button_y + 40.,
                25,
                WHITE,
            );
            let prev_number_of_epicycles = f32::floor(number_of_epicycles) as usize;
            mqanim::ui::Slider::new(vec2(0., button_y), vec2(350., 25.), 1.0..200.0)
                .style(mqanim::ui::SliderStyle {
                    bar_height: 12.,
                    ..Default::default()
                })
                .mouse_pos(mouse_pos)
                .draw(&mut number_of_epicycles);
            if prev_number_of_epicycles != f32::floor(number_of_epicycles) as usize {
                reset_time(&mut time, &mut frame_count, &mut epicycle_path);
            }
        }

        // Draw current picture
        pts.windows(2).for_each(|slice| {
            let a = slice[0];
            let b = slice[1];
            draw_line(
                a.re,
                a.im - HENRY_VERT_SHIFT,
                b.re,
                b.im - HENRY_VERT_SHIFT,
                3.,
                WHITE,
            );
        });
        let last_pt = pts.last();
        let first_pt = pts.first();

        // Connect the polygon at each end
        if let (Some(fp), Some(lp)) = (first_pt, last_pt) {
            draw_line(
                fp.re,
                fp.im - HENRY_VERT_SHIFT,
                lp.re,
                lp.im - HENRY_VERT_SHIFT,
                3.,
                WHITE,
            );
        }

        // Transitions
        match (prev_state, state) {
            (State::BuiltInAnimation, State::BuiltInAnimation)
            | (State::DrawingManualAnimation, State::DrawingManualAnimation) => {
                prev_state = state;
            }
            (State::BuiltInAnimation, State::DrawingManualAnimation) => {
                // Clear all the things. The user is going to fill the buffer with a drawing
                prev_state = state;
                clear_all_buffers(
                    &mut pts,
                    &mut fft_buffer,
                    &mut fi_buffer,
                    &mut epicycle_path,
                );
            }
            (State::DrawingManualAnimation, State::BuiltInAnimation) => {
                prev_state = state;

                clear_all_buffers(
                    &mut pts,
                    &mut fft_buffer,
                    &mut fi_buffer,
                    &mut epicycle_path,
                );
                reset_time(&mut time, &mut frame_count, &mut epicycle_path);
                generate_points(&mut pts);
                compute_fft(&pts, &mut fft_buffer, &mut fi_buffer);
            }
            (State::DrawingManualAnimation, State::DrawingPicture) => {
                prev_state = state;

                clear_all_buffers(
                    &mut pts,
                    &mut fft_buffer,
                    &mut fi_buffer,
                    &mut epicycle_path,
                );
                reset_time(&mut time, &mut frame_count, &mut epicycle_path);
            }
            (State::DrawingPicture, State::DrawingManualAnimation) => {
                prev_state = state;
                reset_time(&mut time, &mut frame_count, &mut epicycle_path);
                compute_fft(&pts, &mut fft_buffer, &mut fi_buffer);
            }
            (State::DrawingPicture, State::DrawingPicture) => {}
            (State::DrawingPicture, State::BuiltInAnimation) => {
                unreachable!("When a drawing is finished it never goes back to built in")
            }
            (State::BuiltInAnimation, State::DrawingPicture) => unreachable!(
                "There should not be a way to transition from BuiltInAnimation to DrawingPicture"
            ),
            (State::Paused, _) => {
                unreachable!("There should be no time that the prev state is paused")
            }
            (_, State::Paused) => {}
        };

        let next_path_pt =
            draw_epicycles(&fi_buffer, time, f32::floor(number_of_epicycles) as usize);
        // Actions
        match state {
            State::BuiltInAnimation => {}
            State::DrawingManualAnimation => {
                if mouse_pos.y < button_y - BUTTON_HEIGHT / 2.
                    && is_mouse_button_down(MouseButton::Left)
                {
                    state = State::DrawingPicture;
                }
            }
            State::DrawingPicture => {
                if !is_mouse_button_down(MouseButton::Left) || pts.len() == MAX_NUM_POINTS {
                    state = State::DrawingManualAnimation;
                }

                let mut can_append_pt = true;
                let last_pt = pts.last();
                if let Some(last_pt) = last_pt {
                    let last_pt = vec2(last_pt.re, last_pt.im);

                    if last_pt.distance_squared(mouse_pos) < 1. {
                        // Make sure we aren't constantly just pushing the same point if the mouse
                        // doesn't move
                        can_append_pt = false;
                    }
                }
                if can_append_pt {
                    pts.push(Complex32::new(mouse_pos.x, mouse_pos.y));
                }
            }
            State::Paused => {
                if !is_paused {
                    // Escape the pause
                    state = prev_state;
                }
                draw_text_centered("PAUSED", 0., 0., 100, BLACK)
            }
        };

        if matches!(state, State::BuiltInAnimation)
            || matches!(state, State::DrawingManualAnimation)
        {
            if draw_manual {
                state = State::DrawingManualAnimation;
            } else {
                state = State::BuiltInAnimation;
            }
            if !pts.is_empty() {
                epicycle_path.push(next_path_pt);
            }
            advance_time(
                &mut time,
                &mut frame_count,
                fft_buffer.len(),
                &mut epicycle_path,
            );
        }

        // Draw the path
        epicycle_path.windows(2).for_each(|slice| {
            let pt_a = slice[0];
            let pt_b = slice[1];
            draw_line(pt_a.x, pt_a.y, pt_b.x, pt_b.y, 4., PURPLE);
        });

        if is_paused {
            // Always check paused, seeing as you can pause from any state
            state = State::Paused;
        }

        animation.set_default_camera();
        animation.draw_frame();

        next_frame().await;
        // println!(
        //     "Capacities: {}, {}, {}, {}",
        //     fft_buffer.capacity(),
        //     epicycle_path.capacity(),
        //     pts.capacity(),
        //     fi_buffer.capacity(),
        // );
        // println!(
        //     "Lens: {}, {}, {}, {}",
        //     fft_buffer.len(),
        //     epicycle_path.len(),
        //     pts.len(),
        //     fi_buffer.len(),
        // );
    }
}

fn advance_time(
    time: &mut f32,
    frame_count: &mut usize,
    num_frequency_bins: usize,
    epicycle_path: &mut Vec<Vec2>,
) {
    *time += TWO_PI / num_frequency_bins as f32; // The split of the frequency bins
    *frame_count += 1;
    if *frame_count == num_frequency_bins {
        reset_time(time, frame_count, epicycle_path);
    }
}

fn reset_time(time: &mut f32, frame_count: &mut usize, epicycle_path: &mut Vec<Vec2>) {
    *frame_count = 0;
    *time = 0.;
    epicycle_path.clear();
}

fn generate_points(buff: &mut Vec<Complex32>) {
    buff.clear();
    let offset = 40.;
    for pt in henry::HENRY {
        let x = mqanim::map(
            pt.x,
            -1.,
            1.,
            (-WINDOW_WIDTH / 2.) + offset,
            (WINDOW_WIDTH / 2.) - offset,
        );
        let y = mqanim::map(
            pt.y,
            -1.,
            1.,
            (-WINDOW_HEIGHT / 2.) + offset,
            (WINDOW_HEIGHT / 2.) - offset,
        );
        let complex_pt = Complex32::new(x, y);
        // println!("{}", complex_pt);
        buff.push(complex_pt);
    }
}

fn draw_epicycles(fft_info: &[FourierInfo], time: f32, depth: usize) -> Vec2 {
    let mut start_pt = vec2(0., -HENRY_VERT_SHIFT);
    fft_info.iter().take(depth).for_each(|fi| {
        let x = start_pt.x + fi.mag * f32::cos(fi.freq * time + fi.phase);
        let y = start_pt.y + fi.mag * f32::sin(fi.freq * time + fi.phase);

        draw_line(start_pt.x, start_pt.y, x, y, 2., ORANGE);
        start_pt = vec2(x, y)
    });

    start_pt
}

fn compute_fft(
    pts: &[Complex32],
    fft_buffer: &mut Vec<Complex32>,
    fi_buffer: &mut Vec<FourierInfo>,
) {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(pts.len());

    // Reset the fft buffer for re computing the fft
    fft_buffer.clear();
    fi_buffer.clear();
    pts.iter().for_each(|pt| fft_buffer.push(*pt));

    fft.process(fft_buffer);
    fft_buffer.iter().enumerate().for_each(|(idx, fft)| {
        let (mag, phase) = fft.to_polar();
        let mag = mag / pts.len() as f32;
        let freq = idx as f32;
        fi_buffer.push(FourierInfo { mag, phase, freq });
    });

    fi_buffer.sort_unstable_by(|a, b| a.mag.total_cmp(&b.mag)); // Sort by the strongest
                                                                // magnitudes
    fi_buffer.reverse();
}
fn draw_toggle(label: &str, pos: Vec2, width: f32, height: f32, mouse_pos: Vec2, data: &mut bool) {
    Button::new(pos, ButtonShape::Rectangle { width, height })
        .mouse_pos(mouse_pos)
        .draw(data);
    draw_text_centered(label, pos.x, pos.y, 20, WHITE);
}

fn clear_all_buffers(
    pts: &mut Vec<Complex32>,
    fft_buffer: &mut Vec<Complex32>,
    fi_buffer: &mut Vec<FourierInfo>,
    epicycle_path: &mut Vec<Vec2>,
) {
    fft_buffer.clear();
    epicycle_path.clear();
    pts.clear();
    fi_buffer.clear();
}
