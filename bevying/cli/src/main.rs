use bevy::DefaultPlugins;
use bevying::app::create_app;
fn main() {
    let (tx, rx) = kanal::bounded(100);
    let mut app = create_app(rx);
    app.add_plugins(bevy_mod_debugdump::CommandLineArgs).run();
}
