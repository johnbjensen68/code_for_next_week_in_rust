mod color;
mod common;
mod hittable;
mod hittable_list;
mod vec3;
mod ray;
mod sphere;
mod camera;
mod material;
mod interval;
mod aabb;
mod bvh;
mod texture;

use std::rc::Rc;
use color::Color;
use hittable_list::HittableList;
use material::{Lambertian, Metal, Dielectric};
use sphere::Sphere;
use vec3::{Point3};
use bvh::BvhNode;
use camera::Camera;
use texture::CheckerTexture;

use crate::{texture::ImageTexture, vec3::Vec3};



fn random_scene() -> HittableList {
    let mut world = HittableList::new();
 
     let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let sp1 = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::from_texture(checker)),
    );
    world.add(Box::new(sp1));

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
 
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_double(),
                0.2,
                b as f64 + 0.9 * common::random_double(),
            );
 
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    let center2 = center + vec3::Vec3::new(0.0, common::random_double_range(0.0,0.5), 0.0);
                    world.add(Box::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_double_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
 
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
 
    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
 
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
 
    world
}

fn bouncing_spheres() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;

    // World
    let world = BvhNode::new(random_scene());
    //let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    const IMAGE_WIDTH: usize = 400;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let cam = Camera::new(
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();
 
    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0), 10.0,
        Rc::new(Lambertian::from_texture(checker.clone())),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0), 10.0,
        Rc::new(Lambertian::from_texture(checker)),
    )));
 
    const IMAGE_WIDTH: usize = 400;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    let cam = Camera::new(
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        10.0);

    cam.render(&world);
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::from_texture(earth_texture));
    let globe = Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface,
    ));
     let mut world = HittableList::new();
     world.add(globe);

    const IMAGE_WIDTH: usize = 400;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
     let cam = Camera::new(
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.1,
        10.0);

    cam.render(&world);
}

fn main() {
    match 3 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => {}
    }
}