use crate::vec3::Vec3;

pub fn write_color(color: &Vec3) {
    println!(
        "{} {} {}",
        (255.999 * color.x.sqrt()) as u8,
        (255.999 * color.y.sqrt()) as u8,
        (255.999 * color.z.sqrt()) as u8,
    )
}
