use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::features::shared::Hydrogen;

pub const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Color::rgba(0.2, 0.2, 0.2, 0.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Color::rgba(0.3, 0.3, 0.3, 0.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color + Color::rgba(0.15, 0.15, 0.15, 0.0),
        ..matl.to_owned()
    })),
};

#[derive(Component)]
pub struct PurchaseButton(pub u32);

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const DISABLED_BUTTON: Color = Color::rgb(0.4, 0.4, 0.4);
pub fn purchase_button_interaction(
    hydrogen: Res<Hydrogen>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &PurchaseButton,
        ),
        (Changed<Interaction>, With<PurchaseButton>),
    >,
) {
    for (interaction, mut color, mut border_color, price) in &mut interaction_query {
        if hydrogen.0 < price.0 {
            continue;
        }
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                continue;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
                continue;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
                continue;
            }
        }
    }
}

// Only run this on getting enough hydrogen and on click
pub fn purchase_button_enablement(
    hydrogen: Res<Hydrogen>,
    mut button_query: Query<
        (&mut BackgroundColor, &mut BorderColor, &PurchaseButton),
        With<PurchaseButton>,
    >,
) {
    for (mut color, mut border_color, price) in &mut button_query {
        if hydrogen.0 < price.0 {
            *color = DISABLED_BUTTON.into();
            border_color.0 = Color::BLACK;
            continue;
        } else {
            *color = NORMAL_BUTTON.into();
            border_color.0 = Color::BLACK;
        }
    }
}
