use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

#[derive(Resource)]
pub struct Hydrogen(pub u32);

pub struct SharedResourcesPlugin;

impl Plugin for SharedResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec3::NEG_Y * 5.0))
            .insert_resource(Hydrogen(0));
    }
}

pub struct Translations {
    pub hydrogen: &'static str,
}

pub static TRANSLATIONS: Translations = Translations {
    hydrogen: "Hydrogen: ",
};

#[derive(PartialEq)]
pub enum Buildings {
    None = 0,
    MinionAssembly = 1,
}
