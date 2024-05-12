use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
};

#[derive(Component,Debug)]
struct Floating {
    start_loc: Vec2,
    velocity: Vec2,
    progress: f32,
}

#[derive(Component, Debug)]
struct Details {
    name: String,
    id: u64,
}

#[derive(Resource, Default)]
struct WorldTime {
    ps: u64
}

fn load_data(mut commands: Commands) {
    println!("Loading ..");
    commands.spawn(
        (
            Details { name: "foobar".to_string(), id: 0 },
            Floating { start_loc: Vec2::new(1.0,1.0), velocity: Vec2::new(1.0,0.0), progress: 0.0},
        )
    );

}

fn update_location(mut commands: Commands, 
                   world_time: Res<WorldTime>, 
                   mut query: Query<(Entity, &Details, &mut Floating)>
                   ) {
    for (entity,details, mut fl) in &mut query {
        println!("{details:?}, {fl:?}");
        fl.progress = world_time.ps as f32 / 100.0f32;
        if fl.progress >= 1.0 {
            println!("{details:?} is done");
            commands.entity(entity).remove::<Floating>();
        } else {
            let cur_pos = fl.start_loc + (fl.velocity * fl.progress);
            println!("Updated cur_pos={cur_pos}");
        }
    }
}

fn tick_world_time(mut _commands: Commands, mut world_time: ResMut<WorldTime>) {
    world_time.ps+=1;
    println!("World time is {}", world_time.ps);
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WorldTime>()
        .add_systems(Startup, load_data)
        .add_systems(Update, (tick_world_time,update_location).chain())
        .run();
    println!("Done!");
}
