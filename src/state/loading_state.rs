//! Loading screen implementation module

use bevy::prelude::*;

use crate::ecs::Rotating;
use crate::state::{AppStates, STAGE_LOADING};

/// Loading state logic
struct LoadState;

/// Bevy plugin for LoadState
pub struct LoadStatePlugin;

/// Loading state entity id storage
#[derive(Debug, Default)]
struct LoadStateEnts {
    ent_sprite_icon: Option<Entity>,
    ent_sprite_spinner: Option<Entity>,
    ent_txt_main: Option<Entity>,
    ent_cam_main: Option<Vec<Entity>>,
    ent_cam_ui: Option<Vec<Entity>>,
}

/// Resources needed by LoadState
#[allow(unused)]
struct LoadStateRes {
    mat_clr_icon: Handle<ColorMaterial>,
    mat_clr_spinner: Handle<ColorMaterial>,
    fnt_bold_fira: Handle<Font>,
}

// Initialization logic block
impl LoadState {
    /// Spawns an ent w/ a Camera2dBundle component w/ default parameters
    fn spawn_cam_2d(commands: &mut Commands, mut ents: ResMut<'_, LoadStateEnts>) {
        // Spawn 2d camera w/ default configuration
        commands.spawn(Camera2dBundle::default());

        // Store Entity Handle
        ents.ent_cam_main = Some(vec![commands.current_entity().unwrap()]);
    }
    /// Spawns an ent w/ a Camera2dBundle component w/ default parameters
    fn spawn_cam_ui(commands: &mut Commands, mut ents: ResMut<'_, LoadStateEnts>) {
        // Spawn UI camera w/ default configuration
        commands.spawn(CameraUiBundle::default());

        // Store Entity Handle
        ents.ent_cam_main = Some(vec![commands.current_entity().unwrap()]);
    }

    /// Spawns an ent w/ a sprite component in the center of the screen
    fn spawn_sprite_main(
        commands: &mut Commands,
        res: Res<'_, LoadStateRes>,
        mut ents: ResMut<'_, LoadStateEnts>,
    ) {
        // TODO Look into borrowing just needed data

        // Create transform matrix
        let trans_mat =
            Mat4::from_scale_rotation_translation(Vec3::one(), Quat::identity(), Vec3::zero());

        // Spawn sprite using texture from LoadStateRes
        commands.spawn(SpriteBundle {
            material: res.mat_clr_icon.clone(),
            transform: Transform::from_matrix(trans_mat),
            ..Default::default()
        });

        // Register sprite entity
        ents.ent_sprite_icon = Some(commands.current_entity().unwrap());
    }

    /// Spawns progress spinner ent
    fn spawn_sprite_progress_spinner(
        commands: &mut Commands,
        res: Res<'_, LoadStateRes>,
        mut ents: ResMut<'_, LoadStateEnts>,
    ) {
        // Create transform matrix
        let trans_mat = Mat4::from_scale_rotation_translation(
            (0.1, 0.1, 1.0).into(),
            Quat::identity(),
            (0.0, -250.0, 0.0).into(),
        );

        // Spawn spinner
        commands
            .spawn(SpriteBundle {
                material: res.mat_clr_spinner.clone(),
                transform: Transform::from_matrix(trans_mat),
                ..Default::default()
            })
            .with(Rotating(-4.0));

        // Store ID of generated entity
        ents.ent_sprite_spinner = Some(commands.current_entity().unwrap());
    }

    /// Spawns loading text entity
    fn spawn_text_loading(
        commands: &mut Commands,
        res: Res<'_, LoadStateRes>,
        mut ents: ResMut<'_, LoadStateEnts>,
    ) {
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

        // Store ID of currently spawn entity
        ents.ent_txt_main = Some(commands.current_entity().unwrap());
    }

    fn transition_on_load_complete(_com: &mut Commands, mut state: ResMut<'_, State<AppStates>>) {
        // TODO don't hardcode target state
        match state.set_next(AppStates::Menu) {
            Ok(_) => {}
            Err(state_err) => println!("{}", state_err),
        }
    }

    fn update(com: &mut Commands, res: ResMut<'_, State<AppStates>>) {
        Self::transition_on_load_complete(com, res);
    }

    fn kill(commands: &mut Commands, ids: Res<'_, LoadStateEnts>) {
        // Create closure which despawn an ent
        let mut despawn_ent = |ent: Option<Entity>| match ent {
            Some(ent) => {
                commands.despawn_recursive(ent);
            }
            None => {
                warn!("Attempted to delete Entity which has not been initialized.");
            }
        };

        // Despawn entities
        // TODO Do in separate systems
        despawn_ent(ids.ent_sprite_icon);
        despawn_ent(ids.ent_sprite_spinner);
        despawn_ent(ids.ent_txt_main);
    }
}

impl FromResources for LoadStateRes {
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
        app.init_resource::<LoadStateRes>()
            .init_resource::<LoadStateEnts>()
            .on_state_enter(
                STAGE_LOADING,
                AppStates::Load,
                LoadState::spawn_cam_2d.system(),
            )
            .on_state_enter(
                STAGE_LOADING,
                AppStates::Load,
                LoadState::spawn_cam_ui.system(),
            )
            .on_state_enter(
                STAGE_LOADING,
                AppStates::Load,
                LoadState::spawn_sprite_main.system(),
            )
            .on_state_enter(
                STAGE_LOADING,
                AppStates::Load,
                LoadState::spawn_sprite_progress_spinner.system(),
            )
            .on_state_enter(
                STAGE_LOADING,
                AppStates::Load,
                LoadState::spawn_text_loading.system(),
            )
            .on_state_update(STAGE_LOADING, AppStates::Load, LoadState::update.system())
            .on_state_exit(STAGE_LOADING, AppStates::Load, LoadState::kill.system());
    }
}
