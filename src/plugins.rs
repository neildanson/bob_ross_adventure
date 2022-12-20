use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        if cfg!(debug_assertions) {
            let group = PluginGroupBuilder::start::<Self>();
            group
                .add(WorldInspectorPlugin::new())
                .add(RapierDebugRenderPlugin::default())
        } else {
            PluginGroupBuilder::start::<Self>()
        }
    }
}
