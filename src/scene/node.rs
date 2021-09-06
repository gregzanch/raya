use crate::geometry::{Primitive, Ray};
use crate::scene::{Intersection, AcousticMaterial};
use nalgebra::{Affine3, Matrix4, Vector3, distance_squared, vector};



#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id: u32,
    pub children: Vec<SceneNode>,
    pub transform: Affine3<f32>,
    pub inv_transform: Affine3<f32>,
    pub name: String,
    pub acoustic_material: AcousticMaterial,
    pub primitive: Primitive,
}

impl SceneNode {
    pub fn new(id: u32, name: String) -> SceneNode {
        SceneNode {
            id,
            children: Vec::new(),
            transform: Affine3::identity(),
            inv_transform: Affine3::identity(),
            name,
            acoustic_material: AcousticMaterial::default(),
            primitive: Primitive::None,
        }
    }

    pub fn find_child_by_id(&self, id: u32) -> Option<&SceneNode> {
        if self.id == id {
            return Some(self);
        } else {
            for child in self.children.iter() {
                let res = child.find_child_by_id(id);
                if res.is_some() {
                    return Some(&res.unwrap())
                }
            }
        }
        None
    }

    pub fn add_child(&mut self, child: SceneNode) {
        self.children.push(child);
    }


    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.apply_transform(Matrix4::new_nonuniform_scaling(&vector![x, y, z]));
    }


    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.apply_transform(Matrix4::new_translation(&vector![x, y, z]));
    }


    pub fn rotate(&mut self, axis: &str, angle: f32) {
        let axis = match axis {
            "x" | "X" => Vector3::x_axis(),
            "y" | "Y" => Vector3::y_axis(),
            "z" | "Z" => Vector3::z_axis(),
            _ => panic!(
                "Got unexpected axis: \'{}\' while trying to apply rotation to node \'{}\'",
                axis, self.name
            ),
        };
        self.apply_transform(Matrix4::from_axis_angle(&axis, angle.to_radians()));
    }


    fn apply_transform(&mut self, t: Matrix4<f32>) {
        let ta: Affine3<f32> = Affine3::from_matrix_unchecked(t);
        self.transform = ta * self.transform;
        self.inv_transform = self.transform.inverse();
    }
}

impl Intersect for SceneNode {
    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let transformed_ray = self.inv_transform * *ray;

        let mut t_value: f32 = 0.0;
        let mut normal = vector![0.0f32, 0.0, 0.0];
        let mut uv = [0.0, 0.0];
        let self_collides =
            if self
                .primitive
                .collides(&transformed_ray, &mut t_value, &mut normal, &mut uv)
            {
                Some(Intersection::new(
                    t_value,
                    transformed_ray.src + (t_value * transformed_ray.dir.normalize()),
                    &self,
                    normal,
                    uv[0],
                    uv[1],
                ))
            } else {
                None
            };

        let min = self
            .children
            .iter()
            .map(|child| child.intersects(&transformed_ray))
            .filter(|child| child.is_some())
            .map(|child| child.unwrap())
            .fold(None, |min, child| match min {
                None => Some(child),
                Some(cmin) => Some(
                    if distance_squared(&cmin.point, &transformed_ray.src)
                        < distance_squared(&child.point, &transformed_ray.src)
                    {
                        cmin
                    } else {
                        child
                    },
                ),
            });

        match (self_collides, min) {
            (None, None) => None,
            (Some(a), None) => Some(a.apply_transform(&self.transform, &self.inv_transform)),
            (None, Some(a)) => Some(a.apply_transform(&self.transform, &self.inv_transform)),
            (Some(a), Some(b)) => Some(
                (if distance_squared(&a.point, &transformed_ray.src)
                    < distance_squared(&b.point, &transformed_ray.src)
                {
                    a
                } else {
                    b
                })
                .apply_transform(&self.transform, &self.inv_transform),
            ),
        }
    }
}

pub trait Intersect {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
}
