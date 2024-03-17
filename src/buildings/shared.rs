use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
