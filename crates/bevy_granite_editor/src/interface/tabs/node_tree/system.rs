use super::ui::expand_to_entity;
use crate::interface::{SideDockState, SideTab};
use crate::interface::events::RequestRemoveParentsFromEntities;
use bevy::{
    ecs::query::{Changed, Or},
    prelude::{Entity, Event, EventWriter, Name, Parent, Query, ResMut, With, Without},
};
use bevy_granite_core::{GraniteType, IdentityData, TreeHiddenEntity};
use bevy_granite_gizmos::{
    ActiveSelection, GizmoMesh, GizmoParent, RequestDeselectEntityEvent, RequestSelectEntityEvent,
    RequestSelectEntityRangeEvent, Selected,
};
use bevy_granite_logging::{log, LogCategory, LogLevel, LogType};

#[derive(Debug, Clone, Event)]
pub struct RequestReparentEntityEvent {
    pub entities: Vec<Entity>, // All entities to reparent (preserving internal relationships)
    pub new_parent: Entity,    // The target parent entity
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeTreeTabData {
    pub active_selection: Option<Entity>,
    pub selected_entities: Vec<Entity>,
    pub new_selection: Option<Entity>,
    pub additive_selection: bool, // ctrl/cmd
    pub range_selection: bool,    // shift
    pub clicked_via_node_tree: bool,
    pub tree_click_frames_remaining: u8, // Frames to wait before allowing external expansion
    pub hierarchy: Vec<HierarchyEntry>,
    pub should_scroll_to_selection: bool,
    pub previous_active_selection: Option<Entity>,
    pub search_filter: String,
    pub drag_payload: Option<Vec<Entity>>, // Entities being dragged
    pub drop_target: Option<Entity>,       // Entity being dropped onto
}

#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyEntry {
    pub entity: Entity,
    pub name: String,
    pub entity_type: String,
    pub parent: Option<Entity>,
    pub is_expanded: bool,
}

pub fn update_node_tree_tabs_system(
    mut right_dock: ResMut<SideDockState>,
    active_selection: Query<Entity, With<ActiveSelection>>,
    all_selected: Query<Entity, With<Selected>>,
    hierarchy_query: Query<
        (Entity, &Name, Option<&Parent>, Option<&IdentityData>),
        (
            Without<GizmoParent>,
            Without<GizmoMesh>,
            Without<TreeHiddenEntity>,
        ),
    >,
    // detect changes (excluding Parent since we check that manually)
    changed_hierarchy: Query<
        Entity,
        (
            Or<(Changed<Name>, Changed<IdentityData>)>,
            Without<GizmoParent>,
            Without<GizmoMesh>,
            Without<TreeHiddenEntity>,
        ),
    >,
    mut select_event_writer: EventWriter<RequestSelectEntityEvent>,
    mut deselect_event_writer: EventWriter<RequestDeselectEntityEvent>,
    mut select_range_event_writer: EventWriter<RequestSelectEntityRangeEvent>,
    mut reparent_event_writer: EventWriter<RequestReparentEntityEvent>,
    mut remove_parents_event_writer: EventWriter<RequestRemoveParentsFromEntities>,
) {
    for (_, tab) in right_dock.dock_state.iter_all_tabs_mut() {
        if let SideTab::NodeTree { ref mut data, .. } = tab {
            let previous_selection = data.active_selection;
            data.active_selection = active_selection.get_single().ok();
            data.selected_entities = all_selected.iter().collect();

            let (entities_changed, data_changed, hierarchy_changed) = 
                detect_changes(&hierarchy_query, &changed_hierarchy, data);

            if entities_changed || data_changed || hierarchy_changed {
                update_hierarchy_data(data, &hierarchy_query, hierarchy_changed);
            }

            // Check if selection changed externally
            if let Some(new_active) = data.active_selection {
                if previous_selection != Some(new_active)
                    && !data.clicked_via_node_tree
                    && data.tree_click_frames_remaining == 0
                {
                    // Auto-expand and scroll for any external selection change (including initial selection)
                    expand_to_entity(&mut data.hierarchy, new_active);
                    data.should_scroll_to_selection = true;
                    
                    log!(
                        LogType::Editor,
                        LogLevel::Info,
                        LogCategory::UI,
                        "External selection detected - expanding to entity {:?}",
                        new_active
                    );
                } else {
                    // Prevent scroll/expand for user clicks or no change
                    data.should_scroll_to_selection = false;
                }
            }

            // Handle tree clicks
            if let Some(new_selection) = data.new_selection {
                if data.clicked_via_node_tree {
                    if data.range_selection {
                        // Range selection: select all between previous_active_selection and new_selection
                        if let Some(prev) = data.active_selection {
                            // Build visual order of currently visible nodes
                            let visual_order = build_visual_order(&data.hierarchy);
                            
                            // Find indices in visual order
                            let prev_index = visual_order.iter().position(|&e| e == prev);
                            let new_index = visual_order.iter().position(|&e| e == new_selection);
                            
                            if let (Some(prev_idx), Some(new_idx)) = (prev_index, new_index) {
                                let start = prev_idx.min(new_idx);
                                let end = prev_idx.max(new_idx);
                                
                                let range_entities = visual_order[start..=end].to_vec();
                                
                                log!(
                                    LogType::Editor,
                                    LogLevel::Info,
                                    LogCategory::UI,
                                    "Range selection from {:?} to {:?}: {} entities",
                                    prev,
                                    new_selection,
                                    range_entities.len()
                                );
                                
                                select_range_event_writer.send(RequestSelectEntityRangeEvent {
                                    entities: range_entities,
                                    additive: true,
                                });
                                // Always set the clicked entity as active selection
                                data.active_selection = Some(new_selection);
                                // The other end is previous_active_selection
                                data.previous_active_selection = Some(prev);
                            } else {
                                // Fallback to single selection if either entity not found in visual order
                                select_event_writer.send(RequestSelectEntityEvent {
                                    entity: new_selection,
                                    additive: false,
                                });
                                data.previous_active_selection = data.active_selection;
                                data.active_selection = Some(new_selection);
                            }
                        } else {
                            // No previous selection, fallback to single
                            select_event_writer.send(RequestSelectEntityEvent {
                                entity: new_selection,
                                additive: false,
                            });
                            data.previous_active_selection = data.active_selection;
                            data.active_selection = Some(new_selection);
                        }
                    } else if data.additive_selection {
                        // Ctrl/Cmd (additive): toggle selection
                        let already_selected = data.selected_entities.contains(&new_selection);
                        if already_selected {
                            // Deselect
                            deselect_event_writer.send(RequestDeselectEntityEvent(new_selection));
                        } else {
                            // Add to selection
                            select_event_writer.send(RequestSelectEntityEvent {
                                entity: new_selection,
                                additive: true,
                            });
                        }
                        // Always set the clicked entity as active selection
                        data.previous_active_selection = data.active_selection;
                        data.active_selection = Some(new_selection);
                    } else {
                        // Normal selection
                        select_event_writer.send(RequestSelectEntityEvent {
                            entity: new_selection,
                            additive: false,
                        });
                        data.previous_active_selection = data.active_selection;
                        data.active_selection = Some(new_selection);
                    }
                    // Set counter to prevent expansion for a few frames while events are processed
                    data.tree_click_frames_remaining = 3;
                    data.clicked_via_node_tree = false;
                }
            }
            data.new_selection = None;
            data.additive_selection = false;
            data.range_selection = false;
            
            // Decrement frame counter for tree click protection
            if data.tree_click_frames_remaining > 0 {
                data.tree_click_frames_remaining -= 1;
            }

            // Handle drag and drop
            if let Some(dragged_entities) = data.drag_payload.clone() {
                // Check if any drop occurred
                if let Some(drop_target) = data.drop_target {
                    if drop_target == Entity::PLACEHOLDER {
                        // Special case: drop on empty space = remove parents
                        log!(
                            LogType::Editor,
                            LogLevel::OK,
                            LogCategory::UI,
                            "Remove parents event - dropped on empty space"
                        );
                        remove_parents_event_writer.send(RequestRemoveParentsFromEntities {
                            entities: dragged_entities,
                        });
                    } else if is_valid_drop(&dragged_entities, drop_target, &data.hierarchy) {
                        log!(
                            LogType::Editor,
                            LogLevel::OK,
                            LogCategory::UI,
                            "Drag parent event"
                        );
                        reparent_event_writer.send(RequestReparentEntityEvent {
                            entities: dragged_entities,
                            new_parent: drop_target,
                        });
                    }
                    // Always clear both drag payload and drop target after processing
                    data.drag_payload = None;
                    data.drop_target = None;
                }
                // Don't clear drag payload here - let it persist until drop or explicit cancel
            }
        }
    }
}

/// Check if dropping the entities onto the target would create a valid hierarchy
pub fn is_valid_drop(entities: &[Entity], target: Entity, hierarchy: &[HierarchyEntry]) -> bool {
    // Don't allow dropping onto any of the entities being dragged
    if entities.contains(&target) {
        return false;
    }

    // Don't allow dropping a parent onto any of its descendants
    for &entity in entities {
        if is_descendant_of(target, entity, hierarchy) {
            return false;
        }
    }

    true
}

/// Check if `potential_descendant` is a descendant of `ancestor`
pub fn is_descendant_of(
    potential_descendant: Entity,
    ancestor: Entity,
    hierarchy: &[HierarchyEntry],
) -> bool {
    let mut current = potential_descendant;

    while let Some(entry) = hierarchy.iter().find(|e| e.entity == current) {
        if let Some(parent) = entry.parent {
            if parent == ancestor {
                return true;
            }
            current = parent;
        } else {
            break;
        }
    }

    false
}

fn detect_changes(
    hierarchy_query: &Query<
        (Entity, &Name, Option<&Parent>, Option<&IdentityData>),
        (
            Without<GizmoParent>,
            Without<GizmoMesh>,
            Without<TreeHiddenEntity>,
        ),
    >,
    changed_hierarchy: &Query<
        Entity,
        (
            Or<(Changed<Name>, Changed<IdentityData>)>,
            Without<GizmoParent>,
            Without<GizmoMesh>,
            Without<TreeHiddenEntity>,
        ),
    >,
    data: &NodeTreeTabData,
) -> (bool, bool, bool) {
    use std::collections::HashSet;
    
    let current_entities: HashSet<Entity> =
        hierarchy_query.iter().map(|(e, _, _, _)| e).collect();
    let existing_entities: HashSet<Entity> =
        data.hierarchy.iter().map(|entry| entry.entity).collect();

    // Check if entities changed OR if any existing entity had its data changed OR if parent relationships changed
    let entities_changed = current_entities != existing_entities;
    let data_changed = !changed_hierarchy.is_empty();
    
    // Also check if any parent relationships changed by comparing current vs stored hierarchy
    let hierarchy_changed = if !entities_changed {
        hierarchy_query.iter().any(|(entity, _, parent, _)| {
            if let Some(entry) = data.hierarchy.iter().find(|e| e.entity == entity) {
                let current_parent = parent.map(|p| p.get());
                entry.parent != current_parent
            } else {
                true // Entity not found in stored hierarchy
            }
        })
    } else {
        false // entities_changed already covers this case
    };

    (entities_changed, data_changed, hierarchy_changed)
}

fn build_visual_order(hierarchy: &[HierarchyEntry]) -> Vec<Entity> {
    use std::collections::HashMap;
    
    // Build parent -> children map
    let mut children_map: HashMap<Option<Entity>, Vec<Entity>> = HashMap::new();
    for entry in hierarchy {
        children_map.entry(entry.parent).or_default().push(entry.entity);
    }
    
    // Sort children by entity index to maintain consistent order
    for children in children_map.values_mut() {
        children.sort_by_key(|entity| entity.index());
    }
    
    // Build expansion state map
    let expanded_map: HashMap<Entity, bool> = hierarchy
        .iter()
        .map(|entry| (entry.entity, entry.is_expanded))
        .collect();
    
    let mut visual_order = Vec::new();
    
    // Recursive function to build visual order
    fn add_visible_children(
        parent: Option<Entity>,
        children_map: &HashMap<Option<Entity>, Vec<Entity>>,
        expanded_map: &HashMap<Entity, bool>,
        visual_order: &mut Vec<Entity>,
    ) {
        if let Some(children) = children_map.get(&parent) {
            for &child in children {
                visual_order.push(child);
                
                // Only add children if this node is expanded
                if expanded_map.get(&child).copied().unwrap_or(false) {
                    add_visible_children(Some(child), children_map, expanded_map, visual_order);
                }
            }
        }
    }
    
    // Start with root nodes (parent = None)
    add_visible_children(None, &children_map, &expanded_map, &mut visual_order);
    
    visual_order
}

fn update_hierarchy_data(
    data: &mut NodeTreeTabData,
    hierarchy_query: &Query<
        (Entity, &Name, Option<&Parent>, Option<&IdentityData>),
        (
            Without<GizmoParent>,
            Without<GizmoMesh>,
            Without<TreeHiddenEntity>,
        ),
    >,
    hierarchy_changed: bool,
) {
    use std::collections::HashMap;
    
    if hierarchy_changed {
        log!(
            LogType::Editor,
            LogLevel::Info,
            LogCategory::UI,
            "Hierarchy relationships changed - refreshing node tree"
        );
    }
    let existing_expanded: HashMap<Entity, bool> = data
        .hierarchy
        .iter()
        .map(|entry| (entry.entity, entry.is_expanded))
        .collect();

    let mut hierarchy_entries: Vec<HierarchyEntry> = hierarchy_query
        .iter()
        .map(|(entity, name, parent, identity)| HierarchyEntry {
            entity,
            name: name.to_string(),
            entity_type: identity
                .map(|id| id.class.type_abv())
                .unwrap_or_else(|| "Unknown".to_string()),
            parent: parent.map(|p| p.get()),
            is_expanded: existing_expanded.get(&entity).copied().unwrap_or(false),
        })
        .collect();

    hierarchy_entries.sort_by_key(|entry| entry.entity.index());
    data.hierarchy = hierarchy_entries;
}
