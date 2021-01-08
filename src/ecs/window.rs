use bevy::{
    prelude::*,
    window::{CreateWindow, WindowResized},
};

/// Bevy plugin for handling window information
pub struct ECSWindowPlug;

impl Plugin for ECSWindowPlug {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WindowResizeEventListenerState::default())
            .add_system(window_resolution_system.system());
    }
}

#[derive(Default)]
pub struct WindowResizeEventListenerState {
    pub resolution: Option<[f32; 2]>,
    pub create_events: EventReader<CreateWindow>,
    pub resize_events: EventReader<WindowResized>,
}

pub fn window_resolution_system(
    mut state: ResMut<'_, WindowResizeEventListenerState>,
    create_events: ResMut<'_, Events<CreateWindow>>,
    resize_events: ResMut<'_, Events<WindowResized>>,
) {
    for event in state.create_events.iter(&create_events) {
        state.resolution = Some([event.descriptor.width, event.descriptor.height]);
    }

    for event in state.resize_events.iter(&resize_events) {
        state.resolution = Some([event.width, event.height]);
    }
}
