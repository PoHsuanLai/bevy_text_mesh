use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mesh::TextMeshPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TextMeshPlugin) // TextMeshPlugin for interop check
        .add_systems(Startup, setup)
        .add_systems(Update, animate_rotation)
        .run();
}

#[derive(Component)]
struct AnimateRotation;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("standard 2d text works too"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Anchor::CENTER,
        Transform::default(),
        AnimateRotation,
    ));
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_secs_f64().cos() as f32);
    }
}
