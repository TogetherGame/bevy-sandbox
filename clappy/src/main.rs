mod file_watcher;
mod companion;

use bevy::prelude::*;
use companion::setup_companion;
use file_watcher::{file_watcher_system, setup_file_watcher};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup_companion, setup_file_watcher))
        .add_systems(Update, file_watcher_system)
        .run();
}
