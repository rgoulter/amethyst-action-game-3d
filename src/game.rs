use amethyst::{
    assets::{AssetStorage, Loader},
    assets::{Completion, ProgressCounter},
    ecs::{Entities},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        ComboMeshCreator, MeshHandle, SpriteSheet,
        VirtualKeyCode,
    },
    ui::{
        UiCreator, UiEvent, UiEventType, UiFinder, UiTransform,
    },
};

use crate::graphics::*;
use crate::grid_of_sprites::*;
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
    kludge_progress: Option<ProgressCounter>,
    kludge_gos_mesh: Option<MeshHandle>,
    level: Level,
}

pub struct Main {
    level: Level,
    kludge_gos_mesh: MeshHandle,
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

        let kludge_trans = match self.progress.complete() {
            Completion::Failed => {
                println!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            Completion::Complete => {
                if self.kludge_progress.is_none() {
                    println!("asset loading complete; loading kludge");
                    // KLUDGE: still want to load the grid_of_sprites,
                    //         after the other assets.
                    let assets = data.world.read_resource::<Assets>().clone();

                    let ssh = assets.map_sprite_sheet;
                    let sprite_sheet_storage =
                        data
                        .world
                        .read_resource::<AssetStorage<SpriteSheet>>();
                    let sprite_sheet: &SpriteSheet =
                        sprite_sheet_storage
                        .get(&ssh)
                        .expect("spritesheet should be loaded by this point");

                    let grid = vec![
                        vec![1, 1, 1, 1],
                        vec![1, 2, 1, 1],
                        vec![1, 1, 3, 3],
                        vec![3, 3, 4, 3],
                    ];

                    let mut kludge_progress = ProgressCounter::default();

                    let gos = GridOfSprites {
                        sprite_sheet: sprite_sheet.clone(),
                        grid,
                        num_rows: 4,
                        num_cols: 4,
                    };
                    let gos_mesh_data = gos.generate::<ComboMeshCreator>(None);
                    let loader = data.world.read_resource::<Loader>();
                    let mesh_storage = data.world.read_resource();
                    let gos_mesh_handle = {
                        let progress: &mut ProgressCounter = &mut kludge_progress;
                        loader.load_from_data(gos_mesh_data, progress, &mesh_storage)
                    };

                    self.kludge_progress = Some(kludge_progress);
                    self.kludge_gos_mesh = Some(gos_mesh_handle);
                }

                Trans::None
            }
            Completion::Loading => {
                Trans::None
            }
        };

        self.kludge_progress.as_ref().map(|kludge_progress| {
            match kludge_progress.complete() {
                Completion::Failed => {
                    println!("Failed loading kludge assets: {:?}", self.progress.errors());
                    Trans::Quit
                }
                Completion::Complete => {
                    // KLUDGE: *now* every asset we care about has completed loading.
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
                        kludge_gos_mesh: self.kludge_gos_mesh.clone().expect("kludge should be loaded")
                    }))
                }
                Completion::Loading => {
                    Trans::None
                }
            }
        }).unwrap_or(kludge_trans)
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
