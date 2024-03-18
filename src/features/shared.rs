use bevy::prelude::*;

#[derive(Resource)]
pub struct Gravity(pub Vec3);

#[derive(Resource)]
pub struct Hydrogen(pub u32);

#[derive(Resource)]
pub struct MinionInventory {
    pub gathering: u32,
}

#[derive(Resource)]
pub struct PowerUpInventory {
    pub powerup1: bool,
    pub powerup2: bool,
    pub powerup3: bool,
}

pub struct SharedResourcesPlugin;

impl Plugin for SharedResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec3::NEG_Y * 5.0))
            .insert_resource(PowerUpInventory {
                powerup1: false,
                powerup2: false,
                powerup3: false,
            })
            .insert_resource(Hydrogen(0))
            .insert_resource(MinionInventory { gathering: 0 });
    }
}

pub fn despawn_component<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct Translations {
    pub top_hydrogen: &'static str,
    pub top_minions: &'static str,
    pub building_laboratory: &'static str,
    pub building_powerup: &'static str,
    pub building_minion_assembly: &'static str,
    pub building_gathering: &'static str,
    pub powerup_title: &'static str,
    pub powerup_powerup1: &'static str,
    pub powerup_powerup2: &'static str,
    pub powerup_powerup3: &'static str,
}

pub static TRANSLATIONS: Translations = Translations {
    top_hydrogen: "Hydrogen: ",
    top_minions: "Minions: ",
    building_laboratory: "Laboratory",
    building_powerup: "Power up (40 H)",
    building_minion_assembly: "Minion assembly",
    building_gathering: "Gathering (50 H)",
    powerup_title: "Power ups",
    powerup_powerup1: "Power Up 1",
    powerup_powerup2: "Power Up 2",
    powerup_powerup3: "Power Up 3",
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

pub enum PowerUps {
    PowerUp1,
    PowerUp2,
    PowerUp3,
}
