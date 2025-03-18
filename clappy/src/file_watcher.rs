use std::{path::Path, sync::mpsc::{channel, Receiver}};
use bevy::prelude::*;
use notify::{EventKind, RecommendedWatcher, Watcher};

use crate::companion::PetState;

#[derive(Resource)]
pub(super) struct FileWatchResource(Receiver<notify::Result<notify::Event>>);

pub(super) fn setup_file_watcher(mut commands: Commands) {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        tx.send(res).unwrap();
    }).unwrap();

    // Watch the status file (modify path as needed)
    watcher.watch(Path::new("pet_status.txt"), notify::RecursiveMode::Recursive).unwrap();

    commands.insert_resource(watcher);
    commands.insert_resource(FileWatchResource(rx));
}

pub(super) fn file_watcher_system(
    file_resource: Res<FileWatchResource>,
    mut pet_state: ResMut<State<PetState>>,
) {
    for event in file_resource.0.try_iter() {
        if let Ok(event) = event {
            if matches!(event.kind, EventKind::Modify(_)) {
                if let Ok(content) = std::fs::read_to_string("pet_status.txt") {
                    let new_state = match content.trim() {
                        "happy" => PetState::Happy,
                        "sad" => PetState::Sad,
                        "angry" => PetState::Angry,
                        _ => PetState::Idle,
                    };
                    pet_state.set(new_state).unwrap();
                }
            }
        }
    }
}
