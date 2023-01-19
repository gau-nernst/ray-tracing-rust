use crate::vec3::Color;

pub fn write_color(color: &Color) {
    println!(
        "{} {} {}",
        (255.999 * color.0) as u8,
        (255.999 * color.1) as u8,
        (255.999 * color.2) as u8,
    )
}
