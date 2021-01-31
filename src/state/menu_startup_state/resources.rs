use bevy::prelude::*;

/// Holds the entity ids of entities spawned by this state
///
/// NOTE: only add entities that need to be cleaned upon exiting the state
#[derive(Debug, Default)]
pub struct StateMenuStartupEnts {
    pub ent_txt_menu_main_title: Option<Entity>,
    pub ent_sprite_icon: Option<Entity>,
    pub ent_cam_main: Option<Vec<Entity>>,
    pub ent_cam_ui: Option<Vec<Entity>>,
    pub ent_button_game_new: Option<Entity>,
}

/// Resources for project startup state
#[derive(Debug)]
pub struct StateMenuStartupResources {
    pub fnt_bold_fira: Handle<Font>,
    pub mat_clr_icon: Handle<ColorMaterial>,
}

impl FromResources for StateMenuStartupResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let asset_srv = resources.get_mut::<AssetServer>().unwrap();
        let mut res_mat_clr = resources.get_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            fnt_bold_fira: asset_srv.load("fnt/FiraSans-Bold.ttf"),
            mat_clr_icon: res_mat_clr.add(asset_srv.load("tex/icon.png").into()),
        }
    }
}

/// List of buttons in main menu
// TODO Bevy reflection
#[derive(Debug)]
pub enum MenuStartupButtons {
    Continue,
    NewGame,
    LoadGame,
    Options,
    Exit,
}

/// UI Style for main menu buttons
#[derive(Debug, Reflect)]
pub struct StyleMenuUiButton {
    pub style_std: Style,
}

impl FromResources for StyleMenuUiButton {
    fn from_resources(_resources: &Resources) -> Self {
        Self {
            style_std: Style {
                // Set button size
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        }
    }
}

/// Returns a closure that spawns a TextBundle from a parent
// TODO Give more control over resulting text
pub fn create_child_txt_builder(
    text: String,
    fnt: Handle<Font>,
) -> impl FnOnce(&mut ChildBuilder<'_>) {
    |parent| {
        parent.spawn(TextBundle {
            text: Text::with_section(
                text,
                TextStyle {
                    font: fnt,
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }
}
