use bevy::prelude::*;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster};

pub(crate) struct LevelPlugin;

#[derive(Component)]
pub(crate) struct Sun;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_ground, setup_visual));
    }
}

fn setup_ground(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(10.))),
        material: materials.add(Color::rgb(0.25, 0.5, 0.25).into()),
        ..default()
    });
}

fn setup_visual(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sun
    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_at(Vec3::new(-0.15, -0.15, 0.25), Vec3::Y),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 0.3,
            maximum_distance: 3.,
            ..default()
        }.build(),
        ..default()
    })
    .insert(Sun);

    // Sky
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}
