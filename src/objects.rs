use glam::Vec3;
use crate::Ray;

pub struct RayHit {
    pub t: f32,
    pub position: Vec3,
    pub surface_normal: Vec3,
}

pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<RayHit>;   
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32
}

impl Default for Sphere {
    fn default() -> Self {
        Self { position: Vec3::ZERO, radius: 1.0 }
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<RayHit> {
        /*
         *  equation of ray: 
         *      a + bt = (x, y) ->
         *          a_x + b_x * t = x
         *          a_y + b_y * t = y
         * 
         *  equation of circle at origin:
         *      x^2 + y^2 = r^2
         * 
         *  with replacement (solving for t):
         *      (a_x + b_x*t)^2 + (a_y + b_y*t)^2 = r^2
         *      a_x^2 + 2*a_x*b_x*t + b_x^2*t^2 + a_y^2 + 2*a_y*b_y*t + b_y^2*t^2 = r^2 
         *      (b_x^2 + b_y^2)*t^2 + 2*(a_x*b_x + a_y*b_y)*t + (a_x^2 + a_y^2 - r^2) = 0
         * 
         *  apply quadratic formula to get solutions for t:
         *      used on formulas of form:
         *          ix^2 + jx + k = 0
         *      quadratic formula:
         *          x = ( -j +- sqrt(j^2 - 4ik) ) / 2i
         * 
         *          t = ( -2*(a_x*b_x + a_y*b_y) +- sqrt((2*(a_x*b_x + a_y*b_y))^2 - 4(b_x^2 + b_y^2)(a_x^2 + a_y^2 - r^2)) ) / 2(b_x^2 + b_y^2)
         *      
         *      make this slightly less disgusting with simplification
         *          discriminant = j^2 - 4ik
         *          i = b_x^2 + b_y^2 = b.dot(b)
         *          j = a_x*b_x + a_y*b_y = a.dot(b)
         *          k = a_x^2 + a_y^2 - r^2 = a.dot(a) - r^2
         *          
         *          discriminant = j^2 - 4*i*k
         *          t = (-j - discriminant) / 2*i 
         * 
         *      if discriminant == 0 -> one intersection
         *         discriminant < 0  -> no real intersection
         *         discriminant > 0  -> two intersections (take smallest value of t because we want the first intersection)
         * 
         *      with t, we can solve for intersection point
         *          a + bt = (x, y)
        */
        
        // We want our sphere to be "at the origin", we'll add this back at the end
        let origin = ray.position - self.position;
        let i = ray.direction.dot(ray.direction);
        let j = 2.0 * origin.dot(ray.direction);
        let k = origin.dot(origin) - self.radius * self.radius;
        let discriminant = j*j - 4.*i*k;

        if discriminant >= 0. {
            let t = (-j - f32::sqrt(discriminant)) / 2.* i;
            let intersection = (origin + ray.direction * t) + self.position; // shifting back into camera coordinate
            let surface_normal = (intersection - self.position).normalize();

            return Some(RayHit {
                t,
                position: intersection,
                surface_normal
            })
        }

        None
    }
}
