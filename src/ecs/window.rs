use bevy::{
    prelude::*,
    window::{CreateWindow, WindowResized},
};

#[derive(Default)]
pub struct WindowResizeEventListenerState {
    pub resolution: Option<[f32; 2]>,
    pub create_events: EventReader<CreateWindow>,
    pub resize_events: EventReader<WindowResized>,
}

pub fn window_resolution_system(
    mut state: ResMut<WindowResizeEventListenerState>,
    create_events: ResMut<Events<CreateWindow>>,
    resize_events: ResMut<Events<WindowResized>>,
) {
    for event in state.create_events.iter(&create_events) {
        state.resolution = Some([event.descriptor.width, event.descriptor.height]);
    }

    for event in state.resize_events.iter(&resize_events) {
        state.resolution = Some([event.width, event.height]);
    }
}
