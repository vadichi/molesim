mod scene_manager;
mod util;

// Old
// mod nodes;

use godot::prelude::*;

struct MolesimExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for MolesimExtensionLibrary {}
