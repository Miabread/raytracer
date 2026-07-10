use std::f64::consts::PI;

use crate::{
    camera::{Background, CameraSceneOptions},
    components::{
        material::{Dielectric, DiffuseLight, Isotropic, Lambert, Material, MaterialEnum, Metal},
        noise::Perlin,
        surface::{
            Surface,
            primitive::{Quad, Sphere},
            structure::{BoundingVolumeHierarchy, SurfaceList},
            volume::ConstantMedium,
        },
        texture::{Checker, NoiseTexture, Texture},
    },
    util::{
        interval::Interval,
        vec3::{Color, arrow, color, point},
    },
};

#[derive(Debug, Clone)]
pub struct Scene {
    pub world: BoundingVolumeHierarchy,
    pub camera: CameraSceneOptions,
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BuiltinScene {
    Empty,
    #[default]
    First,
    Second,
    MovingSpheres,
    CheckeredSpheres,
    PerlinSpheres,
    Quads,
    SimpleLight,
    CornellBox,
    CornellSmoke,
}

impl BuiltinScene {
    pub fn to_scene(self) -> Scene {
        let mut world = SurfaceList::new();
        let camera = match self {
            BuiltinScene::Empty => CameraSceneOptions::default(),

            BuiltinScene::First => {
                let material_ground = Lambert::new(color(0.8, 0.8, 0.0));
                let material_center = Lambert::new(color(0.1, 0.2, 0.5));
                let material_left = Dielectric::new(1.50);
                let material_bubble = Dielectric::new(1.00 / 1.50);
                let material_right = Metal::new(color(0.8, 0.6, 0.2), 1.0);

                world.add(Sphere::stationary(
                    point(0.0, -100.5, -10.0),
                    100.0,
                    material_ground,
                ));
                world.add(Sphere::stationary(
                    point(0.0, 0.0, -1.2),
                    0.5,
                    material_center,
                ));

                world.add(Sphere::stationary(
                    point(-1.0, 0.0, -1.0),
                    0.5,
                    material_left,
                ));
                world.add(Sphere::stationary(
                    point(-1.0, 0.0, -1.0),
                    0.4,
                    material_bubble,
                ));
                world.add(Sphere::stationary(
                    point(1.0, 0.0, -1.0),
                    0.5,
                    material_right,
                ));

                CameraSceneOptions {
                    vertical_fov: 20.0,
                    look_from: point(-2.0, 2.0, 1.0),
                    ..Default::default()
                }
            }

            BuiltinScene::Second => {
                let r = (PI / 4.0).cos();

                let material_left = Lambert::new(color(0.0, 0.0, 1.0));
                let material_right = Lambert::new(color(1.0, 0.0, 0.0));

                world.add(Sphere::stationary(point(-r, 0.0, -1.0), r, material_left));
                world.add(Sphere::stationary(point(r, 0.0, -1.0), r, material_right));

                CameraSceneOptions {
                    vertical_fov: 90.0,
                    ..Default::default()
                }
            }

            BuiltinScene::MovingSpheres => {
                let material_ground = Lambert::new(Checker::new(
                    0.32,
                    color(0.2, 0.3, 0.1),
                    color(0.9, 0.9, 0.9),
                ));
                world.add(Sphere::stationary(
                    point(0.0, -1000.0, 0.0),
                    1000.0,
                    material_ground,
                ));

                for a in -11..11 {
                    for b in -11..11 {
                        let choose_material = Interval::UNIT.random_double();
                        let center_start = point(
                            a as f64 + 0.9 * Interval::UNIT.random_double(),
                            0.2,
                            b as f64 + 0.9 * Interval::UNIT.random_double(),
                        );
                        if (center_start - point(4.0, 0.2, 0.0)).length() <= 0.9 {
                            continue;
                        }

                        let material: MaterialEnum = if choose_material < 0.8 {
                            let albedo =
                                Color::random(Interval::UNIT) * Color::random(Interval::UNIT);
                            Lambert::new(albedo).into()
                        } else if choose_material < 0.95 {
                            let albedo =
                                Color::random(Interval::UNIT) * Color::random(Interval::UNIT);
                            let fuzz = Interval::HALF.random_double();
                            Metal::new(albedo, fuzz).into()
                        } else {
                            Dielectric::new(1.5).into()
                        };

                        // if Interval::UNIT.random_double() < 0.5 {
                        if false {
                            let center_end =
                                center_start + point(0.0, Interval::HALF.random_double(), 0.0);
                            world.add(Sphere::moving(center_start, center_end, 0.2, material));
                        } else {
                            world.add(Sphere::stationary(center_start, 0.2, material));
                        }
                    }
                }

                let material1 = Dielectric::new(1.5);
                world.add(Sphere::stationary(point(0.0, 1.0, 0.0), 1.0, material1));

                let material2 = Lambert::new(color(0.4, 0.2, 0.1));
                world.add(Sphere::stationary(point(-4.0, 1.0, 0.0), 1.0, material2));

                let material3 = Metal::new(color(0.7, 0.6, 0.5), 0.0);
                world.add(Sphere::stationary(point(4.0, 1.0, 0.0), 1.0, material3));

                CameraSceneOptions {
                    vertical_fov: 20.0,
                    look_from: point(13.0, 2.0, 3.0),
                    look_at: point(0.0, 0.0, 0.0),
                    defocus_angle: 0.6,
                    focus_distance: 10.0,
                    ..Default::default()
                }
            }

            BuiltinScene::CheckeredSpheres => {
                let material = Lambert::new(Checker::new(
                    0.32,
                    color(0.2, 0.3, 0.1),
                    color(0.9, 0.9, 0.9),
                ))
                .shared();

                world.add(Sphere::stationary(
                    point(0.0, -10.0, 0.0),
                    10.0,
                    material.clone(),
                ));
                world.add(Sphere::stationary(
                    point(0.0, 10.0, 0.0),
                    10.0,
                    material.clone(),
                ));

                CameraSceneOptions {
                    vertical_fov: 20.0,
                    look_from: point(13.0, 2.0, 3.0),
                    look_at: point(0.0, 0.0, 0.0),
                    ..Default::default()
                }
            }

            BuiltinScene::PerlinSpheres => {
                let material = Lambert::new(NoiseTexture::new(Perlin::new(), 4.0)).shared();
                world.add(Sphere::stationary(
                    point(0.0, -1000.0, 0.0),
                    1000.0,
                    material.clone(),
                ));
                world.add(Sphere::stationary(point(0.0, 2.0, 0.0), 2.0, material));

                CameraSceneOptions {
                    vertical_fov: 20.0,
                    look_from: point(13.0, 2.0, 3.0),
                    look_at: point(0.0, 0.0, 0.0),
                    ..Default::default()
                }
            }

            BuiltinScene::Quads => {
                world.add(Quad::new(
                    point(-3.0, -2.0, 5.0),
                    arrow(0.0, 0.0, -4.0),
                    arrow(0.0, 4.0, 0.0),
                    Lambert::new(color(1.0, 0.2, 0.2)),
                ));
                world.add(Quad::new(
                    point(-2.0, -2.0, 0.0),
                    arrow(4.0, 0.0, 0.0),
                    arrow(0.0, 4.0, 0.0),
                    Lambert::new(color(0.2, 1.0, 0.2)),
                ));
                world.add(Quad::new(
                    point(3.0, -2.0, 1.0),
                    arrow(0.0, 0.0, 4.0),
                    arrow(0.0, 4.0, 0.0),
                    Lambert::new(color(0.2, 0.2, 1.0)),
                ));
                world.add(Quad::new(
                    point(-2.0, 3.0, 1.0),
                    arrow(4.0, 0.0, 0.0),
                    arrow(0.0, 0.0, 4.0),
                    Lambert::new(color(1.0, 0.5, 0.0)),
                ));
                world.add(Quad::new(
                    point(-2.0, -3.0, 5.0),
                    arrow(4.0, 0.0, 0.0),
                    arrow(0.0, 0.0, -4.0),
                    Lambert::new(color(0.2, 0.8, 0.8)),
                ));

                CameraSceneOptions {
                    vertical_fov: 80.0,
                    look_from: point(0.0, 0.0, 9.0),
                    look_at: point(0.0, 0.0, 0.0),
                    ..Default::default()
                }
            }

            BuiltinScene::SimpleLight => {
                let material_perlin = Lambert::new(NoiseTexture::new(Perlin::new(), 4.0).shared());
                world.add(Sphere::stationary(
                    point(0.0, -1000.0, 0.0),
                    1000.0,
                    material_perlin.clone(),
                ));
                world.add(Sphere::stationary(
                    point(0.0, 2.0, 0.0),
                    2.0,
                    material_perlin,
                ));

                let material_light = DiffuseLight::new(color(4.0, 4.0, 4.0)).shared();
                world.add(Sphere::stationary(
                    point(0.0, 7.0, 0.0),
                    2.0,
                    material_light.clone(),
                ));
                world.add(Quad::new(
                    point(3.0, 1.0, -2.0),
                    arrow(2.0, 0.0, 0.0),
                    arrow(0.0, 2.0, 0.0),
                    material_light,
                ));

                CameraSceneOptions {
                    vertical_fov: 20.0,
                    look_from: point(26.0, 3.0, 6.0),
                    look_at: point(0.0, 2.0, 0.0),
                    background: Background::Solid(color(0.0, 0.0, 0.0)),
                    ..Default::default()
                }
            }

            BuiltinScene::CornellBox => {
                let material_red = Lambert::new(color(0.65, 0.05, 0.05));
                let material_white = Lambert::new(color(0.73, 0.73, 0.73)).shared();
                let material_green = Lambert::new(color(0.12, 0.45, 0.15));
                let material_light = DiffuseLight::new(color(15.0, 15.0, 15.0));

                world.add(Quad::new(
                    point(555.0, 0.0, 0.0),
                    arrow(0.0, 555.0, 0.0),
                    arrow(0.0, 0.0, 555.0),
                    material_green,
                ));
                world.add(Quad::new(
                    point(0.0, 0.0, 0.0),
                    arrow(0.0, 555.0, 0.0),
                    arrow(0.0, 0.0, 555.0),
                    material_red,
                ));
                world.add(Quad::new(
                    point(343.0, 554.0, 332.0),
                    arrow(-130.0, 0.0, 0.0),
                    arrow(0.0, 0.0, -105.0),
                    material_light,
                ));
                world.add(Quad::new(
                    point(0.0, 0.0, 0.0),
                    arrow(555.0, 0.0, 0.0),
                    arrow(0.0, 0.0, 555.0),
                    material_white.clone(),
                ));
                world.add(Quad::new(
                    point(555.0, 555.0, 555.0),
                    arrow(-555.0, 0.0, 0.0),
                    arrow(0.0, 0.0, -555.0),
                    material_white.clone(),
                ));
                world.add(Quad::new(
                    point(0.0, 0.0, 555.0),
                    arrow(555.0, 0.0, 0.0),
                    arrow(0.0, 555.0, 0.0),
                    material_white.clone(),
                ));

                world.add(
                    Quad::cube(
                        point(0.0, 0.0, 0.0),
                        point(165.0, 330.0, 165.0),
                        material_white.clone(),
                    )
                    .rotate_y(15.0)
                    .translate(arrow(265.0, 0.0, 295.0)),
                );
                world.add(
                    Quad::cube(
                        point(0.0, 0.0, 0.0),
                        point(165.0, 165.0, 165.0),
                        material_white,
                    )
                    .rotate_y(-18.0)
                    .translate(arrow(130.0, 0.0, 65.0)),
                );

                CameraSceneOptions {
                    vertical_fov: 40.0,
                    look_from: point(278.0, 278.0, -800.0),
                    look_at: point(278.0, 278.0, 0.0),
                    background: Background::Solid(color(0.0, 0.0, 0.0)),
                    ..Default::default()
                }
            }

            BuiltinScene::CornellSmoke => {
                let material_red = Lambert::new(color(0.65, 0.05, 0.05));
                let material_white = Lambert::new(color(0.73, 0.73, 0.73)).shared();
                let material_green = Lambert::new(color(0.12, 0.45, 0.15));
                let material_light = DiffuseLight::new(color(7.0, 7.0, 7.0));

                world.add(Quad::new(
                    point(555.0, 0.0, 0.0),
                    arrow(0.0, 555.0, 0.0),
                    arrow(0.0, 0.0, 555.0),
                    material_green,
                ));
                world.add(Quad::new(
                    point(0.0, 0.0, 0.0),
                    arrow(0.0, 555.0, 0.0),
                    arrow(0.0, 0.0, 555.0),
                    material_red,
                ));
                world.add(Quad::new(
                    point(113.0, 554.0, 127.0),
                    arrow(330.0, 0.0, 0.0),
                    arrow(0.0, 0.0, 305.0),
                    material_light,
                ));
                world.add(Quad::new(
                    point(0.0, 0.0, 0.0),
                    arrow(555.0, 0.0, 0.0),
                    arrow(0.0, 0.0, 555.0),
                    material_white.clone(),
                ));
                world.add(Quad::new(
                    point(555.0, 555.0, 555.0),
                    arrow(-555.0, 0.0, 0.0),
                    arrow(0.0, 0.0, -555.0),
                    material_white.clone(),
                ));
                world.add(Quad::new(
                    point(0.0, 0.0, 555.0),
                    arrow(555.0, 0.0, 0.0),
                    arrow(0.0, 555.0, 0.0),
                    material_white.clone(),
                ));

                world.add(ConstantMedium::new(
                    Quad::cube(
                        point(0.0, 0.0, 0.0),
                        point(165.0, 330.0, 165.0),
                        material_white.clone(),
                    )
                    .rotate_y(15.0)
                    .translate(arrow(265.0, 0.0, 295.0)),
                    0.01,
                    Isotropic::new(color(0.0, 0.0, 0.0)),
                ));
                world.add(ConstantMedium::new(
                    Quad::cube(
                        point(0.0, 0.0, 0.0),
                        point(165.0, 165.0, 165.0),
                        material_white,
                    )
                    .rotate_y(-18.0)
                    .translate(arrow(130.0, 0.0, 65.0)),
                    0.01,
                    Isotropic::new(color(1.0, 1.0, 1.0)),
                ));

                CameraSceneOptions {
                    vertical_fov: 40.0,
                    look_from: point(278.0, 278.0, -800.0),
                    look_at: point(278.0, 278.0, 0.0),
                    background: Background::Solid(color(0.0, 0.0, 0.0)),
                    ..Default::default()
                }
            }
        };

        Scene {
            world: world.into(),
            camera,
        }
    }
}
