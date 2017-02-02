// Copyright 2017 ttf-noto-sans Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! ttf-noto-sans example program

#![warn(bad_style)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#[macro_use]
extern crate conrod;
extern crate ttf_noto_sans;
extern crate piston_window;

use conrod::{Colorable, Positionable, Scalar, UiBuilder, UiCell};
use conrod::backend::piston::draw;
use conrod::backend::piston::event::{self, UpdateEvent};
use conrod::color::WHITE;
use conrod::text::FontCollection;
use conrod::text::rt::Rect;
use conrod::widget::{Canvas, Text, Widget};
use piston_window::{G2d, G2dTexture, OpenGL, PistonWindow, TextureSettings, Window, WindowSettings};
use piston_window::texture::{Format, UpdateTexture};

widget_ids!(struct Ids { canvas, text });

fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut window: PistonWindow = WindowSettings::new("Hello", [WIDTH, HEIGHT])
        .opengl(OpenGL::V3_2)
        .samples(4)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    let ids = Ids::new(ui.widget_id_generator());
    let _ = ui.fonts.insert(FontCollection::from_bytes(ttf_noto_sans::REGULAR)
        .into_font()
        .expect("failed to into_font"));

    let mut text_vertex_data = Vec::new();
    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache =
            conrod::text::GlyphCache::new(WIDTH, HEIGHT, SCALE_TOLERANCE, POSITION_TOLERANCE);
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture = G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings)
            .unwrap();
        (cache, texture)
    };


    let image_map = conrod::image::Map::new();

    while let Some(event) = window.next() {
        let size = window.size();
        let (win_w, win_h) = (size.width as Scalar, size.height as Scalar);
        if let Some(e) = event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        let _ = event.update(|_| {
            let mut ui = ui.set_widgets();
            set_widget(&mut ui, &ids);
        });

        let _ = window.draw_2d(&event, |context, graphics| if let Some(primitives) =
            ui.draw_if_changed() {
            // A function used for caching glyphs to the texture cache.
            let cache_queued_glyphs = |graphics: &mut G2d,
                                       cache: &mut G2dTexture,
                                       rect: Rect<u32>,
                                       data: &[u8]| {
                let offset = [rect.min.x, rect.min.y];
                let size = [rect.width(), rect.height()];
                let format = Format::Rgba8;
                let encoder = &mut graphics.encoder;
                text_vertex_data.clear();
                text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                UpdateTexture::update(cache, encoder, format, &text_vertex_data[..], offset, size)
                    .expect("failed to update texture")
            };

            fn texture_from_image<T>(img: &T) -> &T {
                img
            }

            draw::primitives(primitives,
                             context,
                             graphics,
                             &mut text_texture_cache,
                             &mut glyph_cache,
                             &image_map,
                             cache_queued_glyphs,
                             texture_from_image);
        });
    }
}

fn set_widget(ui: &mut UiCell, ids: &Ids) {
    Canvas::new().color(WHITE).set(ids.canvas, ui);
    Text::new("HELLO!").font_size(100).middle_of(ids.canvas).set(ids.text, ui);
}