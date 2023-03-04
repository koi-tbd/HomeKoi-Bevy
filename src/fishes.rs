use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    camera::Tracked,
    forces::{Alignment, Cohesive, Friction, Moveable, Separation},
};

#[derive(AssetCollection, Resource)]
pub struct FishAssets {
    #[asset(path = "models/fish.glb#Scene0")]
    fish_scene: Handle<Scene>,
    #[asset(path = "models/fish.glb#Animation0")]
    fish_animation: Handle<AnimationClip>,
}

#[derive(Component, Default)]
pub struct Fish;

pub fn fish_animator_system(
    fish_assets: Res<FishAssets>,
    fishes: Query<(Entity, &Moveable, With<Fish>)>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
) {
    fishes.for_each(|(entity, moveable, _)| {
        for child in children.iter_descendants(entity) {
            if let Ok(mut player) = players.get_mut(child) {
                if player.is_added() {
                    player.play(fish_assets.fish_animation.clone()).repeat();
                }

                let speed = moveable.velocity.length();
                let animation_speed = 1.0 + speed;
                player.set_speed(animation_speed);
            }
        }
    });
}

impl Fish {
    pub fn new_npc(transform: Transform, fish_assets: &Res<FishAssets>) -> impl Bundle {
        (
            SceneBundle {
                scene: fish_assets.fish_scene.clone(),
                transform: transform,
                ..default()
            },
            Fish,
            Moveable::default(),
            Friction::default(),
            Separation::default(),
            Cohesive::default(),
            Alignment::default(),
            Tracked::default(),
        )
    }
}