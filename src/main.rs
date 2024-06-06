use fltk::{
    app::{App, Scheme},
    enums::{CallbackTrigger, Color, Font, FrameType},
    group::*,
    input::Input,
    prelude::*,
    text::{StyleTableEntryExt, TextAttr, TextBuffer, TextDisplay},
    window::Window,
};

use similar::{utils::diff_chars, Algorithm, ChangeTag};

// consts
const INPUT_OUTPUT_WIDTH: i32 = 360;
const INPUT_HEIGHT: i32 = 30;
const OUTPUT_HEIGHT: i32 = 40;
const STYLE_TABLE: [StyleTableEntryExt; 3] = [
    StyleTableEntryExt {
        color: Color::from_hex(0x000000),
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
    StyleTableEntryExt {
        color: Color::from_hex(0x00ff00),
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
    StyleTableEntryExt {
        color: Color::from_hex(0xff0000),
        font: Font::Courier,
        size: 16,
        attr: TextAttr::None,
        bgcolor: Color::TransparentBg,
    },
];

/// [main] entry point
fn main() {
    let app = App::default().with_scheme(Scheme::Base);

    let mut window = Window::default()
        .with_size(
            INPUT_OUTPUT_WIDTH + 28,
            INPUT_HEIGHT * 2 + OUTPUT_HEIGHT * 2 + 28,
        )
        .with_label("Pico Diff");
    window.set_color(Color::White);

    let mut vpack = Pack::new(
        4,
        4,
        INPUT_OUTPUT_WIDTH + 20,
        INPUT_HEIGHT * 2 + OUTPUT_HEIGHT * 2 + 20,
        "",
    );
    vpack.set_spacing(4);
    let mut source_input = Input::default().with_size(INPUT_OUTPUT_WIDTH, INPUT_HEIGHT);
    source_input.set_color(Color::from_hex(0xffeecc));
    let mut target_input = Input::default().with_size(INPUT_OUTPUT_WIDTH, INPUT_HEIGHT);
    target_input.set_color(Color::from_hex(0xccffee));

    #[allow(unused_mut)]
    let mut diff_pack = Pack::new(4, 4, INPUT_OUTPUT_WIDTH, 220, "");
    diff_pack.end();

    vpack.end();

    // wind.make_resizable(true);
    window.end();
    window.show();

    source_input.set_trigger(CallbackTrigger::Changed);
    let (s1, t1, d1) = (
        source_input.clone(),
        target_input.clone(),
        diff_pack.clone(),
    );
    source_input.set_callback(move |_| {
        diff(
            s1.clone().value().as_str(),
            t1.clone().value().as_str(),
            &mut d1.clone(),
        );
    });
    target_input.set_trigger(CallbackTrigger::Changed);
    let (s2, t2, d2) = (
        source_input.clone(),
        target_input.clone(),
        diff_pack.clone(),
    );
    target_input.set_callback(move |_| {
        diff(
            s2.clone().value().as_str(),
            t2.clone().value().as_str(),
            &mut d2.clone(),
        )
    });

    app.run().unwrap();
}

/// text comparison
fn diff(source: &str, target: &str, diff_pack: &mut Pack) {
    let (source_text, source_style, target_text, target_style) = diff_text_buffers(source, target);

    let mut vpack = Pack::new(4, 4, INPUT_OUTPUT_WIDTH + 20, OUTPUT_HEIGHT * 2 + 20, "");
    vpack.set_spacing(4);

    let mut source_display = TextDisplay::default().with_size(INPUT_OUTPUT_WIDTH, OUTPUT_HEIGHT);
    source_display.set_buffer(source_text);
    source_display.set_highlight_data_ext(source_style, STYLE_TABLE);
    source_display.set_frame(FrameType::NoBox);
    let mut target_display = TextDisplay::default().with_size(INPUT_OUTPUT_WIDTH, OUTPUT_HEIGHT);
    target_display.set_buffer(target_text);
    target_display.set_highlight_data_ext(target_style, STYLE_TABLE);
    target_display.set_frame(FrameType::NoBox);

    vpack.end();

    diff_pack.clear();
    diff_pack.add(&vpack);
    diff_pack.redraw();
}

/// text buffers on source display and target display
fn diff_text_buffers(
    source: &str,
    target: &str,
) -> (TextBuffer, TextBuffer, TextBuffer, TextBuffer) {
    let mut source_text = TextBuffer::default();
    let mut source_style = TextBuffer::default();
    let mut target_text = TextBuffer::default();
    let mut target_style = TextBuffer::default();

    let diffs = diff_chars(Algorithm::Myers, source, target);
    diffs.iter().for_each(|(change_tag, str)| match change_tag {
        ChangeTag::Delete => {
            source_text.append(str);
            source_style.append("C".repeat(str.len()).as_str());
            target_text.append(" ".repeat(str.len()).as_str());
            target_style.append(" ".repeat(str.len()).as_str());
        }
        ChangeTag::Insert => {
            source_text.append(" ".repeat(str.len()).as_str());
            source_style.append(" ".repeat(str.len()).as_str());
            target_text.append(str);
            target_style.append("B".repeat(str.len()).as_str());
        }
        ChangeTag::Equal => {
            source_text.append(str);
            source_style.append("A".repeat(str.len()).as_str());
            target_text.append(str);
            target_style.append("A".repeat(str.len()).as_str());
        }
    });
    (source_text, source_style, target_text, target_style)
}
