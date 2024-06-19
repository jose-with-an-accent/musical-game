use std::{any::Any, collections::BTreeMap};

use bevy::{prelude::*, reflect::serde::ReflectSerializer};
pub struct LevelPlugin;

#[derive(Event)]
pub enum LevelIOEvent {
    SAVE,
    LOAD,
}
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, )
        app
        // .add_systems(PreUpdate, save_default().into_file("world.ron").run_if(should_save));
        .add_systems(Update, (handle_open_save));
    }
}
// // maybe this can help? https://github.com/JosephCatrambone/roguelike_template_rust/blob/98fb3b73de9e3d7b86f9cf49566e670251b2c6fc/src/saveload.rs#L17
fn handle_open_save(mut event_reader: EventReader<LevelIOEvent>, world: &World) {
    for event in event_reader.read() {
        match event {
            LevelIOEvent::SAVE => {
                let mut entities_to_serialize: BTreeMap<Entity, BTreeMap<usize, String>> = BTreeMap::new();
                let type_registry = world.resource::<AppTypeRegistry>().clone();
                let type_registry = type_registry.read();

                // Iterate over entities and copy them into our scene_world
                for entity in world.iter_entities() {
                    let mut serializable_entries = BTreeMap::new();
                    for component_id in entity.archetype().components() {
                        if let Some(info) = world.components().get_info(component_id) {
                            let type_id = info.type_id().unwrap();

                            if let Some(registration) = type_registry.get(type_id) {
                                if !matches!(registration.data::<ReflectComponent>(), Some(_)) {
                                    if let Some(reflect_components) = registration.data::<ReflectComponent>() {
                                        let reflected_ref = reflect_components.reflect(entity).unwrap();
    
                                        let serializer = ReflectSerializer::new(reflected_ref, &type_registry);
                                        let _ = &serializable_entries.insert(component_id.index(), ron::ser::to_string(&serializer).unwrap());
                                    }
    
                                }
                            };  

                        }
                        entities_to_serialize.insert(entity.id(), serializable_entries.clone());
                    }


                }   

                
                let serialized_value: String = ron::to_string(&entities_to_serialize).unwrap();
            
                // Showing the scene in the console
                info!("{}", serialized_value);
            }
            LevelIOEvent::LOAD => {
                // Handle loading here if needed
            }
        }
    }
}