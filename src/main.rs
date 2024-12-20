use bevy::{
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_cursor, mouse_click_system))
        .run();
}

#[derive(Component, Default)]
struct CursorColor {
    color: Color,
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    windows: Single<&Window>,
    mut gizmos: Gizmos,
    color_query: Single<&CursorColor>,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane
    let Some(distance) = 
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(
        Isometry3d::new(
            point + ground.up() * 0.01,
            Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            color_query.color,
    );
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
            Ground,
    ));

    // light
    commands.spawn((
            DirectionalLight::default(),
            Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // camera
    commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ball
    commands.spawn((
            Mesh3d(meshes.add(Sphere::new(1.0).mesh().build())),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    commands.spawn(CursorColor::default());
}

fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut color_query: Query<&mut CursorColor>,
) {
    if let Some(mut cursor_color_state) = color_query.get_single_mut().ok() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            info!("left mouse just pressed");
            cursor_color_state.color = if cursor_color_state.color == Color::WHITE {
                Color::srgb(1.0, 0.3, 0.3)
            } else {
                Color::WHITE
            };
        } else {
            cursor_color_state.color = Color::WHITE
        }

    }
}
