use crate::algebra::{Bounded, BoundingBox, Distance, Point3, UnitVector3, Vector3};
use crate::scene::geometry::{Cube, HitResult, Plane, Sphere};
use crate::algebra::Ray;
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

    pub fn sphere(center: Point3, radius: Distance, texture: &dyn Texture) -> Self {
        Object::new(
            Sphere::new(),
            Transform::new(Vector3::new(center.x, center.y, center.z), Vector3::zeros(), Vector3::new(radius, radius, radius)),
            texture
        )
    }
    pub fn cube(center: Point3, side_length: Distance, texture: &dyn Texture) -> Self {
        Object::new(
            Cube::new(),
            Transform::new(Vector3::new(center.x, center.y, center.z), Vector3::zeros(), Vector3::new(side_length, side_length, side_length)),
            texture
        )
    }

    pub fn plane(point: Point3, normal: Vector3, texture: &dyn Texture) -> Self {
        Object::new(
            Plane::new(),
            Transform::new(Vector3::new(point.x, point.y, point.z), Transform::rotation_to(Plane::NORMAL.into_inner(), normal), Vector3::new(1.0, 1.0, 1.0)),
            texture
        )
    }
    
    pub fn surface_at(&self, hr: &HitResult) -> Surface {
        self.texture.surface_at(hr.coords)
    } 
}

impl Bounded for Object {
    fn bounding_box(&self) -> BoundingBox {
        // Get the local bounding box of the geometry
        let local_bbox = self.geometry.bounding_box();

        // Transform the 8 corners of the local bounding box to world space
        let corners = [
            Point3::new(local_bbox.min.x, local_bbox.min.y, local_bbox.min.z),
            Point3::new(local_bbox.min.x, local_bbox.min.y, local_bbox.max.z),
            Point3::new(local_bbox.min.x, local_bbox.max.y, local_bbox.min.z),
            Point3::new(local_bbox.min.x, local_bbox.max.y, local_bbox.max.z),
            Point3::new(local_bbox.max.x, local_bbox.min.y, local_bbox.min.z),
            Point3::new(local_bbox.max.x, local_bbox.min.y, local_bbox.max.z),
            Point3::new(local_bbox.max.x, local_bbox.max.y, local_bbox.min.z),
            Point3::new(local_bbox.max.x, local_bbox.max.y, local_bbox.max.z),
        ];

        // Transform each corner to world space
        let transformed_corners: Vec<Point3> = corners
            .iter()
            .map(|corner| self.transform.apply_to_point(corner))
            .collect();

        // Find the new min and max points in world space
        let min = transformed_corners
            .iter()
            .fold(Point3::new(Distance::INFINITY, Distance::INFINITY, Distance::INFINITY), |acc, p| acc.inf(p));
        let max = transformed_corners
            .iter()
            .fold(Point3::new(Distance::NEG_INFINITY, Distance::NEG_INFINITY, Distance::NEG_INFINITY), |acc, p| acc.sup(p));

        // Create the transformed bounding box
        BoundingBox::new(min, max)
    }
}



impl Geometry for Object {
    fn distance(&self, ray: &Ray) -> Option<Distance> {
        let local_ray = self.transform.to_local_ray_unnormalized(ray);

        // Check for intersection in local space
        self.geometry.distance(&local_ray)
    }

    /// Finds the intersection with the object in world space.

    /// Computes the detailed hit result in world space.
    fn hit(&self, ray: &Ray, distance: Distance) -> HitResult {
        let local_ray = self.transform.to_local_ray(ray);
        let hr = self.geometry.hit(&local_ray, self.transform.to_local_distance(&ray, distance));
        HitResult {
            position: self.transform.apply_to_point(&hr.position),
            normal: UnitVector3::new_normalize(self.transform.apply_to_vector(&hr.normal)),
                ..hr
        }
    }
}
