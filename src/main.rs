use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("image.jpg"),
            ..Default::default()
        })
        .insert(Player);
}

fn move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Transform, &mut Sprite), With<Player>>,
) {
    let (mut player, mut sprite) = players.iter_mut().next().unwrap();
    if keyboard_input.pressed(KeyCode::A) {
        sprite.flip_x = false;
        player.translation.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        sprite.flip_x = true;
        player.translation.x += 1.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_system)
        .run();
}
