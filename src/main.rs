use bevy::{
    prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use rand::prelude::*;

// use render::camera;
// use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

// mod hello_module;
// mod quad_tree_plugin;

#[derive(Component)]
struct QuadTree{
    points: Vec<Point>,
    quads: Vec<Quad>,
    is_sub_divided: bool
}

impl QuadTree{
    fn new() -> QuadTree {
        return QuadTree{
            points: vec![Point::default(); 100],
            quads: vec![Quad::default(); 4],
            is_sub_divided: false
        }
    }
}

#[derive(Component)]
#[derive(Default)]
#[derive(Clone, Copy)]
struct Quad{
    center: Vec2,
    width: f32,
    height: f32
}

#[derive(Component)]
#[derive(Default)]
#[derive(Clone, Copy)]
struct Point{
    x: f32,
    y: f32
}

fn main(){
    let mut points: [Point; 100] = [Point::default(); 100];

    let tree = QuadTree::new();
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                resolution: (500.0, 500.0).into(),
                title: "Quad Tree".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        // .add_plugins(quad_tree_plugin::QuadTreePlugin)
        .add_systems(Startup, window_debug)
        .add_systems(Startup, setup_quad_tree)
        .add_systems(Startup, setup_points)
        .run();
}

fn window_debug(windows: Query<&Window>){
    let window = windows.single();
    println!("Window Res: {0} x {1}", window.width().to_string(), window.height().to_string());
}

fn setup_quad_tree() {
    println!("Creating quad tree");
}

fn setup_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn(Camera2dBundle::default());

    let circle: Mesh2dHandle = Mesh2dHandle(meshes.add(Circle {radius: 5.0}));
    let color = Color::hsl(1.0, 1.0, 1.0);

    for _ in 0..100{
        let mut rng = thread_rng();
        let rand_x = rng.gen_range(0.0..500.0) - 250.0;
        let rand_y = rand::random::<f32>() * 500.0 - 250.0;
        if (rand_x > 250.0 || rand_x < -250.0 || rand_y > 250.0 || rand_y < -250.0){
            println!("error: {0}, {1}", rand_x, rand_y);
        }
        else{
            println!("{}", format!("{rand_x} {rand_y}"));
        }

        commands.spawn(MaterialMesh2dBundle {
            mesh: circle.clone(),
            material: materials.add(color),
            transform: Transform::from_xyz(rand_x, rand_y, 0.0),..default()
        });
    }
}
