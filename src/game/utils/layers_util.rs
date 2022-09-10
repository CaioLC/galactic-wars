use bevy::math::{Vec2, Vec3};

pub enum Layers {
    Ships,
    Planets,
    BoxSelect,
    Text,
}

pub fn get_z(obj_type: Layers) -> f32 {
    match obj_type {
        Layers::Ships => 0.,
        Layers::Planets => 0.,
        Layers::BoxSelect => 3.,
        Layers::Text => 7.5,
    }
}

pub fn vec2_to_vec3(xy: Vec2, obj_type: Layers) -> Vec3 {
    Vec3::new(xy.x, xy.y, get_z(obj_type))
}

pub fn to_layer(vec: Vec3, obj_type: Layers) -> Vec3 {
    Vec3::new(vec.x, vec.y, get_z(obj_type))
}
