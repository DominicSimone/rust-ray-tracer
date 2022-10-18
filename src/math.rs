use glam::Vec3;

pub fn reflect(incident: Vec3, surface_normal: Vec3) -> Vec3 {
    incident - 2. * surface_normal * (incident.dot(surface_normal))
}