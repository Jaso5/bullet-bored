use std::f32::consts::PI;

use macroquad::{prelude::{Vec2, Color, Rect}, shapes::draw_line};

pub fn angle_between_radians(a1: f32, a2: f32) -> f32 {
    ((a1 + a2) % (2.0 * PI)) / 2.0
}

pub fn middle_vec2(v1: Vec2, v2: Vec2) -> Vec2 {
    (v1 + v2).normalize_or_zero()
}

pub fn vec2_to_angle(vec: Vec2) -> f32 {
    // WHY DOES THIS NEED A NEGATIVE
    -vec.angle_between(Vec2::new(1.0, 0.0))
}

pub fn visualize_vec(pos: Vec2, vec: Vec2, color: Color) {
    draw_line(pos.x, pos.y, pos.x + vec.x, pos.y + vec.y, 1.5, color)
}

pub trait ClampToRect {
    fn clamp_inside_rect(&mut self, rect:Rect);
}

impl ClampToRect for Vec2 {
    fn clamp_inside_rect(&mut self, rect:Rect) {
        self.x = self.x.clamp(rect.left(), rect.right());
        self.y = self.y.clamp(rect.top(), rect.bottom());
    }
}