use crate::{
    components::surface::{SurfaceHit, Ray, Surface, SurfaceEnum},
    util::{
        bounding_box::BoundingBox,
        interval::{Interval, interval},
    },
};

#[derive(Debug, Clone, Default)]
pub struct SurfaceList {
    surfaces: Vec<SurfaceEnum>,
    bounding_box: BoundingBox,
}

impl SurfaceList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, surface: impl Into<SurfaceEnum>) {
        let surface = surface.into();
        self.bounding_box = self.bounding_box.join(surface.bounding_box());
        self.surfaces.push(surface);
    }
}

impl Surface for SurfaceList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
        let mut best_hit = None;
        let mut closest_so_far = ray_t.max;

        for surface in &self.surfaces {
            if let Some(hit) = surface.hit(ray, interval(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

#[derive(Debug, Clone)]
pub struct BoundingVolumeHierarchy {
    left: Box<SurfaceEnum>,
    right: Box<SurfaceEnum>,
    bounding_box: BoundingBox,
}

impl BoundingVolumeHierarchy {
    pub fn new(surfaces: &mut [SurfaceEnum]) -> Self {
        let mut bounding_box = BoundingBox::default();
        for surface in surfaces.iter() {
            bounding_box = bounding_box.join(surface.bounding_box());
        }

        let axis = bounding_box.longest_axis();

        let left;
        let right;

        if surfaces.len() == 1 {
            left = Box::new(surfaces[0].clone());
            right = Box::new(surfaces[0].clone());
        } else if surfaces.len() == 2 {
            left = Box::new(surfaces[0].clone());
            right = Box::new(surfaces[1].clone());
        } else {
            surfaces.sort_by(|a, b| {
                let a = a.bounding_box()[axis].min;
                let b = b.bounding_box()[axis].min;
                a.partial_cmp(&b).unwrap()
            });

            let (left_surfaces, right_surfaces) = surfaces.split_at_mut(surfaces.len() / 2);
            left = Box::new(BoundingVolumeHierarchy::new(left_surfaces).into());
            right = Box::new(BoundingVolumeHierarchy::new(right_surfaces).into());
        }

        BoundingVolumeHierarchy {
            left,
            right,
            bounding_box,
        }
    }
}

impl Surface for BoundingVolumeHierarchy {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
        if !self.bounding_box.hit(ray, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        let max = hit_left.as_ref().map_or(ray_t.max, |hit| hit.t);
        let hit_right = self.right.hit(ray, interval(ray_t.min, max));

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

impl From<SurfaceList> for BoundingVolumeHierarchy {
    fn from(mut value: SurfaceList) -> Self {
        BoundingVolumeHierarchy::new(&mut value.surfaces)
    }
}
