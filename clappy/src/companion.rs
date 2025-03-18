use bevy::prelude::*;

// Animation states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PetState {
    Idle,
    Happy,
    Sad,
    Angry,
}

// Animation component
#[derive(Component)]
pub(crate) struct Animation(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

pub(super) fn setup_companion(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture = assets.load("textures/clappy/idle.png");
    let texture_atlas_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None)
    );
    
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(texture, TextureAtlas {
            index: 0,
            layout: texture_atlas_layout,
        }),
        Transform::from_scale(Vec3::splat(6.0)),
        Animation(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
