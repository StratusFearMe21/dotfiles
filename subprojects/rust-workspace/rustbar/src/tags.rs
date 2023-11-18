use std::sync::Arc;

use iced_tiny_skia::{
    core::{
        alignment::{Horizontal, Vertical},
        text::{LineHeight, Shaping},
        Background, Color, Rectangle,
    },
    graphics::backend::Text,
    /* Custom ,*/ Primitive,
};
use tiny_skia::{Paint, PathBuilder, Stroke, Transform};

use crate::znet_dwl::znet_tapesoftware_dwl_wm_monitor_v1::TagState;

#[derive(Clone, Copy)]
pub struct Tag {
    pub state: TagState,
    pub num_clients: u32,
    pub focused_client: i32,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            state: TagState::None,
            num_clients: 0,
            focused_client: 0,
        }
    }
}

pub struct Tags {
    pub tags_background: Arc<Primitive>,
    pub primitives: Arc<Primitive>,
    pub tag_windows: Arc<Primitive>,
    pub tags: Vec<Tag>,
    pub width: f32,
    pub num_width: f32,
}

impl Default for Tags {
    fn default() -> Self {
        Self {
            tags_background: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            primitives: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            tag_windows: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            tags: Vec::new(),
            num_width: 0.0,
            width: 0.0,
        }
    }
}

impl Tags {
    pub fn relayout(
        &mut self,
        padding_x: f32,
        height: f32,
        backend: &iced_tiny_skia::Backend,
        ascii_num_width: f32,
        tag_count: usize,
    ) {
        let mut primitives = Vec::new();

        let num_width = (padding_x * 2.0) + ascii_num_width;

        for tag in 0..tag_count {
            primitives.push(Primitive::Text {
                content: (tag + 1).to_string(),
                bounds: Rectangle {
                    x: (tag as f32 * num_width) + padding_x,
                    y: height / 2.0,
                    width: num_width,
                    height,
                },
                color: Color::BLACK,
                size: backend.default_size(),
                line_height: LineHeight::Relative(1.0),
                font: backend.default_font(),
                horizontal_alignment: Horizontal::Left,
                vertical_alignment: Vertical::Center,
                shaping: Shaping::Basic,
            });
        }

        self.primitives = Arc::new(Primitive::Group { primitives });
        self.num_width = num_width;
        self.width = tag_count as f32 * num_width;
    }
    pub fn relayout_bg(
        &mut self,
        color_inactive: (Color, Color),
        color_active: (Color, Color),
        bar_size: f32,
    ) {
        let mut primitives = Vec::new();

        let mut x1 = 0.0;
        let mut x2 = 0.0;
        let mut in_selection = false;
        for (tag, primitive) in self
            .tags
            .iter()
            .zip(match Arc::make_mut(&mut self.primitives) {
                Primitive::Group { primitives } => primitives.iter_mut(),
                _ => unsafe { std::hint::unreachable_unchecked() },
            })
        {
            if matches!(tag.state, TagState::Active | TagState::Urgent) {
                if !in_selection {
                    in_selection = true;
                    primitives.push(Primitive::Quad {
                        bounds: Rectangle {
                            x: x1,
                            y: 0.0,
                            width: x2 - x1,
                            height: bar_size,
                        },
                        background: Background::Color(color_inactive.1),
                        border_radius: [0.0, 0.0, 0.0, 0.0],
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    });
                    x1 = x2;
                }
                x2 += self.num_width;
                match primitive {
                    Primitive::Text { color, .. } => {
                        *color = color_active.0;
                    }
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            } else {
                if in_selection {
                    in_selection = false;
                    x1 = x2;
                }
                x2 += self.num_width;
                match primitive {
                    Primitive::Text { color, .. } => {
                        *color = color_inactive.0;
                    }
                    _ => unsafe { std::hint::unreachable_unchecked() },
                }
            }
        }
        if !in_selection {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x: x1,
                    y: 0.0,
                    width: x2 - x1,
                    height: bar_size,
                },
                background: Background::Color(color_inactive.1),
                border_radius: [0.0, 0.0, 0.0, 0.0],
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }

        self.tags_background = Arc::new(Primitive::Group { primitives });
    }
    pub fn relayout_windows(&mut self, color_active: Color, color_inactive: Color, padding_x: f32) {
        let mut primitives = Vec::new();

        for (val, tag) in self.tags.iter().enumerate() {
            let pos_x = val as f32 * self.num_width;
            let stroke = Stroke {
                width: 1.0,
                ..Default::default()
            };
            let mut paint = Paint::default();
            paint.anti_alias = false;
            if matches!(tag.state, TagState::Active | TagState::Urgent) {
                paint.set_color(
                    tiny_skia::Color::from_rgba(
                        color_active.b,
                        color_active.g,
                        color_active.r,
                        1.0,
                    )
                    .unwrap(),
                );
            } else {
                paint.set_color(
                    tiny_skia::Color::from_rgba(
                        color_inactive.b,
                        color_inactive.g,
                        color_inactive.r,
                        1.0,
                    )
                    .unwrap(),
                );
            }

            /*
            let mut path = PathBuilder::new();

            for client in 0..tag.num_clients {
                let pos_y = client as f32 * 2.0;
                path.move_to(pos_x, pos_y + 0.0001);
                if tag.focused_client >= 0 && tag.focused_client as u32 == client {
                    path.line_to(pos_x + padding_x, pos_y);
                } else {
                    path.line_to(pos_x + 1.0, pos_y);
                }
            }

            if let Some(path) = path.finish() {
                primitives.push(Primitive::Custom(Custom::Stroke {
                    path,
                    paint,
                    stroke,
                    transform: Transform::default(),
                }));
            }
            */
        }

        self.tag_windows = Arc::new(Primitive::Group { primitives });
    }
    pub fn new(
        tag_count: usize,
        padding_x: f32,
        height: f32,
        ascii_num_width: f32,
        backend: &iced_tiny_skia::Backend,
    ) -> Self {
        let mut tags = Self {
            tags_background: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            primitives: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            tag_windows: Arc::new(Primitive::Group {
                primitives: Vec::new(),
            }),
            tags: vec![Tag::default(); tag_count],
            num_width: 0.0,
            width: 0.0,
        };

        tags.relayout(padding_x, height, backend, ascii_num_width, tag_count);

        tags
    }

    pub fn tag_event(
        &mut self,
        tag: u32,
        tag_state: TagState,
        num_clients: u32,
        focused_client: i32,
        color_inactive: (Color, Color),
        color_active: (Color, Color),
        bar_size: f32,
        padding_x: f32,
    ) {
        let new_tag = self.tags.get_mut(tag as usize).unwrap();

        new_tag.state = tag_state;
        new_tag.num_clients = num_clients;
        new_tag.focused_client = focused_client;

        self.relayout_bg(color_inactive, color_active, bar_size);
        self.relayout_windows(color_active.0, color_inactive.0, padding_x);
    }
}
