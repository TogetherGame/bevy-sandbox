mod environment;

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::window::CursorGrabMode;
use environment::LevelPlugin;

#[derive(Component)]
pub(crate) struct ViewPoint;
#[derive(Component)]
pub(crate) struct FollowCam;

#[derive(Resource)]
pub(crate) struct CameraSensitivity {
    pub x: f32,
    pub y: f32,
}

impl CameraSensitivity {
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, LevelPlugin))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, camera_control)
        .run();
}

fn setup(
    mut cmd: Commands,
    mut windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let initial_vp_tr = Transform::from_translation(Vec3::ZERO);

    // Lock cursor inside of game window
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;

    cmd.spawn(NodeBundle {
        transform: initial_vp_tr,
        ..Default::default()  
    }).insert(ViewPoint);

    cmd.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 10.),
            ..Default::default()
        },
        FogSettings {
            color: Color::rgba(0.1, 0.2, 0.4, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.75, 0.5),
            directional_light_exponent: 20.0,
            falloff: FogFalloff::from_visibility_colors(
                15.0,
                Color::rgb(0.5, 0.6, 0.73),
                Color::rgb(0.8, 0.844, 1.0),
            ),
        },
    ));

    // DEBUG
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.))),
        material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
        transform: initial_vp_tr,
        ..default()
    });
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.))),
        material: materials.add(Color::rgb(0.1, 0.1, 0.8).into()),
        transform: Transform::from_translation(Vec3::new(0., 0., -6.)),
        ..default()
    })
    .insert(FollowCam);

    cmd.insert_resource(CameraSensitivity::from_xy(0.0001, 0.0001));
}

fn camera_control(
    mut cam_query: Query<&mut Transform, With<FollowCam>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_scroll: EventReader<MouseWheel>,
    viewpoint: Query<&Transform, (With<ViewPoint>, Without<FollowCam>)>,
) {
    let mut cam_tf = cam_query.single_mut();
    let vp = viewpoint.single();

    for ev in mouse_motion.iter() {
        // TODO: lock range
        if ev.delta.x != 0.0 {
            cam_tf.rotate_around(vp.translation, Quat::from_rotation_y(0.001 * ev.delta.x));
        }
    }
    for ev in mouse_scroll.iter() {
        // scroll += ev.y;
    }
}
