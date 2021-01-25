use crate::state::{world, AppStates, STAGE_MENU};
/// Project startup menu state module
use bevy::prelude::*;
use derive_more::Constructor;

/// Bevy State for project startup screen
#[derive(Constructor, Debug)]
pub struct StateMenuStartup;

/// Bevy plugin for project startup state
#[derive(Debug)]
pub struct StateMenuStartupPlugin;

/// Holds the entity ids of entities spawned by this state
///
/// NOTE: only add entities that need to be cleaned upon exiting the state
#[derive(Debug, Default)]
struct StateMenuStartupEnts {
    ent_txt_menu_main_title: Option<Entity>,
    ent_cam_main: Option<Vec<Entity>>,
    ent_cam_ui: Option<Vec<Entity>>,
}

/// Resources for project startup state
#[derive(Debug)]
pub struct StateMenuStartupResources {
    fnt_bold_fira: Handle<Font>,
}

impl StateMenuStartup {
    /// Spawns loading text entity
    fn spawn_txt_menu_main_title(commands: &mut Commands, res: Res<'_, StateMenuStartupResources>) {
        commands.spawn(TextBundle {
            style: Style {
                // Setup Margin
                size: Size::new(Val::Percent(40.0), Val::Percent(20.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent(20.0),
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "Project Name".to_string(),
                font: res.fnt_bold_fira.clone(),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::BLACK,
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
            },
            ..Default::default()
        });
    }

    fn update(_com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // Do nothing
        println!("Updating");
    }
    fn kill(_com: &mut Commands, _res: Res<'_, StateMenuStartupResources>) {
        // Do nothing
    }
}
impl Plugin for StateMenuStartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<StateMenuStartupResources>()
        .init_resource::<StateMenuStartupEnts>()
            .on_state_enter(
                STAGE_MENU,
                AppStates::Menu,
                StateMenuStartup::spawn_txt_menu_main_title.system(),
            )
            .on_state_update(
                STAGE_MENU,
                AppStates::Menu,
                StateMenuStartup::update.system(),
            )
            .on_state_exit(STAGE_MENU, AppStates::Menu, StateMenuStartup::kill.system());
    }
}

impl FromResources for StateMenuStartupResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let asset_srv = resources.get_mut::<AssetServer>().unwrap();

        Self {
            fnt_bold_fira: asset_srv.load("fnt/FiraSans-Bold.ttf"),
        }
    }
}
