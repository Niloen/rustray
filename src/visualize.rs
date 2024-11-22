use std::time::Duration;
use async_channel::{Receiver, Sender};
use gtk4::{gio, glib, Align, Application, ApplicationWindow, Label, Orientation, Picture};
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::glib::{timeout_future, Bytes};
use gtk4::prelude::*;
use image::{Rgb, RgbImage};

pub enum ShowMessage {
    ShowPixelMessage(u32, u32, Rgb<u8>),
    ShowImage(RgbImage)
}

struct Processor {
    pub pixbuf: Pixbuf,
    dirty: bool,
    picture: Picture,
}

fn create_pixbuf_from_rgb_image(image: RgbImage) -> Pixbuf {
    // Convert the RgbImage into a raw byte buffer
    let (width, height) = image.dimensions();
    let stride = 3 * width as i32; // 3 bytes per pixel (R, G, B)

    // Convert the raw buffer into a `glib::Bytes`
    let raw_buffer = image.into_raw(); // Consumes the RgbImage and returns a Vec<u8>
    let buffer = Bytes::from(&raw_buffer[..]); // Create `glib::Bytes` from the slice

    // Create the Pixbuf from the buffer
    Pixbuf::from_bytes(&buffer, Colorspace::Rgb, false, 8, width as i32, height as i32, stride)
}

impl Processor {
    fn new(width: i32, height: i32) -> Self {
        let pixbuf = Pixbuf::new(Colorspace::Rgb, false, 8, width, height).unwrap();
        let picture = Picture::for_pixbuf(&pixbuf);
        
        Processor {
            pixbuf,
            picture,
            dirty: false
        }
    }
    
    fn put_pixel(&mut self, x: u32, y: u32, color: Rgb<u8>) {
        let Rgb([r, g, b]) = color;
        self.pixbuf.put_pixel(x, y, r, g, b, 0);
        self.dirty = true;
    }
    
    fn process(&mut self, message: ShowMessage, rx: &Receiver<ShowMessage>) {
        match message {
            ShowMessage::ShowPixelMessage(x, y, Rgb([r, g, b])) => {
                self.pixbuf.put_pixel(x, y, r, g, b, 0);

                // Process up to 100,000 more pixels without blocking
                for _ in 0..100_000 {
                    match rx.try_recv() {
                        Ok(ShowMessage::ShowPixelMessage(x, y, c)) => self.put_pixel(x, y, c),
                        Ok(other) => {
                            self.flush_pixels();
                            self.process(other, rx);
                        }
                        Err(_) => break, // Exit if there are no more messages immediately available
                    }
                }
                self.flush_pixels();
            }
            ShowMessage::ShowImage(image) => {
                self.pixbuf = create_pixbuf_from_rgb_image(image);
                self.dirty = true;
                self.flush_pixels();
            }
        }
    }
    
    fn flush_pixels(&mut self) {
        if self.dirty {
            self.dirty = false;
            self.picture.set_pixbuf(Some(&self.pixbuf));
        }
    }
}
pub fn show(width: i32, height: i32, f: impl Fn(Sender<ShowMessage>) + Copy + Send + 'static) {
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

        let mut processor = Processor::new(width, height);

        let image_widget = &processor.picture;
        image_widget.set_hexpand(true);
        image_widget.set_vexpand(true);
        image_widget.set_halign(Align::Fill);
        image_widget.set_valign(Align::Fill);
        image_widget.set_vexpand(true);

        let container = gtk4::Box::new(Orientation::Vertical, 0);
        container.append(&Label::new(Some("hello world")));
        container.append(image_widget);
        container.set_hexpand(true);
        container.set_vexpand(true);

        window.set_child(Some(&container));
        window.present();

        glib::spawn_future_local(async move {
            let (tx, rx) = async_channel::unbounded::<ShowMessage>();

            gio::spawn_blocking(move || {
                f(tx)
            });

            while let Ok(message) = rx.recv().await {
                processor.process(message, &rx);

                // Yield control to GTK to keep the UI responsive
                timeout_future(Duration::from_millis(1)).await;
            }
        });
    });

    app.run();

    ()
}
