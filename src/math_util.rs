use bevy::prelude::*;

/// Gets the heading given a delta vector separating two points.
/// 
/// # Arguments
/// 
/// * `delta`: difference vector equal to `target - source`.
pub fn get_heading_to_point(delta: Vec3) -> f32 {
    delta.y.atan2(delta.x)
}

pub fn world_to_local(point: Vec3, local_space: Transform) -> Vec3 {
    let first = point - local_space.translation;
    let x_component = first.dot(local_space.right());
    let y_component = first.dot(local_space.up());
    let z_component = first.dot(local_space.forward());
    Vec3::new(x_component, y_component, z_component)
}

/// Returns the smallest angle difference between two stated angles.
/// 
/// # Arguments
/// 
/// * `target`: target heading.
/// 
/// * `initial`: initial heading.
pub fn get_angle_difference(target: f32, initial: f32) -> f32 {
    // custom mod required - mod(a,n) = a - floor(a / n) * n
    let a = (target - initial) + std::f32::consts::PI;
    let n = 2.0 * std::f32::consts::PI;
    (a - (a / n).floor() * n) - std::f32::consts::PI
}