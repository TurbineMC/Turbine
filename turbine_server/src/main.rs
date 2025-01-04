use bevy::prelude::*;
#[derive(Resource)]
struct RuntimeResource { // a wrapper as we cant derive on 3rd party stuff
    runtime: tokio::runtime::Runtime,
}
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(RuntimeResource {
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        })
        .add_systems(Startup, start_networking)
        .run();
}

#[profiling::function]
fn start_networking(mut commands: Commands, runtime: Res<RuntimeResource>) {}

#[profiling::function]
fn process_networking(runtime: Res<RuntimeResource>) {}
