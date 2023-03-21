mod entity;
use entity::Entity;
use nannou::prelude::*;

use crate::pipe_builder;

pub struct Bird {
    parent: Entity,
    velocity: f32,
    texture: wgpu::Texture,
    applied_force: f32,
    mass: f32,
    height: f32,
    width: f32,
    window_height: f32,
    window_width: f32,
}

impl Bird {
    pub fn new(x: f32, y: f32, mass: f32, texture: wgpu::Texture, window_height:f32, window_width:f32) -> Bird {
        Bird {
            parent: Entity::new(x, y),
            velocity: 0.0,
            applied_force: 0.0,
            texture: texture,
            mass: mass,
            height: 30.0,
            width: 50.0,
            window_height,
            window_width
        }
    }

    pub fn get_acceleration(&self) -> f32 {
        return self.applied_force / self.mass;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.parent.y - (self.height / 2.0) > -self.window_height / 2.0{
            self.velocity += self.get_acceleration() * delta_time;
            self.parent.y += self.velocity * delta_time;
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.texture(&self.texture)
            .x_y(self.parent.x, self.parent.y)
            .w_h(self.width, self.height);
    }

    pub fn apply_force(&mut self, force: f32) {
        self.applied_force += force;
    }

    pub fn set_force(&mut self, force: f32) {
        self.applied_force = force;
    }

    pub fn reset_velocity(&mut self) {
        self.velocity = 0.0;
    }

    pub fn detect_collision(&self, pipes: &Vec<pipe_builder::Pipe>) -> bool {
        if self.parent.y - (self.height / 2.0) < -self.window_height / 2.0
        {
            return true;
        }
        for pipe in pipes {
            if self.parent.x + self.width / 2.0 > pipe.get_x() - pipe.get_width() / 2.0
                && self.parent.x - self.width / 2.0 < pipe.get_x() + pipe.get_width() / 2.0
            {
                if self.parent.y + self.height / 2.0
                    > pipe.get_gap_center_y() + pipe.get_gap() / 2.0
                    || self.parent.y - self.height / 2.0
                        < pipe.get_gap_center_y() - pipe.get_gap() / 2.0
                {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn reset(&mut self){
        self.parent.x = 0.0;
        self.parent.y = 0.0;
        self.reset_velocity();
        self.applied_force = 0.0;
    }

    pub fn get_texture(&self) -> &wgpu::Texture {
        return &self.texture;
    }
}
