use amethyst::{
    assets::{Completion, ProgressCounter},
    ecs::{Entities},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        VirtualKeyCode,
    },
    ui::{
        UiCreator, UiEvent, UiEventType, UiFinder, UiTransform,
    },
};

use crate::graphics::*;
use crate::level::{
    Level,
    init_level,
};

#[derive(Default)]
pub struct MainMenu {
}

#[derive(Default)]
pub struct Loading {
    progress: ProgressCounter,
    level: Level,
}

pub struct Main {
    level: Level,
}

fn ui_transform_id_of_ui_event<'a, 'b>(
    ui_event: &UiEvent,
    world: &World,
) -> Option<String> {
    let entity = ui_event.target;
    let ui_transform_storage =
        &world.read_storage::<UiTransform>();
    if let Some(ui_transform) = &ui_transform_storage.get(entity) {
        let id = &ui_transform.id;
        Some(id.clone())
    } else {
        None
    }
}

fn unload_main_menu(
    world: &mut World,
) -> () {
    let main_menu_ui_ids = [
        "main_menu_background",
        "main_menu_game_title",
        "start_game_button",
        "exit_game_button"
    ];
    world.exec(|(finder, entities): (UiFinder<'_>, Entities<'_>)| {
        for main_menu_ui_id in main_menu_ui_ids.iter() {
           if let Some(entity) = finder.find(main_menu_ui_id) {
               entities.delete(entity);
           }
        }
    });
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut progress: ProgressCounter = ProgressCounter::new();
        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/main_menu.ron", &mut progress);
            creator.create("ui/fps.ron", &mut progress);
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                let window_closed = is_close_requested(&event);
                let esc_key_down = is_key_down(&event, VirtualKeyCode::Escape);
                if window_closed || esc_key_down {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                match &ui_event.event_type {
                    UiEventType::Click => {
                        let maybe_id = ui_transform_id_of_ui_event(
                            &ui_event,
                            &data.world
                        );
                        if let Some(id) = maybe_id {
                            if id == "exit_game_button" {
                                return Trans::Quit
                            }
                            if id == "start_game_button" {
                                let next_state = Loading::default();

                                return Trans::Switch(Box::new(next_state));
                            }
                        }
                    }
                    _ => {}
                }
                Trans::None
            }
        }
    }
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", &mut self.progress);
        });

        load_assets(data.world, &mut self.progress);
    }

    fn update(
        &mut self,
        data: &mut StateData<'_, GameData<'_, '_>>
    ) -> SimpleTrans {
        unload_main_menu(&mut data.world);
        match self.progress.complete() {
            Completion::Failed => {
                println!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            Completion::Complete => {
                println!("Assets loaded ({}/{}), swapping state",
                         self.progress.num_finished(),
                         self.progress.num_assets());
                if let Some(entity) = data
                    .world
                    .exec(|finder: UiFinder<'_>| finder.find("loading"))
                {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(Main {
                    level: self.level.clone(),
                }))
            }
            Completion::Loading => {
                Trans::None
            }
        }
    }
}

impl SimpleState for Main {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let assets = world.read_resource::<Assets>().clone();
        init_level(world, assets, &self.level);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Handle Quitting
            let window_closed = is_close_requested(&event);
            let esc_key_down = is_key_down(&event, VirtualKeyCode::Escape);
            if window_closed || esc_key_down {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}
