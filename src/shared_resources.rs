use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

pub struct SharedResourcesPlugin;

impl Plugin for SharedResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec3::NEG_Y * 5.0));
    }
}
