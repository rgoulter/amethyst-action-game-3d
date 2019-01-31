use amethyst::{
    assets::{Completion, ProgressCounter},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        VirtualKeyCode,
    },
    ui::{UiCreator, UiFinder},
};

use crate::graphics::*;
use crate::level::{
    Level,
    init_level,
};


#[derive(Default)]
pub struct Loading {
    progress: ProgressCounter,
    level: Level,
}

pub struct Main {
    level: Level,
}

impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress);
        });

        load_assets(data.world, &mut self.progress);
    }

    fn update(
        &mut self,
        data: &mut StateData<'_, GameData<'_, '_>>
    ) -> SimpleTrans {
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
