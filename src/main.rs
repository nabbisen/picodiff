mod models;

use {
    cascade::cascade,
    fltk::{
        app,
        enums::{CallbackTrigger, Color, Event, Font, FrameType},
        frame::Frame,
        group::Flex,
        input::Input,
        prelude::*,
        text::{StyleTableEntryExt, TextAttr, TextBuffer, TextDisplay},
        window::Window,
    },
    std::{cell::RefCell, rc::Rc},
};

// consts
const PAD: i32 = 10;
const SCROLL: i32 = PAD / 3;
const HEIGHT: i32 = PAD * 3;
const STYLE_TABLE: [StyleTableEntryExt; 3] = [
    StyleTableEntryExt {
        color: Color::Green,
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
    StyleTableEntryExt {
        color: Color::Red,
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
    StyleTableEntryExt {
        color: Color::Blue,
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
];

enum Update {
    Diff = 41,
}

impl Update {
    const fn event(self) -> Event {
        Event::from_i32(self as i32)
    }
}

/// [main] entry point
fn main() -> Result<(), FltkError> {
    let state = Rc::from(RefCell::from(models::Model::default()));
    const UPDATE: Event = Update::Diff.event();
    let app = app::App::default();
    cascade!(
        Window::default().with_size(640, 360).center_screen();
        ..set_label("Pico Diff");
        ..make_resizable(true);
        ..set_callback(move |_| {
            if app::event() == Event::Close {
                app::quit();
            }
        });
        ..add(&cascade!(
            Flex::default_fill().column();
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..fixed(&cascade!(
                Input::default();
                ..set_trigger(CallbackTrigger::Changed);
                ..set_callback(glib::clone!(@strong state => move |input| {
                    state.borrow_mut().set_source(input.value());
                    app::handle_main(UPDATE).unwrap();
                }));
            ), HEIGHT);
            ..fixed(&cascade!(
                Input::default();
                ..set_trigger(CallbackTrigger::Changed);
                ..set_callback(glib::clone!(@strong state => move |input| {
                    state.borrow_mut().set_target(input.value());
                    app::handle_main(UPDATE).unwrap();
                }));
            ), HEIGHT);
            ..add(&cascade!(
                Flex::default().column();
                ..add(&Frame::default());
                ..end();
                ..set_pad(PAD);
                ..set_margin(0);
                ..handle(glib::clone!(@strong state => move |flex, event| {
                    if event == UPDATE {
                        flex.clear();
                        flex.begin();
                        flex.fixed(&cascade!(
                            TextDisplay::default();
                            ..set_scrollbar_size(SCROLL);
                            ..set_buffer(cascade!(
                                TextBuffer::default();
                                ..set_text(state.borrow().source_text());
                            ));
                            ..set_highlight_data_ext(cascade!(
                                TextBuffer::default();
                                ..set_text(state.borrow().source_style());
                            ), STYLE_TABLE);
                            ..set_frame(FrameType::NoBox);
                        ), HEIGHT + SCROLL);
                        flex.fixed(&cascade!(
                            TextDisplay::default();
                            ..set_scrollbar_size(SCROLL);
                            ..set_buffer(cascade!(
                                TextBuffer::default();
                                ..set_text(state.borrow().target_text());
                            ));
                            ..set_highlight_data_ext(cascade!(
                                TextBuffer::default();
                                ..set_text(state.borrow().target_style());
                            ), STYLE_TABLE);
                            ..set_frame(FrameType::NoBox);
                        ), HEIGHT + SCROLL);
                        flex.add(&Frame::default());
                        flex.end();
                        flex.redraw();
                        return true;
                    };
                    false
                }));
                ..end();
            ));
        ));
        ..end();
    )
    .show();
    let color = [
        0xeee8d5, //base2
        0xfdf6e3, //base3
        0x586e75, //base01
        0xcb4b16, //orange
        0xb58900, //yellow
    ];
    let (r, g, b) = Color::from_hex(color[0]).to_rgb();
    app::set_background_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[1]).to_rgb();
    app::set_background2_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[2]).to_rgb();
    app::set_foreground_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[3]).to_rgb();
    app::set_selection_color(r, g, b);
    let (r, g, b) = Color::from_hex(color[4]).to_rgb();
    app::set_inactive_color(r, g, b);
    for (color, hex) in [
        (Color::Yellow, 0xb58900),
        (Color::Red, 0xdc322f),
        (Color::Magenta, 0xd33682),
        (Color::Blue, 0x268bd2),
        (Color::Cyan, 0x2aa198),
        (Color::Green, 0x859900),
    ] {
        let (r, g, b) = Color::from_hex(hex).to_rgb();
        app::set_color(color, r, g, b);
    }
    app.run()
}
