use std::time::Duration;
use async_channel::Sender;
use gtk4::{gio, glib, Align, Application, ApplicationWindow, Label, Orientation, Picture};
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::glib::timeout_future;
use gtk4::prelude::*;
use image::Rgb;

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

        glib::spawn_future_local(async move {
            let (tx, rx) = async_channel::unbounded::<(u32, u32, Rgb<u8>)>();

            gio::spawn_blocking(move || {
                f(tx)
            });

            while let Ok((x, y, Rgb([r, g, b]))) = rx.recv().await {
                // Apply the initial received pixel
                pixbuf.put_pixel(x, y, r, g, b, 0);

                // Process up to 100,000 more pixels without blocking
                for _ in 0..100_000 {
                    match rx.try_recv() {
                        Ok((x, y, Rgb([r, g, b]))) => pixbuf.put_pixel(x, y, r, g, b, 0),
                        Err(_) => break, // Exit if there are no more messages immediately available
                    }
                }

                // Update the image widget with the new pixbuf state
                image_widget.set_pixbuf(Some(&pixbuf));

                // Yield control to GTK to keep the UI responsive
                timeout_future(Duration::from_millis(1)).await;
            }
        });
    });

    app.run();

    ()
}
