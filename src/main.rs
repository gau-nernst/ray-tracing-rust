mod camera;
mod hittable;
mod material;
mod pcg32;
mod tiff;
mod vec3;
use std::rc::Rc;

use std::time::Instant;

use camera::{Camera, Renderer};
use hittable::{BVHNode, HittableList, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use pcg32::PCG32;
use tiff::TiffFile;
use vec3::Vec3;

fn generate_spheres(objects: &mut HittableList) {
    let mut rng = PCG32::new(19, 29);

    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Vec3(0.5, 0.5, 0.5))),
    )));
    objects.push(Rc::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    objects.push(Rc::new(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Vec3(0.4, 0.2, 0.1))),
    )));
    objects.push(Rc::new(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0)),
    )));

    let something = Vec3(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.f32();
            let center = Vec3(a as f32 + 0.9 * rng.f32(), 0.2, b as f32 + 0.9 * rng.f32());
            if (center - something).length() > 0.9 {
                let material: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    material = Rc::new(Lambertian::new(Vec3::rand(&mut rng) * Vec3::rand(&mut rng)));
                } else if choose_mat < 0.95 {
                    material = Rc::new(Metal::new(Vec3::rand_between(&mut rng, 0.5, 1.0), rng.f32() * 0.5));
                } else {
                    material = Rc::new(Dielectric::new(1.5));
                }
                objects.push(Rc::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    // let bvh = Rc::new(BVHNode::new(&objects.objects, &mut rng));
    // objects.clear();
    // objects.push(bvh);
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let camera = Camera::new(
        aspect_ratio,
        Vec3(13.0, 2.0, 3.0),
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        20.0,
        0.1,
        10.0,
    );
    let renderer = Renderer::new(400, aspect_ratio, 10, 10);

    let mut objects = HittableList::new();
    generate_spheres(&mut objects);
    let mut buffer = vec![0_u8; (renderer.img_height * renderer.img_width * 3) as usize];

    let now = Instant::now();
    renderer.render(&objects, &camera, &mut buffer);
    let elapsed_time = now.elapsed();
    eprintln!("\nDone.");
    eprintln!("{} seconds.", elapsed_time.as_secs());

    let mut tiff_file = TiffFile::new("sample.tiff", renderer.img_width, renderer.img_height);
    tiff_file.write(&buffer);
}
