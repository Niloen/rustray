use std::thread;
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
use gtk4::Image;
mod world;
mod vector;
mod camera;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.RealtimeRenderer")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(3800)
            .default_height(1920)
            .title("Realtime Renderer")
            .build();

        // Create an initial empty Pixbuf for the Image widget.
        let width = 800;
        let height = 600;
        let pixbuf = Pixbuf::new(Colorspace::Rgb, false, 8, width, height).unwrap();
        let image_widget = Image::from_pixbuf(Some(&pixbuf));
        window.set_child(Some(&image_widget));


        window.present();

        glib::MainContext::default().spawn_local(async move {
            let (tx, rx) = async_channel::unbounded::<(u32, u32, Rgb<u8>)>();

            thread::spawn(move || {
                generate_image(width as u32, height as u32, tx)
            });

            while let Ok((x, y, Rgb([r,g,b]))) = rx.recv().await {
                pixbuf.put_pixel(x, y, r, g, b, 0);
                image_widget.set_from_pixbuf(Some(&pixbuf));
            }
        });
    });

    app.run()
}

fn generate_image(width: u32, height: u32, tx: Sender<(u32, u32, Rgb<u8>)>) {
    let mut world = World::new();
    world.add(Sphere::new(Vector3::new(0.0, 0.0, 100.0), 20.0, Rgb([1.0, 0.0, 0.0])));
    world.add(Sphere::new(Vector3::new(20.0, 0.5, 200.0), 50.0, Rgb([0.0, 1.0, 0.0])));

    let camera_base = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.1, 1.0).normalize());
    let camera = Camera::new(camera_base, width, height, 50.0);

    let image = camera.take_photo(&world, tx);

    image.save("output.png").unwrap();
    println!("Generated image");
}
