use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct SelectMinionAssemblyBuilding;

impl From<ListenerInput<Pointer<Select>>> for SelectMinionAssemblyBuilding {
    fn from(_: ListenerInput<Pointer<Select>>) -> Self {
        SelectMinionAssemblyBuilding
    }
}

#[derive(Event)]
pub struct SelectLaboratoryBuilding;

impl From<ListenerInput<Pointer<Select>>> for SelectLaboratoryBuilding {
    fn from(_: ListenerInput<Pointer<Select>>) -> Self {
        SelectLaboratoryBuilding
    }
}

#[derive(Event)]
pub struct DeselectBuilding;

impl From<ListenerInput<Pointer<Deselect>>> for DeselectBuilding {
    fn from(_: ListenerInput<Pointer<Deselect>>) -> Self {
        DeselectBuilding
    }
}
