use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(WorldInspectorPlugin::new())
            .add(RapierDebugRenderPlugin::default());
        group
    }
}
