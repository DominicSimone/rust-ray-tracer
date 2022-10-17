use glam::Vec3;

pub trait LightEmitting {
    fn light_on(&self, position: &Vec3, surface_normal: &Vec3) -> Vec3;
    fn dir_from(&self, position: &Vec3) -> Vec3;
}

pub struct PointLight {
    pub position: Vec3,
    pub power: Vec3,
}

pub struct DirectionalLight {
    pub direction: Vec3,
    pub power: Vec3
}

impl Default for DirectionalLight {

    fn default() -> Self {
        Self { direction: Vec3::NEG_Y, power: Vec3::splat(1.) }
    }
}

impl LightEmitting for DirectionalLight {
    fn light_on(&self, _position: &Vec3, surface_normal: &Vec3) -> Vec3 {
        surface_normal.dot(self.direction) * self.power
    }

    fn dir_from(&self, _position: &Vec3) -> Vec3 {
        -self.direction
    }
}
