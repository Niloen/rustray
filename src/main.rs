use std::thread;
use std::time::Instant;
use async_channel::{Sender};
use crate::camera::Camera;
use crate::vector::Vector3;
use crate::world::ray::Ray;
use crate::world::sphere::Sphere;
use crate::world::World;
use image::{Rgb};
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::{Orientation, Box, Label, Align, Picture};
mod world;
mod vector;
mod camera;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.RealtimeRenderer")
        .build();

    let width = 1024;
    let height = 768;
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

        let container = Box::new(Orientation::Vertical, 0);
        container.append(&Label::new(Some("hello world")));
        container.append(&image_widget);
        container.set_hexpand(true);
        container.set_vexpand(true);

        window.set_child(Some(&container));
        window.present();

        glib::MainContext::default().spawn_local(async move {
            let (tx, rx) = async_channel::unbounded::<(u32, u32, Rgb<u8>)>();

            thread::spawn(move || {
                generate_image(width as u32, height as u32, tx)
            });

            while let Ok((x, y, Rgb([r,g,b]))) = rx.recv().await {
                pixbuf.put_pixel(x, y, r, g, b, 0);
                image_widget.set_pixbuf(Some(&pixbuf));
            }
        });
    });

    app.run()
}

fn generate_image(width: u32, height: u32, tx: Sender<(u32, u32, Rgb<u8>)>) {
    let mut world = World::new();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, 100.0), 20.0, Rgb([1.0, 0.0, 0.0])));
    for i in 1..=1000 {
        let ifl = i as f64;

        world.add(Sphere::new(Vector3::new(20.0 + ifl, 0.5, 200.0 - ifl * 3.0), 50.0, Rgb([0.0, 1.0, ifl / 1000.0])));
    }

    let camera_base = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.1, 1.0).normalize());
    let camera = Camera::new(camera_base, width, height, 50.0);

    let photo_start_time = Instant::now();
    let image = camera.take_photo(&world, tx);
    let photo_duration = photo_start_time.elapsed();
    println!("Photo generation completed in: {:?}", photo_duration);
    
    image.save("output.png").unwrap();
    println!("Generated image");
}
