use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct BVHNode {
    left: Arc<dyn Hittable + Sync + Send>,
    right: Arc<dyn Hittable + Sync + Send>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: Vec<Arc<dyn Hittable + Send + Sync>>, t_0: f64, t_1: f64) -> BVHNode {
        let length = hittable_list.len();
        return make_bvh_node(&mut hittable_list.clone(), 0, length, t_0, t_1);
    }
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

fn random_axis() -> Axis {
    let i: usize = rand::random();
    match i % 3 {
        0 => Axis::X,
        1 => Axis::Y,
        _ => Axis::Z,
    }
}

fn box_compare(
    axis: Axis,
    a: &Arc<dyn Hittable + Send + Sync>,
    b: &Arc<dyn Hittable + Send + Sync>,
) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(a_box), Some(b_box)) => match axis {
            Axis::X => return f64::partial_cmp(&a_box.minimum.x, &b_box.minimum.x).unwrap(),
            Axis::Y => return f64::partial_cmp(&a_box.minimum.y, &b_box.minimum.y).unwrap(),
            Axis::Z => return f64::partial_cmp(&a_box.minimum.z, &b_box.minimum.z).unwrap(),
        },
        _ => unreachable!(),
    }
}

fn make_bvh_node(
    objects: &mut Vec<Arc<dyn Hittable + Sync + Send>>,
    start: usize,
    end: usize,
    t_0: f64,
    t_1: f64,
) -> BVHNode {
    let axis = random_axis();
    let object_span = end - start;

    let (left, right): (
        Arc<dyn Hittable + Sync + Send>,
        Arc<dyn Hittable + Sync + Send>,
    ) = match object_span {
        1 => (objects[start].clone(), objects[start].clone()),
        2 => {
            if box_compare(axis, &objects[start], &objects[start + 1]) == Ordering::Less {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        }
        _ => {
            let range = start..end;
            let mut focussed_objects: Vec<Arc<dyn Hittable + Sync + Send>> =
                objects.clone()[range].to_vec();
            focussed_objects.sort_by(|a, b| box_compare(axis, a, b));
            let range = start..end;
            objects.splice(range.clone(), focussed_objects);

            let mid = start + object_span / 2;
            let left = make_bvh_node(objects, start, mid, t_0, t_1);
            let right = make_bvh_node(objects, mid, end, t_1, t_1);
            (Arc::new(left), Arc::new(right))
        }
    };
    let bounding_box = match (left.bounding_box(t_0, t_1), right.bounding_box(t_0, t_1)) {
        (Some(left_box), Some(right_box)) => left_box + right_box,
        _ => unreachable!(),
    };
    return BVHNode {
        left,
        right,
        bounding_box,
    };
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.does_hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        if let Some(hit_record) = hit_left {
            return self
                .right
                .hit(ray, t_min, hit_record.time)
                .or(Some(hit_record));
        } else {
            return self.right.hit(ray, t_min, t_max);
        }
    }

    fn bounding_box(&self, _t_0: f64, _t_1: f64) -> Option<AABB> {
        return Some(self.bounding_box);
    }
}
