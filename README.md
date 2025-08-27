

# Granite Bevy Editor

This crate provides a way to interactively create, edit, save, and load Bevy data in 3D.

```
Caution: This is in early development and you will likely encounter bugs
```

![Screenshot](./screenshots/Image_2.png)


## Getting Starting

Navigate to your projects Cargo.toml, or create a fresh rust project with cargo new.

Be sure to use bevy 0.14 and link the bevy_granite plugin to my github repo. You can select alternate branches for more up-to-date releases. Also make sure you have the serde crate.
```rust
[dependencies]
bevy = "0.14.0"
bevy_granite = { git = "https://github.com/BlakeDarrow/bevy_granite", branch = "main" }
serde = "1.0.215"

```

There are 3 optional feature sets.
- Gizmos - Only loads needed content for gizmos. No editor or UI.
- Core - No Editor or Gizmos, just the bare bones needed to use our serial/deserial functions with macros and logging.
- Editor - The same as default features. Includes Gizmos, Core, and all Editor functionality.


## Example main.rs

Check out the [examples](https://github.com/BlakeDarrow/bevy_granite/tree/main/examples).

```rust
// main.rs - bevy 0.14
use bevy::prelude::*;
use bevy_granite::prelude::*; // Import the Granite plugin prelude

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
        .add_plugins(bevy_granite::BevyGranite{
            default_world: STARTING_WORLD.to_string(),
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut open_event: EventWriter<RequestLoadEvent>) {
    // Event to load a world (.scene)
    // When finished loading it will send a `WorldLoadSuccessEvent` with the loaded world str name
    open_event.write(RequestLoadEvent(STARTING_WORLD.to_string()));
}
```

You will want to create a fresh camera 3d using "Shift-A" and select Camera from the gameplay sub-menu. Once added, in the *Entity Editor* add a new component and search for "MainCamera" this will allow us to track the camera for things like UI and Gizmos.


## Example starting.scene

This is an example of what the file format looks like once a few entities have been added.

An entity is mainly composed of three main parts:
- **identity**: Contains the entity’s name, uuid, and type/class (such as Camera, Light, OBJ). This class data contains everything necessary to rebuild this bundle and any other adjacently relevant data. Not everything is currently available in classes.
- **transform**: Describes the entity’s position, rotation, and scale. This determines where the entity is located and how it is oriented in the world.
- **components**: (Optional) Holds additional data or behaviors attached to the entity. This is where you extend the entity’s functionality via the `#[granite_component]` macro. Note, in this example we are using the crates internal MainCamera granite_component. This help sync a UI camera that renders additional information. Please attach this struct to your main camera.
```ron
(
	metadata: (
		format_version: "0.1.4",
		entity_count: 3,
	),
	entities: [(
		identity: (
			uuid: "edc43545-977a-42e2-b0fb-3c44b081f13c",
			name: "Main cam",
			class: Camera3D((
				is_active: true,
				has_volumetric_fog: false,
			)),
		),
		transform: (
			position: (16.538, 8.348, 11.926),
			rotation: (-0.197, 0.427, 0.096, 0.877),
			scale: (1.0, 1.0, 1.0),
		),
		components: Some({
			"bevy_granite_core::entities::MainCamera": "{\"bevy_granite_core::entities::MainCamera\":()}",
		}),
	), (
		identity: (
			uuid: "afd7153c-126e-4b57-af9d-c30a654439dc",
			name: "Ico sphere",
			class: OBJ((
				mesh_path: "models/Ico Sphere.obj",
				material: (
					path: "materials/obj_imports/Ico_Sphere.mat",
				),
			)),
		),
		transform: (
			position: (0.0, 0.0, 0.0),
			rotation: (0.0, 0.0, 0.0, 1.0),
			scale: (1.0, 1.0, 1.0),
		),
	), (
		identity: (
			uuid: "f805f8b8-64be-461c-93f1-3eab2eb3b399",
			name: "Sun",
			class: DirLight((
				color: (0.45490196, 0.68235296, 0.9098039),
				illuminance: 2000.0,
				shadows_enabled: true,
				volumetric: false,
			)),
		),
		transform: (
			position: (0.0, 0.0, 0.0),
			rotation: (-0.116, 0.0, 0.0, 0.993),
			scale: (1.0, 1.0, 1.0),
		),
	)],
)

```

## Documentation

While comprehensive documentation is currently unavailable, here are some helpful events you can use to interact with the editor while I write said documentation:

### Editor Control Events

- `RequestEditorToggle` - Toggle the editor UI on/off
- `RequestToggleCameraSync` - Toggle camera synchronization between editor and main camera

### Entity Selection Events
- `RequestSelectEntityEvent { entity: Entity, additive: bool }` - Select an entity (additive for multi-selection)
- `RequestDeselectEntityEvent(Entity)` - Deselect a specific entity
- `RequestDeselectAllEntitiesEvent` - Clear all entity selections
- `RequestCameraEntityFrame` - Frame the UI camera to focus on active entity

### Entity Duplication Events
- `RequestDuplicateEntityEvent { entity: Entity }` - Duplicate a specific entity
- `RequestDuplicateAllSelectionEvent` - Duplicate all currently selected entities

### Entity Hierarchy Events
- `RequestNewParent` - Request to set active as parent for selected entities
- `RequestRemoveParents` - Remove parent relationships from selected entities
- `RequestRemoveChildren` - Remove child relationships from selected entities

### World Management Events
- `RequestSaveEvent(String)` - Save the specific world
- `RequestLoadEvent(String)` - Load a world from specified path
- `RequestReloadEvent(String)` - Reload a world from specified path
- `WorldLoadSuccessEvent(String)` - Event sent when world loading completes successfully
- `WorldSaveSuccessEvent(String)` - Event sent when world saving completes successfully
- `RequestDespawnSerializableEntities` - Event to despawn all serializable entities
- `RequestDespawnBySource(String)` - Event to despawn a specific source that is loaded


## Feedback

If you have any feedback, please reach out to me via a [GitHub issue](https://github.com/BlakeDarrow/bevy_granite/issues). I look forward to maintaining and improving this tool and am happy to hear y'alls opinions, but please keep it constructive.

## Authors

- [@BlakeDarrow](https://www.youtube.com/@blakedarrow) on YouTube

## Support Table

This project was started when bevy 0.14 was just released, and I haven't upgraded since this. This is my top priorty to bring new bevy features into the editor.

| bevy | bevy_granite |
|------|--------------|
| 0.14 | 0.1.0        |

## License

Granite is free and open source. Except when noted, all assets are licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

## Contributing

Any sort of contributions are welcome! Open a pull request and I will be sure to look at it. If you are unsure of what you can fix or add, open an issue and lets talk about it. Though again I will add our frist priorty should be upgrading past bevy 0.14.

Any contributions by you, shall be dual licensed as above, without any additional terms or conditions.

## Additional Images

![Screenshot](./screenshots/Image_1.png)
![Screenshot](./screenshots/Image_3.png)


## Special Thanks

 - Noah
 - Silas
 - Ethan
 - Max
