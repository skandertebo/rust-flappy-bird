use nannou::prelude::*;

pub struct Pipe {
    pub window_height: f32,
    pub window_width: f32,
    pub x: f32,
    pub gap_center_y: f32,
    pub gap: f32,
    pub width: f32
}

impl Pipe {
    const velocity: f32 = -60.0;

    pub fn new(gap_center_y: f32, gap: f32, window_height: f32, window_width: f32, width:f32) -> Pipe {
        Pipe {
            width,
            window_height,
            window_width,
            x: window_width / 2.0,
            gap_center_y,
            gap,
        }
    }
    pub fn draw(&self, draw: &Draw) {
        let upper_pipe_length = (self.window_height / 2.0) - (self.gap_center_y + self.gap / 2.0);
        let lower_pipe_length = (-self.window_height / 2.0) - (self.gap_center_y - self.gap / 2.0);

        draw.rect()
            .x_y(self.x, self.window_height / 2.0 - upper_pipe_length / 2.0)
            .w_h(self.width, upper_pipe_length)
            .color(GREEN);
        draw.rect()
            .x_y(self.x, -self.window_height / 2.0 - lower_pipe_length / 2.0)
            .w_h(self.width, -lower_pipe_length)
            .color(GREEN);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.x += Pipe::velocity * delta_time;
    }

    pub fn get_x(&self) -> f32 {
        return self.x;
    }

    pub fn get_gap_center_y(&self) -> f32 {
        return self.gap_center_y;
    }

    pub fn get_gap(&self) -> f32 {
        return self.gap;
    }

    pub fn get_width(&self) -> f32 {
        return 50.0;
    }

    pub fn get_height(&self) -> f32 {
        return self.window_height;
    }
    pub fn is_out_of_bounds(&self) -> bool {
        return self.x < -self.window_width / 2.0 - self.width / 2.0;
    }
}
