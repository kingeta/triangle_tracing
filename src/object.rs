/* Encapsulates objects which have a shape and material and colour/other properties */
#![allow(dead_code, unused_imports)]
use super::shape::*;
use super::material::Material;
use super::vector::*;
use super::colour::*;
use obj::Obj;

/// Information about an intersection with an object
#[derive(Copy, Clone)] // This is only required for the boxed vector of objects
pub struct ObjectHit {
    pub point: Vec3,
    pub normal: Vec3,
    pub dist: Float, // Also only required for boxed vector of objects
    pub material: Material,
    pub colour: Colour,
}

/// An intersectable object with some look (material, colour)
pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<ObjectHit>;
}

/// Object type which holds a single primitive
pub struct GeneralObject<T: Shape> {
    pub shape: T,
    pub material: Material,
    pub colour: Colour,
}

impl<T> Object for GeneralObject<T> where T: Shape {
    fn intersect(&self, ray: Ray) -> Option<ObjectHit> {
        match self.shape.intersect(ray) {
            Some(vals) => Some(ObjectHit{
                point: vals.point, normal: vals.norm, dist: vals.dist,
                material: self.material, colour: self.colour
            }),
            None => None,
        }
    }
}

/// A list of triangles basically
//pub struct TriangleCollection {
pub struct ObjectCollection<T: Shape> {
    //pub triangles: Vec<Triangle>,
    pub shapes: Vec<T>,
    pub material: Material,
    pub colour: Colour,
}

impl ObjectCollection<Triangle> {
    // Create a square given the four points which go clockwise from top left (conventionally)
    pub fn square(a: Vec3, b: Vec3, c: Vec3, d: Vec3, material: Material, colour: Colour) -> ObjectCollection<Triangle> {

        ObjectCollection::<Triangle> {
        shapes: vec![
            Triangle::new(
                a, d, b, // Top left triangle
            ),
            Triangle::new(
                b, d, c, // Bottom right triangle
            )
        ],
        material: material,
        colour: colour,
        }
    }
}

//impl Object for TriangleCollection {
impl<T> Object for ObjectCollection<T> where T: Shape {
    fn intersect(&self, ray: Ray) -> Option<ObjectHit> {
        let mut hit: Option<Hit> = None;

        for shape in self.shapes.iter() {
            if let Some(candidate_hit) = shape.intersect(ray) {
                match hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => if candidate_hit.dist < prev.dist {
                        hit = Some(candidate_hit);
                    }
                }
            }
        }
        
        match hit {
            Some(vals) => Some(ObjectHit{ point: vals.point, normal: vals.norm, dist: vals.dist,
                                material: self.material, colour: self.colour }),
            None => None, // None of the shapes were hit
        }

    }
}


/*
/// Contains multiple different shapes
pub struct ObjectMultiple {
    pub objects: Vec<Box<dyn Object>>,
    pub material: Material,
    pub colour: Colour,
}

*/

impl Object for Vec<Box<dyn Object>> {
    fn intersect(&self, ray: Ray) -> Option<ObjectHit> {
        let mut object_hit: Option<ObjectHit> = None;

        for object in self.iter() {
            if let Some(candidate_hit) = object.intersect(ray) {
                match object_hit {
                    None => object_hit = Some(candidate_hit),
                    Some(previous) => if candidate_hit.dist < previous.dist {
                        object_hit = Some(candidate_hit);
                    }
                }
            }
        }

        object_hit
    }
}


/// Convert an OBJ file into a large amount of triangles
pub fn convert_objects_to_polygons(obj: &Obj<obj::SimplePolygon>) -> Vec<Triangle> {
        let mut polygons = vec![];

    let make_vector = |floats: &[f32; 3]| {
        Vec3 {
            x: floats[0],
            y: floats[1],
            z: floats[2],
        }
    };

    let make_polygon = |index1, index2, index3| {
        let obj::IndexTuple(index1, _, _) = index1;
        let obj::IndexTuple(index2, _, _) = index2;
        let obj::IndexTuple(index3, _, _) = index3;

        let vertex1 = make_vector(&obj.position[index1]);
        let vertex2 = make_vector(&obj.position[index2]);
        let vertex3 = make_vector(&obj.position[index3]);

        //let a = vertex2.sub(vertex1);
        //let b = vertex3.sub(vertex1);

        //let normal = a.cross(b).normalize();

        Triangle::new(vertex1, vertex2, vertex3)
    };

    for object in &obj.objects {
        for group in &object.groups {
            for poly in &group.polys {
                let index1 = poly[0];
                for others in poly[1..].windows(2) {
                    let polygon = make_polygon(index1, others[0], others[1]);
                    polygons.push(polygon);
                }
            }
        }
    }

    return polygons;
}


/// A participating medium with given density;
/// should probably use the gas material type
pub struct MediumObject {
    pub density: Float,
    pub material: Material,
    pub colour: Colour,
}

impl Object for MediumObject {
    fn intersect(&self, ray: Ray) -> Option<ObjectHit> {
        let dist = -(-random_float()).ln_1p() / self.density;

        Some(ObjectHit {
            point: ray.eval(dist),
            normal: ray.direction,
            dist: dist,
            material: self.material,
            colour: self.colour,
        })
    }

}