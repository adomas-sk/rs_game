use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

#[derive(Resource)]
pub struct Hydrogen(pub u32);

#[derive(Resource)]
pub struct MinionInventory {
    pub gathering: u32,
}

pub struct SharedResourcesPlugin;

impl Plugin for SharedResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec3::NEG_Y * 5.0))
            .insert_resource(Hydrogen(0))
            .insert_resource(MinionInventory { gathering: 0 });
    }
}

pub struct Translations {
    pub hydrogen: &'static str,
    pub laboratory: &'static str,
    pub power_up: &'static str,
    pub minion_assembly: &'static str,
    pub gathering: &'static str,
    pub minions: &'static str,
}

pub static TRANSLATIONS: Translations = Translations {
    hydrogen: "Hydrogen: ",
    laboratory: "Laboratory",
    power_up: "Power up (40 H)",
    minion_assembly: "Minion assembly",
    gathering: "Gathering (50 H)",
    minions: "Minions: ",
};

pub struct Prices {
    pub gathering_minion: u32,
    pub power_up: u32,
}

pub static PRICES: Prices = Prices {
    gathering_minion: 50,
    power_up: 40,
};

#[derive(Component)]
pub struct BuildingUI;
