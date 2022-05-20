use bevy::math::{Vec2, Vec3};


pub enum Layers {
    Ships,
    Planets,
    BoxSelect,
}

pub fn get_z(obj_type: Layers) -> f32 {
    match obj_type {
        Layers::Ships => 0.,
        Layers::Planets => 1.,
        Layers::BoxSelect => 3.,
    }
}

pub fn vec2_to_vec3(xy: Vec2, obj_type: Layers) -> Vec3 {
    Vec3::new(xy.x, xy.y, get_z(obj_type))
}