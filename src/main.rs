use crate::algebra::{Point3, Vector3};
use crate::visualize::show;
use crate::visualize::ShowMessage::{ShowImage, ShowPixelMessage};
use crate::algebra::Ray;
use scene::texture::CheckerboardTexture;
use image::{Rgb, RgbImage};
use crate::render::{RenderListener, Renderer, TraceRenderer};
use crate::scene::light::Light;
use crate::scene::material::BaseMaterial;
use crate::scene::object::Object;
use crate::scene::Scene;
use crate::scene::surface::Surface;

mod algebra;
mod visualize;
mod scene;
mod render;

fn generate_image(scene: &Scene, width: u32, height: u32, tx: impl RenderListener) -> RgbImage {
    let renderer = TraceRenderer::new();
    
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
    scene.add(Object::sphere(Point3::new(200.0, 0.0, z + 100.0), 100.0, &Surface::new(Rgb([1.0, 0.0, 0.0]), &mat)));
    //scene.add(Object::sphere(Point3::new(-50.0, -50.0, 100.0), 50.0, Rgb([0.0, 1.0, 0.0]), &mat));
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

    let refrac: BaseMaterial = BaseMaterial {
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
    let visualize = true;
    let video = true;
    let width: u32 = 3820;
    let height: u32 = 1920;

    if visualize {
        show(width as i32, height as i32, move |tx| {
            if video {
                for i in 0..1024 {
                    let scene = create_scene(i);
                    let image = generate_image(&scene, width, height, |_m| {});

                    tx.send_blocking(ShowImage(image)).unwrap()
                }
            } else {
                let scene = create_scene(0);
                generate_image(&scene, width, height, |(x, y, c)|tx.send_blocking(ShowPixelMessage(x, y, c)).unwrap());
            }
        })
    } else {
        let scene = create_scene(0);
        let image = generate_image(&scene, width, height, |_m| {});
        image.save("../example.png").unwrap();

        println!("Generated image");
    }
}