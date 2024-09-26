use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

#[derive(Resource)]
pub struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

pub fn hello_world(){
    println!("Hello, world");
}

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Dan Primo".to_string())));
    commands.spawn((Person, Name("Stacy Segie".to_string())));
    commands.spawn((Person, Name("Alex Trace".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>){
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query{
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>){
    for mut name in &mut query {
        if name.0 == "Stacy Segie"{
            name.0 = "Stacy Inda-House".to_string();
            break;
        }
    }
}
