use bevy::prelude::*;

fn main() {
    let window = Window {
        title: "Gizmo".into(),
        resolution: (200., 200.).into(),
        ..default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                }),
        ) // prevents blurry sprites
        .add_systems(PreStartup, configure_gizmos)
        .add_systems(Startup, setup)
        .add_systems(Update, update_gizmos)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .run();
}

fn configure_gizmos(mut config: ResMut<GizmoConfig>) {
    config.depth_bias = -1.;
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture: Handle<Image> = assets.load("sprite.png");
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..Default::default()
    });
}

fn update_gizmos(mut gizmos: Gizmos) {
    gizmos.rect_2d(Vec2::splat(0.), 0., Vec2::splat(50.), Color::WHITE);
}
