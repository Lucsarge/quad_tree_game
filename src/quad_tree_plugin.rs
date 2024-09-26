use bevy::{
    prelude::*,
    app::PluginGroupBuilder,
};

#[derive(Component)]
struct Quad{
    x: f32,
    y: f32,
    center: (f32, f32)
}

pub struct QuadTreePlugin;

impl Plugin for QuadTreePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, intialize_quad_tree);
    }
}

fn intialize_quad_tree(){
    println!("initializing quad tree");
}

// fn subdivide(){
//     println!("Subdividing")
// }
