//! Loading screen implementation module

use bevy::ecs::StateError;
use bevy::prelude::*;

use crate::ecs::Rotating;
use crate::state::{AppStates, STAGE_LOADING};

/// Loading state logic
struct LoadState;

/// Bevy plugin for LoadState
pub struct LoadStatePlugin;

/// Loading state entity id storage
struct LoadStateData {
    ent_sprite_icon: Entity,
    ent_sprite_spinner: Entity,
    ent_txt_main: Entity,
}

/// Resources needed by LoadState
#[allow(unused)]
struct LoadStateResources {
    pub(crate) mat_clr_icon: Handle<ColorMaterial>,
    pub(crate) mat_clr_spinner: Handle<ColorMaterial>,
    pub(crate) fnt_bold_fira: Handle<Font>,
}

// Initialization logic block
impl LoadState {
    /// Creates entities for the LoadState
    fn spawn(com: &mut Commands, res: Res<'_, LoadStateResources>) {
        let data = LoadStateData {
            ent_sprite_icon: Self::spawn_sprite_progress_spinner(com, res.mat_clr_spinner.clone()),
            ent_sprite_spinner: Self::spawn_text_loading(com, res.fnt_bold_fira.clone()),
            ent_txt_main: Self::spawn_sprite_main(com, res.mat_clr_icon.clone()),
        };

        // Spawn entities and store the ent ID's
        com.insert_resource(data);

        super::world::setup_world(com);
    }

    /// Spawns an ent w/ a sprite component in the center of the screen
    fn spawn_sprite_main(commands: &mut Commands, icon: Handle<ColorMaterial>) -> Entity {
        // Create transform matrix
        let trans_mat =
            Mat4::from_scale_rotation_translation(Vec3::one(), Quat::identity(), Vec3::zero());

        // Spawn sprite using provided texture
        commands.spawn(SpriteBundle {
            material: icon,
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        });

        // Return ID of spawned entity
        commands.current_entity().unwrap()
    }

    /// Spawns progress spinner ent
    fn spawn_sprite_progress_spinner(
        commands: &mut Commands,
        spinner: Handle<ColorMaterial>,
    ) -> Entity {
        // Create transform matrix
        let trans_mat = Mat4::from_scale_rotation_translation(
            (0.1, 0.1, 1.0).into(),
            Quat::identity(),
            (0.0, -250.0, 0.0).into(),
        );

        // Spawn spinner
        commands
            .spawn(SpriteBundle {
                material: spinner,
                transform: Transform::from_matrix(trans_mat),
                ..Default::default()
            })
            .with(Rotating(-4.0));
        // Return ID of spawned entity
        commands.current_entity().unwrap()
    }

    /// Spawns loading text entity
    fn spawn_text_loading(commands: &mut Commands, font: Handle<Font>) -> Entity {
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
                value: format!("TDGame version {}", env!("CARGO_PKG_VERSION")),
                font,
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

        // Return ID of spawned entity
        commands.current_entity().unwrap()
    }

    fn transtion_on_load_complete(
        _com: &mut Commands,
        mut state: ResMut<'_, State<super::AppStates>>,
    ) {
        // TODO don't hardcode target state
        match state.set_next(super::AppStates::Menu) {
            Ok(_) => {}
            Err(state_err) => println!("{:?}", state_err),
        }
    }

    fn update(com: &mut Commands, res: ResMut<'_, State<super::AppStates>>) {
        Self::transtion_on_load_complete(com, res);
    }

    fn kill(com: &mut Commands, _res: Res<'_, LoadStateResources>) {}
}
impl FromResources for LoadStateResources {
    fn from_resources(resources: &Resources) -> Self {
        // Get engine stores
        let mut res_mat_clr = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_srv = resources.get_mut::<AssetServer>().unwrap();

        // Load assets
        Self {
            mat_clr_icon: res_mat_clr.add(asset_srv.load("tex/icon.png").into()),
            mat_clr_spinner: res_mat_clr.add(asset_srv.load("tex/spinner.png").into()),
            fnt_bold_fira: asset_srv.load("fnt/FiraSans-Bold.ttf"),
        }
    }
}

impl Plugin for LoadStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LoadStateResources>()
            .on_state_enter(STAGE_LOADING, AppStates::Load, LoadState::spawn.system())
            .on_state_update(STAGE_LOADING, AppStates::Load, LoadState::update.system())
            .on_state_exit(STAGE_LOADING, AppStates::Load, LoadState::kill.system());
    }
}
