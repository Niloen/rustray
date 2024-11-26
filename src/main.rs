use std::path::PathBuf;
use clap::Parser;
use crate::algebra::{Point3, Vector3};
use crate::visualize::show;
use crate::visualize::ShowMessage::{ShowImage, ShowPixelMessage};
use crate::algebra::Ray;
use scene::texture::CheckerboardTexture;
use image::{Rgb, RgbImage};
use nalgebra::min;
use crate::buffer::BufferedChannel;
use crate::render::{RenderListener, Renderer, TraceRenderConfig, TraceRenderer};
use crate::scene::light::Light;
use crate::scene::material::BaseMaterial;
use crate::scene::object::Object;
use crate::scene::Scene;
use crate::scene::surface::Surface;

mod algebra;
mod visualize;
mod scene;
mod render;
mod buffer;

/// Command-line interface for the raytracer application.
#[derive(Parser, Debug)]
#[command(version, about = "A raytracer application with optional video and visualization modes.")]
struct Cli {
    /// Width of the output image or video frame
    #[arg(long, default_value_t = 3840)]
    width: u32,

    /// Height of the output image or video frame
    #[arg(long, default_value_t = 1920)]
    height: u32,

    #[arg(long = "no-parallel", action = clap::ArgAction::SetFalse, default_value_t = true)]
    parallel: bool,

    /// Enables visualization mode
    #[arg(long = "visualize", default_value_t = false)]
    visualize: bool,

    /// Number of frames to generate for video mode
    #[arg(short = 'f', long, default_value_t = 64, requires = "video")]
    video_frames: u32,

    /// Buffer size for video frames
    #[arg(short, long, default_value_t = 1, requires = "video")]
    video_buffer: u32,


    /// Enables video mode
    #[arg(long, default_value_t = false)]
    video: bool,

    /// Path to save the generated image (ignored in visualization or video mode)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}
fn generate_image(scene: &Scene, width: u32, height: u32, tx: impl RenderListener, parallel: bool) -> RgbImage {
    let renderer = TraceRenderer::new(TraceRenderConfig {
        parallel
    });
    
    renderer.render(scene, width, height, tx)
}

fn create_scene(frame: u32) -> Scene {
    create_scene2(frame)
}

#[allow(dead_code)]
fn create_scene1(_frame: u32) -> Scene {
    const MAT: BaseMaterial = BaseMaterial::DEFAULT;
    let mut scene = Scene::new();
    scene.add_light(Light::new(Ray::new(Point3::new(0.0, 100.0, 100.0), Vector3::new(0.0, -1.0, 0.0)), Rgb([1.0, 1.0, 1.0])));
    scene.add_light(Light::new(Ray::new(Point3::new(-100.0, 0.0, 80.0), Vector3::new(0.0, -1.0, 0.0)), Rgb([0.0, 0.0, 1.0])));
    scene.add(Object::sphere(Point3::new(0.0, 25.0, 100.0), 20.0, &Surface::new(Rgb([1.0, 1.0, 1.0]), &MAT)));
    for i in 1..=1000 {
        let ifl = i as f64;

        scene.add(Object::sphere(Point3::new(20.0 + ifl, 0.5, 200.0 - ifl * 3.0), 50.0, &Surface::new(Rgb([1.0, 1.0, 1.0]), &MAT)));
    }
    scene
}

#[allow(dead_code)]
fn create_scene2(frame: u32) -> Scene {
    let mat: BaseMaterial = BaseMaterial::DEFAULT;
    let mirror: BaseMaterial = BaseMaterial {
        reflectivity: 0.7,
        ..mat
    };

    let _refrac: BaseMaterial = BaseMaterial {
        refractive: 1.2,
        ..mat
    };
    let white = Rgb([1.0, 1.0, 1.0]);
    let green = Rgb([0.0, 1.0, 0.0]);
    let blue = Rgb([0.0, 0.0, 1.0]);
    let _black = Rgb([0.0, 0.0, 0.0]);

    let mut scene = Scene::new();

    let z = 200.0;

    scene.add_light(Light::new(Ray::new(Point3::new(0.0, 100.0, z), Vector3::new(0.0, -1.0, 0.0)), white));
    scene.add_light(Light::new(Ray::new(Point3::new(-10.0, -25.0, z), Vector3::new(0.0, 0.0, -1.0)), Rgb([0.2, 0.2, 0.2])));

    let checkerboard_texture1 = CheckerboardTexture::new(Surface::new(white, &mat), Surface::new(green, &mat), 0.01);
    let checkerboard_texture2 = CheckerboardTexture::new(Surface::new(white, &mat), Surface::new(blue, &mat), 0.01);

    scene.add(Object::plane(Point3::new(0.0, -100.0, z + 0.0), Vector3::new(0.0, 1.0, 0.0), &checkerboard_texture1));
    scene.add(Object::plane(Point3::new(0.0, 150.0, z + 0.0), Vector3::new(0.0, -1.0, 0.0), &checkerboard_texture2));

    scene.add(Object::sphere(Point3::new(20.0, 20.0, z + 100.0), 20.0, &Surface::new(white, &mirror)));
    scene.add(Object::sphere(Point3::new(-50.0 + 4.0 * frame as f64, 20.0, z + 75.0), 40.0, &Surface::new(white, &mirror)));
    scene.add(Object::sphere(Point3::new(200.0, (frame as f64 * 0.25).sin().abs() * 100.0 , z + 100.0), 100.0, &Surface::new(Rgb([1.0, 0.0, 0.0]), &mat)));

    scene.add(Object::cube(Point3::new(-10.0, -25.0, z + 50.0), 20.0, &Surface::new(Rgb([0.0, 0.0, 1.0]), &mat)));
    scene.add(Object::cube(Point3::new(-50.0, -25.0, z + 120.0), 30.0, &Surface::new(Rgb([1.0, 1.0, 0.0]), &mat)));
    scene.add(Object::cube(Point3::new(0.0, -20.0, z + 300.0), 100.0, &Surface::new(white, &mirror)));
    scene
}

#[allow(dead_code)]
fn create_scene3(_frame: u32) -> Scene {
    let mat: BaseMaterial = BaseMaterial::DEFAULT;
    let _mirror: BaseMaterial = BaseMaterial {
        reflectivity: 0.9,
        ..mat
    };

    let white = Rgb([1.0, 1.0, 1.0]);
    let green = Rgb([0.0, 1.0, 0.0]);
    let _blue = Rgb([0.0, 0.0, 1.0]);

    let mut scene = Scene::new();

    scene.add_light(Light::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)), white));
    scene.add(Object::plane(Point3::new(0.0, -100.0, 0.0), Vector3::new(0.0, 1.0, 0.0), &Surface::new(green, &mat)));
    scene.add(Object::sphere(Point3::new(20.0, 20.0, 100.0), 20.0, &Surface::new(white, &mat)));
    scene
}

#[allow(dead_code)]
fn create_scene4<'a>(_frame: u32) -> Scene {
    let mat: BaseMaterial = BaseMaterial::DEFAULT;
    let _mirror: BaseMaterial = BaseMaterial {
        reflectivity: 0.9,
        ..mat
    };

    let _refrac: BaseMaterial = BaseMaterial {
        refractive: 4.0,
        ..mat
    };

    let white = Rgb([1.0, 1.0, 1.0]);
    let green = Rgb([0.0, 1.0, 0.0]);
    let blue = Rgb([0.0, 0.0, 1.0]);

    let z = 200.0;

    let checkerboard_texture1 = CheckerboardTexture::new(Surface::new(white, &mat), Surface::new(green, &mat), 0.01);

    let mut scene = Scene::new();

    scene.add_light(Light::new(Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0)), white));
    scene.add(Object::plane(Point3::new(0.0, -40.0, z + 200.0), Vector3::new(0.0, 1.0, 0.0), &checkerboard_texture1));
    scene.add(Object::sphere(Point3::new(100.0, 100.0, z), 60.0, &Surface::new(blue, &mat)));
    scene
}


fn main() {
    let cli = Cli::parse();

    if cli.visualize {
        show(cli.width as i32, cli.height as i32, move |tx| {
            if cli.video {
                let btx = BufferedChannel::new(min(cli.video_frames as usize, cli.video_buffer as usize), move |m|tx.send_blocking(m).unwrap());
                for i in 0..cli.video_frames {
                    let scene = create_scene(i);
                    let image = generate_image(&scene, cli.width, cli.height, |_m| {}, cli.parallel);

                    btx.send(ShowImage(image)).unwrap()
                }
            } else {
                let scene = create_scene(0);
                generate_image(&scene, cli.width, cli.height, |(x, y, c)|tx.send_blocking(ShowPixelMessage(x, y, c)).unwrap(), cli.parallel);
            }
        })
    } else {
        let scene = create_scene(0);
        let image = generate_image(&scene, cli.width, cli.height, |_m| {}, cli.parallel);
        let output_path = cli.output.unwrap_or(PathBuf::from("output.png"));

        image.save(output_path).expect("Failed to save image");
    }
}