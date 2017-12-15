use gdk_pixbuf;
use gdk::ContextExt;

use gtk;
use gtk::ContainerExt;
use gtk::StyleContextExt;
use gtk::WidgetExt;

use pango;
use pango::LayoutExt;

use unicode_segmentation::UnicodeSegmentation;

use std::collections::hash_map::DefaultHasher;
use std::f64;
use std::hash::{Hash, Hasher};

const KR: f64 = 0.299;
const KG: f64 = 0.587;
const KB: f64 = 0.114;
const Y: f64 = 0.731;
const BLEND_FACTOR: f64 = 0.2;

pub fn avatar<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(name: &str, avatar: P) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let h = hasher.finish();

    let angle = ((h & (0xffff << 48)) >> 48) as f64 / 65536.0 * 2.0 * f64::consts::PI;
    let (cb, cr) = angle.sin_cos();
    let factor = cr.abs().max(cb.abs());
    let (cb, cr) = (cb * factor, cr * factor);
    let r = 2.0 * (1.0 - KR) * cr + Y;
    let b = 2.0 * (1.0 - KB) * cb + Y;
    let g = (Y - KR * r - KB * b) / KG;

    let mut font = pango::FontDescription::new();
    font.set_size(48);

    let drawing = gtk::DrawingArea::new();
    drawing.set_size_request(128, 128);
    if let Some(pic) = avatar.into() {
        drawing.connect_draw(clone!( pic => move |_, ctx| {
            ctx.set_source_pixbuf(&pic, 0.0, 0.0);
            ctx.paint();
            gtk::Inhibit(true)
        }));
    } else {
        let first = UnicodeSegmentation::graphemes(name, true)
            .nth(0)
            .unwrap_or_default()
            .to_owned();
        drawing.connect_draw(move |widget, ctx| {
            // If we can get a style_context, blend the color with the background.
            let (rb, bb, gb) = match widget.get_style_context() {
                Some(style_context) => {
                    let bg = style_context.get_background_color(style_context.get_state());
                    let r = BLEND_FACTOR * (1.0 - bg.red) + (1.0 - BLEND_FACTOR) * r;
                    let b = BLEND_FACTOR * (1.0 - bg.blue) + (1.0 - BLEND_FACTOR) * b;
                    let g = BLEND_FACTOR * (1.0 - bg.green) + (1.0 - BLEND_FACTOR) * g;
                    (r, b, g)
                }
                None => (r, b, g),
            };
            ctx.set_source_rgb(rb, gb, bb);
            ctx.paint();

            if !first.is_empty() {
                if let Some(layout) = widget.create_pango_layout(&first[..]) {
                    if let Some(style_context) = widget.get_style_context() {
                        layout.set_font_description(&font);
                        ctx.set_source_rgb(0.0, 0.0, 0.0);
                        let (textw, texth) = layout.get_pixel_size();
                        let (width, height) = (widget.get_allocated_width(), widget.get_allocated_height());
                        ctx.move_to(f64::from(width - textw) / 2.0, f64::from(height - texth) / 2.0);
                        gtk::render_layout(
                            &style_context,
                            ctx,
                            0.0,
                            0.0,
                            &layout,
                        );
                    }
                }
            }
            gtk::Inhibit(true)
        });
    }

    let name_label = gtk::Label::new(name);

    vbox.add(&drawing);
    vbox.add(&name_label);
    vbox
}
