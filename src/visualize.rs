use std::thread;
use async_channel::Sender;
use gtk4::{glib, Align, Application, ApplicationWindow, Label, Orientation, Picture};
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::prelude::*;
use image::Rgb;
use crate::generate_image;

pub type ShowPixelMessage = (u32, u32, Rgb<u8>);

pub fn show(width: i32, height: i32, f: impl Fn(Sender<ShowPixelMessage>) + Copy + Send + 'static) {
    let app = Application::builder()
        .application_id("org.example.RealtimeRenderer")
        .build();

    app.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(width)
            .default_height(height)
            .title("Realtime Renderer")
            .build();

        // Create an initial empty Pixbuf for the Image widget.
        let pixbuf = Pixbuf::new(Colorspace::Rgb, false, 8, width, height).unwrap();
        let image_widget = Picture::for_pixbuf(&pixbuf);

        image_widget.set_hexpand(true);
        image_widget.set_vexpand(true);
        image_widget.set_halign(Align::Fill);
        image_widget.set_valign(Align::Fill);
        image_widget.set_vexpand(true);

        let container = gtk4::Box::new(Orientation::Vertical, 0);
        container.append(&Label::new(Some("hello world")));
        container.append(&image_widget);
        container.set_hexpand(true);
        container.set_vexpand(true);

        window.set_child(Some(&container));
        window.present();

        glib::MainContext::default().spawn_local(async move {
            let (tx, rx) = async_channel::unbounded::<(u32, u32, Rgb<u8>)>();

            thread::spawn(move || {
                f(tx)
            });

            while let Ok((x, y, Rgb([r,g,b]))) = rx.recv().await {
                pixbuf.put_pixel(x, y, r, g, b, 0);
                image_widget.set_pixbuf(Some(&pixbuf));
            }
        });
    });

    app.run();

    ()
}
