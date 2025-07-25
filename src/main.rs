use bevy::prelude::*;
use bevy::pbr::wireframe::WireframePlugin;

use crate::lighting::*;

mod lighting;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WireframePlugin::default(),
        ))
        .add_systems(Startup, setup_lighting)
        .add_systems(Update, (animate_light_direction, toggle_wireframe))
        .run();
}
