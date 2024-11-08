use super::vector::Vector3;
struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn at(&self, distance: f64) -> Vector3 {
        self.origin + self.direction * distance
    }
}

struct HitResult {
    distance: f64,
    normal: Vector3
}
trait Object {
    fn intersects(&self, ray: &Ray) -> bool {
        self.hit(ray).is_some()
    }
    
    fn hit(&self, r: &Ray) -> Option<HitResult>;
}

struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Object for &Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        let L = self.center - ray.origin;
        let tca = L.dot(ray.direction);
        let d2 = L.dot(L) - tca * tca;
        if d2 > self.radius * self.radius { 
            return None 
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;


        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        if (t0 < 0.0) {
            t0 = t1; // If t0 is negative, let's use t1 instead.
            if t0 < 0.0 {
                return None; // Both t0 and t1 are negative.
            } 
        }

        Some(HitResult {
            distance: t0,
            normal: (ray.at(t0) - self.center).normalize()
        })
    }
}