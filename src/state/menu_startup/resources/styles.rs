use bevy::{prelude::*, text::DefaultTextPipeline};

/// UI Style for main menu buttons
#[derive(Debug, Reflect)]
pub struct StateUiResources {
    pub style_node_root: Style,
    pub style_std: Style,
    pub style_popup: Style,
}

impl FromResources for StateUiResources {
    fn from_resources(_resources: &Resources) -> Self {
        Self {
            style_node_root: Style {
                display: Display::Flex,
                // Make children use flex layout
                align_items: AlignItems::FlexStart,
                // Set left margin
                margin: Rect {
                    right: Val::Percent(67.0),
                    ..Default::default()
                },
                // Make children lay out in reverse column order
                flex_direction: FlexDirection::ColumnReverse,

                ..Default::default()
            },
            style_std: Style {
                // Set button size
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexStart,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            style_popup: Style {
                // Set size
                size: Size::new(Val::Percent(60.0), Val::Percent(60.0)),
                // Set position
                position_type: PositionType::Absolute,
                position: Rect::all(Val::Percent(20.0)),
                ..Default::default()
            }
        }
    }
}
