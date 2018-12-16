use cairo;
use gdk::ContextExt;
use gdk_pixbuf;
use gtk;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::StyleContextExt;
use gtk::WidgetExt;
use unicode_segmentation::UnicodeSegmentation;

use std::collections::hash_map::DefaultHasher;
use std::f64;
use std::hash::{Hash, Hasher};

use crate::res;

const KR: f64 = 0.299;
const KG: f64 = 0.587;
const KB: f64 = 0.114;
const Y: f64 = 0.731;
const BLEND_FACTOR: f64 = 0.4;

pub fn avatar<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(name: &str, avatar: P) -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 12);
    if let Some(style_context) = vbox.get_style_context() {
        style_context.add_class("avatar");
        let provider = gtk::CssProvider::new();
        match provider.load_from_data(res::STYLE_AVATAR) {
            Ok(_) => {
                style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION)
            }
            Err(err) => eprintln!("error loading style provider for list: {}", err),
        }
    }

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
            // If we can get a style_context, blend the color with the background color.
            let (width, height) = (widget.get_allocated_width(), widget.get_allocated_height());
            let size = f64::from(width.min(height));
            let half = size / 2.0;
            match widget.get_style_context() {
                Some(style_context) => gtk::render_background(
                    &style_context,
                    &ctx,
                    f64::from(width) / 2.0 - half,
                    f64::from(height) / 2.0 - half,
                    size,
                    size,
                ),
                None => {}
            };
            ctx.set_source_rgba(r, g, b, 1.0 - BLEND_FACTOR);
            ctx.rectangle(
                f64::from(width) / 2.0 - half,
                f64::from(height) / 2.0 - half,
                size,
                size,
            );
            ctx.fill();

            if !first.is_empty() {
                ctx.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                ctx.set_font_size(48.0);
                let extents = ctx.text_extents(&first[..]);
                let x = f64::from(width) / 2.0 - (extents.width / 2.0 + extents.x_bearing);
                let y = f64::from(height) / 2.0 - (extents.height / 2.0 + extents.y_bearing);
                ctx.move_to(x, y);
                ctx.set_source_rgb(1.0, 1.0, 1.0);
                // TODO: Use pango for this, otherwise complex graphemes likely won't be shown
                // correctly.
                ctx.show_text(&first[..]);
            }
            gtk::Inhibit(true)
        });
    }

    let name_label = gtk::Label::new(name);

    vbox.add(&drawing);
    vbox.add(&name_label);
    vbox
}
