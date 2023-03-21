#[path ="pipe.rs"]
mod pipe;
use nannou::rand::rand;
pub use pipe::Pipe;

pub fn build(window_height: f32, window_width: f32) -> pipe::Pipe {
    let gap_center_upper_offset = window_height / 4.0;
    let gap_center_lower_offset = -window_height / 4.0;
    let gap_max = window_height / 4.0;
    let gap_min = window_height / 5.0;
    let gap_offset = gap_max - gap_min;
    let gap_center_y = (gap_center_upper_offset - gap_center_lower_offset * rand::random::<f32>())
        + gap_center_lower_offset;
    let gap = gap_offset * rand::random::<f32>() + gap_min;
    Pipe::new(gap_center_y, gap, window_height, window_width,50.0)
}
