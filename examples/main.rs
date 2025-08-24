// main.rs - bevy 0.14
use bevy::{input::keyboard::Key, prelude::*};
use bevy_granite::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiContexts}; // Import the Granite plugin prelude

const STARTING_WORLD: &str = "scenes/starting.scene"; // Your starting world file. Doesn't have to actually exist yet

// Macro to define a component that can be edited in Granite
// This also adds the necessary derives for saving/loading
// You can add `default` to the argument to then add a 'Default' implementation
// You can also add `ui_hidden` to hide the component from the UI
// This component can be queried in bevy like any other component
#[granite_component]
struct MyTestComponent {
    value: i32,
}

#[granite_component("default")]
struct AnotherComponent {
    message: String,
}

impl Default for AnotherComponent {
    fn default() -> Self {
        AnotherComponent {
            message: "Hello, Granite!".to_string(),
        }
    }
}

fn main() {
    let mut app = App::new();

    // Critical macro to register all your editor components
    // If components are not registered here they will not be available in the editor
    // So you should import all your components in main.rs and then call this macro
    register_editor_components!();

    app.add_plugins(DefaultPlugins)
        // Add the Granite plugin
        // Make sure to pass a default world otherwise a default will be used
        // It it used to easily save/load your starting world via UI
        .add_plugins(bevy_granite::BevyGranite {
            default_world: STARTING_WORLD.to_string(),
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, print_cube_color)
        .run();
}

fn setup(mut open_event: EventWriter<RequestLoadEvent>) {
    // Event to load a world (.scene)
    // When finished loading it will send a `WorldLoadSuccessEvent` with the loaded world str name
    open_event.write(RequestLoadEvent(STARTING_WORLD.to_string()));
}

fn test_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ONE * 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Create a simple cube with a material
    let mesh = meshes.add(Cuboid::from_length(1.));
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..Default::default()
    });

    commands.spawn((Mesh3d(mesh), MeshMaterial3d(material)));
}

fn print_cube_color(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&MeshMaterial3d<StandardMaterial>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    if !input.just_pressed(KeyCode::F12) {
        return;
    }
    for mesh_material in query.iter() {
        if let Some(material) = materials.get(&mesh_material.0) {
            println!("Cube color: {:?}", material.base_color);
        } else {
            println!("Material not found for handle: {:?}", mesh_material.0);
        }
    }
}
