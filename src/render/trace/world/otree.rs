use crate::algebra::{Bounded, BoundingBox, Distance, Ray};
use crate::render::trace::world::intersect::{Intersecting, Intersection};
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub struct OctreeConfig {
    pub max_objects: usize,
    pub max_depth: usize,
    pub loose_factor: Distance,
}

impl OctreeConfig {
    pub fn new(max_objects: usize, max_depth: usize, loose_factor: Distance) -> Self {
        Self {
            max_objects,
            max_depth,
            loose_factor,
        }
    }
}

pub struct OctreeNode {
    bounding_box: BoundingBox,
    children: Vec<OctreeNode>, // 8 child nodes if subdivided
    objects: Vec<Arc<dyn Intersecting>>,  
    depth: usize,                     // Depth of this node in the tree
}

impl fmt::Debug for OctreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OctreeNode {{ depth: {}, bounding_box: {:?}, objects: {}, children: {} }}",
            self.depth,
            self.bounding_box,
            self.objects.len(),
            self.children.len()
        )?;

        for (i, child) in self.children.iter().enumerate() {
            write!(f, "\nChild {}: {:?}", i, child)?;
        }

        Ok(())
    }
}
impl OctreeNode {
    /// Creates a new empty octree node with the given bounding box and depth.
    pub fn new(bounding_box: BoundingBox, depth: usize) -> Self {
        Self {
            bounding_box,
            children: vec![],
            objects: vec![],
            depth,
        }
    }

    /// Subdivides the current node into 8 child octants.
    fn subdivide(&mut self, config: &OctreeConfig) {
        let center = self.bounding_box.center();
        let mut children = Vec::with_capacity(8);

        // Create child nodes
        for i in 0..8 {
            let child_bbox = self.bounding_box.subdivide(i, center);
            children.push(OctreeNode::new(child_bbox, self.depth + 1));
        }

        // Replace current children with the newly created children
        self.children = children;

        // Redistribute existing objects using self.insert
        let existing_objects = self.objects.drain(..).collect::<Vec<_>>();
        for obj in existing_objects {
            let bounding_box = obj.bounding_box();
            self.insert(obj, &bounding_box, config);
        }
    }


    /// Adds an object to this node or its children.
    pub fn insert(&mut self, obj: Arc<dyn Intersecting>, obj_bbox: &BoundingBox, config: &OctreeConfig) {
        // If the object doesn't fit within this node's loose bounding box, expand the bounding box
        self.bounding_box = self.bounding_box.union(obj_bbox);

        // If the node is a leaf and within capacity, insert the object here
        if self.children.is_empty() && (self.objects.len() < config.max_objects || self.depth >= config.max_depth) {
            self.objects.push(obj);
            return;
        }

        // Subdivide if this is the first time we're adding children
        if self.children.is_empty() {
            self.subdivide(config);
        }

        // Try inserting the object into one of the children
        let mut fits_in_child = false;
        for child in self.children.iter_mut() {
            if child.bounding_box.expand_by_factor(config.loose_factor).contains(obj_bbox) {
                child.insert(obj.clone(), obj_bbox, config);
                fits_in_child = true;
                break; // Only in one
            }
        }

        // If the object doesn't fit neatly into any child, store it in this node
        if !fits_in_child {
            self.objects.push(obj);
        }
    }
}

impl Intersecting for OctreeNode {
    fn closest_intersection(&self, ray: &Ray, max: Distance) -> Option<Intersection> {
        if !self.bounding_box.intersects_ray(ray, max) {
            return None;
        }

        // Start with the result from the objects in this node
        let mut result = self.objects.closest_intersection(ray, max);

        // Check intersections in the children
        if let Some(result2) = self.children.closest_intersection(ray, result.as_ref().map_or(max, |i| i.distance)) {
            // Update result if the new intersection is closer
            result = Some(result2);
        }

        result
    }

    fn any_intersects(&self, ray: &Ray, max: Distance) -> bool {
        if !self.bounding_box.intersects_ray(ray, max) {
            return false;
        }

        if self.objects.any_intersects(ray, max) {
            return true;
        }

        self.children.any_intersects(ray, max)
    }
}

impl Bounded for OctreeNode {
    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

pub struct Octree {
    root: OctreeNode,
    outside: Vec<Arc<dyn Intersecting>>, // Objects not fit for tree, such as infinite objects
    config: OctreeConfig,
}

impl fmt::Debug for Octree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OcTree {{ root: {:?}, config: {:?}, objects: {} }}",
            self.root,
            self.config,
            self.outside.len(),
        )?;

        Ok(())
    }
}


impl Octree {
    /// Creates a new empty octree.
    pub fn new(config: OctreeConfig, objects: Vec<Arc<dyn Intersecting>>) -> Self {
        // Objects with infinite size cannot be handled with the tree
        let (infinite, finite): (Vec<_>, Vec<_>) = objects.iter().map(|o|o.clone()).partition(|o|o.bounding_box().is_infinite());

        let mut t = Self {
            root: OctreeNode::new(finite.bounding_box(), 0),
            outside: infinite,
            config,
        };

        finite.iter().for_each(|o|t.add(o.clone()));

        t
    }

    /// Inserts an object into the octree.
    fn add(&mut self, obj: Arc<dyn Intersecting>) {
        let bounding_box = obj.bounding_box();
        self.root.insert(obj, &bounding_box, &self.config);
    }
}

impl Intersecting for Octree {
    fn closest_intersection(&self, ray: &Ray, max: Distance) -> Option<Intersection> {
        // Start with the result from the objects in this node
        let mut result = self.root.closest_intersection(ray, max);

        if let Some(result2) = self.outside.closest_intersection(ray, result.as_ref().map_or(max, |i| i.distance)) {
            // Update result if the new intersection is closer
            result = Some(result2);
        }

        result
    }

    fn any_intersects(&self, ray: &Ray, max: Distance) -> bool {
        if self.root.any_intersects(ray, max) {
            return true;
        }

        self.outside.any_intersects(ray, max)
    }
}

impl Bounded for Octree {
    fn bounding_box(&self) -> BoundingBox {
        self.root.bounding_box
    }
}

