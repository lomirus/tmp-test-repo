use bevy::prelude::*;

#[derive(Component)]
struct Person {
    name: String,
    job: String,
    age: u8,
}

fn print_people(query: Query<&Person>) {
    for person in query.iter() {
        println!("{}：{} 歳、{}です", person.name, person.age, person.job);
    }
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert(Person {
        name: "野獣先輩".to_string(),
        job: "学生".to_string(),
        age: 24,
    });
    println!("Greet from Lomirus!");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("image.jpg"),
        ..Default::default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn().insert_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            margin: Rect::all(Val::Auto),
            ..Default::default()
        },
        text: Text::with_section(
            "Hello Bevy!",
            TextStyle {
                font: asset_server.load("NotoSansMono-Regular.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_startup_system(print_people)
        .run();
}
