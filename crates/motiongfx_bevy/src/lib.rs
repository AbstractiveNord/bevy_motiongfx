use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::prelude::*;
use bevy_render::prelude::*;
use bevy_sprite::prelude::*;
use bevy_transform::prelude::*;
use motiongfx_core::{prelude::*, sequence::sequence_update_system};

mod sprite;
mod standard_material;
mod transform;

pub mod prelude {
    pub use crate::{
        sprite::SpriteMotion, standard_material::StandardMaterialMotion,
        transform::TransformMotion, MotionGfxBevy,
    };
}

pub struct MotionGfxBevy;

impl Plugin for MotionGfxBevy {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            ((
                sequence_update_system::<Transform, Vec3, EmptyRes>,
                sequence_update_system::<Transform, Quat, EmptyRes>,
                sequence_update_system::<Handle<StandardMaterial>, Color, Assets<StandardMaterial>>,
                sequence_update_system::<Sprite, Color, EmptyRes>,
            ),),
        );
    }
}
