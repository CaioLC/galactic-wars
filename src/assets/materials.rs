use bevy::{
    prelude::{Color, Handle, Image, Material, Vec2},
    reflect::TypeUuid,
    render::render_resource::AsBindGroup,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "33d2536b-8b55-4e8e-9d47-462c338dfc08"]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub color: Color,

    #[texture(1)]
    #[sampler(2)]
    pub image: Handle<Image>,
}
impl Material for PlanetMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "my_material.wgsl".into()
    }
}
