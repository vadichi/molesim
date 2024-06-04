mod util;
mod nodes;

use godot::prelude::*;

struct MolesimExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for MolesimExtensionLibrary {}
