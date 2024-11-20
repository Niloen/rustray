use crate::algebra::{Point3, Vector3};
use crate::scene::geometry::{Cube, HitResult, Plane, Sphere};
use crate::scene::ray::Ray;
use crate::scene::texture::Texture;
use crate::scene::transform::Transform;
use crate::scene::geometry::Geometry;
use crate::scene::surface::Surface;

pub struct Object {
    pub geometry: Box<dyn Geometry>, // The geometry (e.g., sphere, plane)
    pub transform: Transform,             // The object's transform (world space)
    pub texture: Box<dyn Texture>,
}

impl Object {
    pub fn new(geometry: impl Geometry + 'static, transform: Transform, texture: &dyn Texture) -> Self {
        Self {
            geometry: Box::new(geometry),
            transform,
            texture: texture.clone_box()
        }
    }

    pub fn sphere(center: Point3, radius: f64, texture: &dyn Texture) -> Self {
        Object::new(
            Sphere::new(),
            Transform::new(Vector3::new(center.x, center.y, center.z), Vector3::zeros(), Vector3::new(radius, radius, radius)),
            texture
        )
    }
    pub fn cube(center: Point3, side_length: f64, texture: &dyn Texture) -> Self {
        Object::new(
            Cube::new(),
            Transform::new(Vector3::new(center.x, center.y, center.z), Vector3::zeros(), Vector3::new(side_length, side_length, side_length)),
            texture
        )
    }

    pub fn plane(point: Point3, normal: Vector3, texture: &dyn Texture) -> Self {
        Object::new(
            Plane::new(),
            Transform::new(Vector3::new(point.x, point.y, point.z), Transform::rotation_to(Plane::NORMAL, normal), Vector3::new(1.0, 1.0, 1.0)),
            texture
        )
    }
    
    pub fn surface_at(&self, hr: &HitResult) -> Surface {
        self.texture.surface_at(hr.coords)
    } 
}

impl Geometry for Object {
    fn distance(&self, ray: &Ray) -> Option<f64> {
        let local_ray = self.transform.to_local_ray(ray);

        // Check for intersection in local space
        self.geometry.distance(&local_ray).map(|distance| {
            distance * self.transform.scale()
        })
    }

    /// Finds the intersection with the object in world space.

    /// Computes the detailed hit result in world space.
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        let local_ray = self.transform.to_local_ray(ray);
        self.geometry.hit(&local_ray).map(|hr| HitResult {
            position: self.transform.apply_to_point(&hr.position),
            normal: self.transform.matrix.transform_vector(&hr.normal).normalize(),
                ..hr
        })
    }
}
