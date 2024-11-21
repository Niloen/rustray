use std::fmt;
use std::sync::Arc;
use crate::algebra::{Bounded, BoundingBox, Ray};
use crate::render::trace::world::intersect::{Intersecting, Intersection};

#[derive(Debug)]
pub struct OctreeConfig {
    pub max_objects: usize,
    pub max_depth: usize,
    pub loose_factor: f64,
}

impl OctreeConfig {
    pub fn new(max_objects: usize, max_depth: usize, loose_factor: f64) -> Self {
        Self {
            max_objects,
            max_depth,
            loose_factor,
        }
    }
}

pub struct OctreeNode {
    bounding_box: BoundingBox,
    children: Option<Vec<OctreeNode>>, // 8 child nodes if subdivided
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
            if self.children.is_some() { 8 } else { 0 }
        )?;

        if let Some(children) = &self.children {
            for (i, child) in children.iter().enumerate() {
                write!(f, "\nChild {}: {:?}", i, child)?;
            }
        }

        Ok(())
    }
}
impl OctreeNode {
    /// Creates a new empty octree node with the given bounding box and depth.
    pub fn new(bounding_box: BoundingBox, depth: usize) -> Self {
        Self {
            bounding_box,
            children: None,
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
        self.children = Some(children);

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
        if self.children.is_none() && (self.objects.len() < config.max_objects || self.depth >= config.max_depth) {
            self.objects.push(obj);
            return;
        }

        // Subdivide if this is the first time we're adding children
        if self.children.is_none() {
            self.subdivide(config);
        }

        // Try inserting the object into one of the children
        let mut fits_in_child = false;
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.bounding_box.expand_by_factor(config.loose_factor).contains(obj_bbox) {
                    child.insert(obj.clone(), obj_bbox, config);
                    fits_in_child = true;
                    break; // Only in one
                }
            }
        }

        // If the object doesn't fit neatly into any child, store it in this node
        if !fits_in_child {
            self.objects.push(obj);
        }
    }
}

impl Intersecting for OctreeNode {
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        if !self.bounding_box.intersects_ray(ray) {
            return None;
        }

        // Start with the result from the objects in this node
        let mut result = self.objects.intersects(ray, max);

        // Check intersections in the children
        if let Some(children) = &self.children {
            if let Some(result2) = children.intersects(ray, result.as_ref().map_or(max, |i| i.distance)) {
                // Update result if the new intersection is closer
                result = Some(result2);
            }
        }

        result
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
    fn intersects(&self, ray: &Ray, max: f64) -> Option<Intersection> {
        // Start with the result from the objects in this node
        let mut result = self.root.intersects(ray, max);

        if let Some(result2) = self.outside.intersects(ray, result.as_ref().map_or(max, |i| i.distance)) {
            // Update result if the new intersection is closer
            result = Some(result2);
        }

        result
    }
}

impl Bounded for Octree {
    fn bounding_box(&self) -> BoundingBox {
        self.root.bounding_box
    }
}
