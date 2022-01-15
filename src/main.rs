use editor::EditorPlugin;
use game::GamePlugin;
use game::prelude::*;

fn main() {
    App::new()
        .add_plugin(GamePlugin {})
        .add_plugin(EditorPlugin {})
        .run();
}
