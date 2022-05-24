mod components;
mod entities;
mod systems;
mod events;
mod resources;

mod prelude {

    // Bevy
    pub use bevy::prelude::*;

    // Crate
    pub use crate::entities::*;
    pub use crate::components::*;
    pub use crate::components::Name; // Shadowing bevy::prelude::Name
    pub use crate::systems::*;
    pub use crate::events::*;
    pub use crate::resources::*;
}

use prelude::*;

fn main() {

    let mut app = App::new();

    app
        .add_plugins(MinimalPlugins)
        .add_event::<InputEvent>()
        .add_startup_system(spawn_player)
        .add_system(overview)
        .add_system(input)
        .add_system(handle_command);
    
    app.run();

}