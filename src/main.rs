extern crate clap;
extern crate image;
extern crate cgmath;
extern crate rand;
extern crate pbr;
extern crate rtnw_rs;

use std::fs::File;
use std::f32;
use std::rc::Rc;
use clap::{App, Arg};
use image::ColorType;
use image::png::PNGEncoder;
use image::GenericImageView;
use cgmath::prelude::*;
use cgmath::vec3;

use rtnw_rs::renderer::*;
use rtnw_rs::material::*;
use rtnw_rs::texture::*;
use rtnw_rs::shape::*;
use rtnw_rs::transform::*;

fn random_scene() -> HitableList {
    let mut list = HitableList::new();

    list.hitable.push(Box::new(
        Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0,
        Rc::new(
            Lambertian {
                albedo: Box::new(CheckerTexture::new(
                    Box::new(ConstantTexture::new(vec3(0.2, 0.3, 0.1))),
                    Box::new(ConstantTexture::new(vec3(0.9, 0.9, 0.9)))
                ))
            }
        ))
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = vec3((a as f32) + 0.9 * rand::random::<f32>(), 0.2, (b as f32) + 0.9 * rand::random::<f32>());
            if (center - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                list.hitable.push(
                    if choose_mat < 0.8 {
                        Box::new(MovingSphere::new(
                            center,
                            center + vec3(0.0, rand::random::<f32>(), 0.0),
                            0.0,
                            1.0,
                            0.2,
                            Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(
                                rand::random::<f32>() * rand::random::<f32>(),
                                rand::random::<f32>() * rand::random::<f32>(),
                                rand::random::<f32>() * rand::random::<f32>()
                            )))})
                        ))
                    } else if choose_mat < 0.95 {
                        Box::new(Sphere::new(center, 0.2,
                            Rc::new(Metal::new(
                                vec3(
                                    0.5 * (1.0 + rand::random::<f32>()),
                                    0.5 * (1.0 + rand::random::<f32>()),
                                    0.5 * (1.0 + rand::random::<f32>())
                                ),
                                0.5 * rand::random::<f32>()
                            ))
                        ))
                    } else {
                        Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric{ref_idx: 1.5})))
                    }
                );
            }
        }
    }

    list.hitable.push(Box::new(
        Sphere::new(
            vec3(0.0, 1.0, 0.0), 1.0,
            Rc::new(Dielectric{ref_idx: 1.5})
        )
    ));
    list.hitable.push(Box::new(
        Sphere::new(
            vec3(-4.0, 1.0, 0.0), 1.0,
            Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.4, 0.2, 0.1)))})
        )
    ));
    list.hitable.push(Box::new(
        Sphere::new(
            vec3(4.0, 1.0, 0.0), 1.0,
            Rc::new(Metal{albedo: vec3(0.7, 0.6, 0.5), fuzz: 0.0})
        )
    ));

    list
}

fn sunrise_scene() -> HitableList {
    let mut list = HitableList::new();

    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0), 1000.0 - 0.01,
        Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.1, 0.8, 0.1)))})
    )));

    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0), 1000.0,
        Rc::new(Dielectric{ref_idx: 1.05})
    )));

    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, 0.0, -100.0), 10.0,
        Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.9, 0.1, 0.1)))})
    )));

    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, 0.0, -100.0), 11.0,
        Rc::new(Dielectric{ref_idx: 1.5})
    )));

    for (d, r) in [(14.0, 2.0), (16.0, 1.5), (17.5, 1.0)].iter() {
        for i in 0..8 {
            let th = 2.0 * f32::consts::PI / 8.0 * (i as f32);
            list.hitable.push(Box::new(Sphere::new(
                vec3(th.cos() * d, th.sin() * d, -100.0), *r,
                Rc::new(Metal{albedo: vec3(1.0, 0.5, 0.5), fuzz: 0.0})
            )));
        }
    }

    for _ in 0..25 {
        let r = 1000.2;

        let th_range = f32::consts::PI * 0.001;
        let th = -th_range + rand::random::<f32>() * th_range * 2.0;
        let center = vec3(-th.sin() * r, th.cos() * r, 0.0);

        let th_range = f32::consts::PI * 0.005;
        let th = -rand::random::<f32>() * th_range;
        let center = vec3(center.x, th.cos() * center.y, th.sin() * center.y) - vec3(0.0, 1000.0, 0.0);

        let choose_mat = rand::random::<f32>();
        list.hitable.push(Box::new(Sphere::new(center, 0.2,
            if choose_mat < 0.8 {
                Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(
                    rand::random::<f32>() * rand::random::<f32>(),
                    rand::random::<f32>() * rand::random::<f32>(),
                    rand::random::<f32>() * rand::random::<f32>()
                )))})
            } else if choose_mat < 0.95 {
                Rc::new(Metal::new(
                        vec3(
                        0.5 * (1.0 + rand::random::<f32>()),
                        0.5 * (1.0 + rand::random::<f32>()),
                        0.5 * (1.0 + rand::random::<f32>())
                    ),
                    0.5 * rand::random::<f32>()
                ))
            } else {
                Rc::new(Dielectric{ref_idx: 1.5})
            }
        )));
    }

    list
}

fn two_perlin_spheres() -> HitableList {
    let mut list = HitableList::new();

    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0), 1000.0,
        Rc::new(Lambertian{albedo: Box::new(NoiseTexture::new(3.0))})
    )));
    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, 2.0, 0.0), 2.0,
        Rc::new(Lambertian{albedo: Box::new(NoiseTexture::new(6.0))})
    )));

    list
}

fn simple_light() -> HitableList {
    let mut list = HitableList::new();

    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0), 1000.0,
        Rc::new(Lambertian{albedo: Box::new(NoiseTexture::new(3.0))})
    )));
    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, 2.0, 0.0), 2.0,
        Rc::new(Lambertian{albedo: Box::new(NoiseTexture::new(6.0))})
    )));
    list.hitable.push(Box::new(Sphere::new(
        vec3(0.0, 7.0, 0.0), 2.0,
        Rc::new(DiffuseLight{emit: Box::new(ConstantTexture::new(vec3(4.0, 4.0, 4.0)))})
    )));
    list.hitable.push(Box::new(XYRect::new(
        3.0, 5.0, 1.0, 3.0, -2.0,
        Rc::new(DiffuseLight{emit: Box::new(ConstantTexture::new(vec3(4.0, 4.0, 4.0)))})
    )));

    list
}

fn cornell_box() -> HitableList {
    let mut list = HitableList::new();

    let white = Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.73, 0.73, 0.73)))});

    list.hitable.push(Box::new(FlipNormals{ptr:Box::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0,
        Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.12, 0.45, 0.15)))})
    ))}));
    list.hitable.push(Box::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0,
        Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.65, 0.05, 0.05)))})
    )));
    list.hitable.push(Box::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0,
        Rc::new(DiffuseLight{emit: Box::new(ConstantTexture::new(vec3(15.0, 15.0, 15.0)))})
    )));
    list.hitable.push(Box::new(FlipNormals{ptr: Box::new(XZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, white.clone()
    ))}));
    list.hitable.push(Box::new(XZRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, white.clone()
    )));
    list.hitable.push(Box::new(FlipNormals{ptr: Box::new(XYRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, white.clone()
    ))}));

    list.hitable.push(
        Box::new(
            Translate::new(
                Box::new(
                    RotateY::new(
                        Box::new(
                            MyBox::new(
                                vec3(0.0, 0.0, 0.0),
                                vec3(165.0, 165.0, 165.0),
                                white.clone()
                            )
                        ),
                        -18.0
                    )
                ),
                vec3(130.0, 0.0, 65.0)
            )
        )
    );

    list.hitable.push(
        Box::new(
            Translate::new(
                Box::new(
                    RotateY::new(
                        Box::new(
                            MyBox::new(
                                vec3(0.0, 0.0, 0.0),
                                vec3(165.0, 330.0, 165.0),
                                white.clone()
                            )
                        ),
                        15.0
                    )
                ),
                vec3(265.0, 0.0, 295.0)
            )
        )
    );

    list
}

fn cornell_smoke() -> HitableList {
    let mut list = HitableList::new();

    let red = Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.65, 0.05, 0.05)))});
    let white = Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.73, 0.73, 0.73)))});
    let green = Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.12, 0.45, 0.15)))});
    let light = Rc::new(DiffuseLight{emit: Box::new(ConstantTexture::new(vec3(7.0, 7.0, 7.0)))});

    list.hitable.push(Box::new(FlipNormals{ptr:Box::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green.clone()
    ))}));
    list.hitable.push(Box::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, red.clone()
    )));
    list.hitable.push(Box::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light.clone()
    )));
    list.hitable.push(Box::new(FlipNormals{ptr: Box::new(XZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, white.clone()
    ))}));
    list.hitable.push(Box::new(XZRect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, white.clone()
    )));
    list.hitable.push(Box::new(FlipNormals{ptr: Box::new(XYRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, white.clone()
    ))}));

    let b1 = Box::new(
        Translate::new(
            Box::new(
                RotateY::new(
                    Box::new(
                        MyBox::new(
                            vec3(0.0, 0.0, 0.0),
                            vec3(165.0, 165.0, 165.0),
                            white.clone()
                        )
                    ),
                    -18.0
                )
            ),
            vec3(130.0, 0.0, 65.0)
        )
    );

    let b2 = Box::new(
        Translate::new(
            Box::new(
                RotateY::new(
                    Box::new(
                        MyBox::new(
                            vec3(0.0, 0.0, 0.0),
                            vec3(165.0, 330.0, 165.0),
                            white.clone()
                        )
                    ),
                    15.0
                )
            ),
            vec3(265.0, 0.0, 295.0)
        )
    );

    list.hitable.push(
        Box::new(ConstantMedium::new(b1, 0.01, Box::new(
            ConstantTexture::new(vec3(1.0, 1.0, 1.0))
        )))
    );
    list.hitable.push(
        Box::new(ConstantMedium::new(b2, 0.01, Box::new(
            ConstantTexture::new(vec3(0.0, 0.0, 0.0))
        )))
    );

    list
}

fn final_scene() -> HitableList {
    let nb = 20;
    let mut list = HitableList::new();
    let mut boxlist = Vec::<Rc<dyn Hitable>>::new();
    let mut boxlist2 = Vec::<Rc<dyn Hitable>>::new();
    let white = Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.73, 0.73, 0.73)))});
    let ground = Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.48, 0.83, 0.53)))});

    for i in 0..nb {
        for j in 0..nb {
            let i = i as f32;
            let j = j as f32;
            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rand::random::<f32>() + 0.01);
            let z1 = z0 + w;
            boxlist.push(Rc::new(MyBox::new(vec3(x0, y0, z0), vec3(x1, y1, z1), ground.clone())));
        }
    }

    list.hitable.push(Box::new(BVHNode::new(&mut boxlist, 0.0, 0.0)));

    let light = Rc::new(DiffuseLight{emit: Box::new(ConstantTexture::new(vec3(7.0, 7.0, 7.0)))});
    list.hitable.push(Box::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light)));

    let center = vec3(400.0, 400.0, 200.0);
    list.hitable.push(Box::new(MovingSphere::new(center, center + vec3(30.0, 0.0, 0.0), 0.0, 1.0, 50.0, Rc::new(Lambertian{albedo: Box::new(ConstantTexture::new(vec3(0.7, 0.3, 0.1)))}))));

    list.hitable.push(Box::new(Sphere::new(vec3(260.0, 150.0, 45.0), 50.0, Rc::new(Dielectric{ref_idx: 1.5}))));
    list.hitable.push(Box::new(Sphere::new(vec3(0.0, 150.0, 145.0), 50.0, Rc::new(Metal::new(vec3(0.8, 0.8, 0.9), 10.0)))));

    let boundary = Sphere::new(vec3(360.0, 150.0, 145.0), 70.0, Rc::new(Dielectric{ref_idx: 1.5}));
    list.hitable.push(Box::new(boundary.clone()));
    list.hitable.push(Box::new(ConstantMedium::new(Box::new(boundary.clone()), 0.2, Box::new(ConstantTexture::new(vec3(0.2, 0.4, 0.9))))));

    let boundary = Sphere::new(vec3(0.0, 0.0, 0.0), 5000.0, Rc::new(Dielectric{ref_idx: 1.5}));
    list.hitable.push(Box::new(ConstantMedium::new(Box::new(boundary.clone()), 0.0001, Box::new(ConstantTexture::new(vec3(1.0, 1.0, 1.0))))));

    let img = image::open("2k_jupiter.jpg").unwrap();
    list.hitable.push(Box::new(Sphere::new(vec3(400.0, 200.0, 400.0), 100.0, Rc::new(Lambertian{albedo: Box::new(
        ImageTexture::new(img.raw_pixels(), img.width() as usize, img.height() as usize)
    )}))));

    list.hitable.push(Box::new(Sphere::new(vec3(220.0, 280.0, 300.0), 80.0, Rc::new(Lambertian{albedo: Box::new(NoiseTexture::new(0.1))}))));

    let ns = 1000;
    for _j in 0..ns {
        boxlist2.push(Rc::new(Sphere::new(vec3(165.0 * rand::random::<f32>(), 165.0 * rand::random::<f32>(), 165.0 * rand::random::<f32>()), 10.0, white.clone())));
    }

    list.hitable.push(Box::new(Translate::new(Box::new(RotateY::new(Box::new(BVHNode::new(&mut boxlist2, 0.0, 1.0)), 15.0)), vec3(-100.0, 270.0, 395.0))));

    list
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(
        &pixels,
        bounds.0 as u32, bounds.1 as u32,
        ColorType::RGB(8)
    )?;

    Ok(())
}

fn main() {
    let matches = App::new("rtnw")
        .version("0.1.0")
        .author("blackspotbear")
        .arg(Arg::with_name("sample")
            .short("s")
            .long("sample")
            .value_name("SAMPLE")
            .help("number of sampling per pixel")
            .required(false)
            .default_value("500")
            .takes_value(true))
        .arg(Arg::with_name("SCENE")
            .help("scene name")
            .required(false)
            .default_value("finalscene")
            .possible_values(&[
                "twoperlinspheres",
                "simplelight",
                "cornellbox",
                "cornellsmoke",
                "randomscene",
                "finalscene",
                "sunrise"])
            .index(1))
        .get_matches();

    let ns = matches.value_of("sample").unwrap().parse::<i32>().unwrap();

    let (nx, ny, world, lookfrom, lookat, dist_to_focus, aperture, vfov, bg) = match matches.value_of("SCENE").unwrap() {
        "twoperlinspheres" => (
            640, 480,
            two_perlin_spheres(),
            vec3(13.0, 2.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            None,
            0.0,
            20.0,
            Background::Sky
        ),
        "simplelight" => (
            640, 480,
            simple_light(),
            vec3(26.0, 4.0, 6.0),
            vec3(0.0, 2.0, 0.0),
            None,
            0.0,
            20.0,
            Background::Black
        ),
        "cornellbox" => (
            640, 480,
            cornell_box(),
            vec3(278.0, 278.0, -800.0),
            vec3(278.0, 278.0, 0.0),
            Some(10.0),
            0.0,
            40.0,
            Background::Black
        ),
        "cornellsmoke" => (
            640, 480,
            cornell_smoke(),
            vec3(278.0, 278.0, -800.0),
            vec3(278.0, 278.0, 0.0),
            Some(10.0),
            0.0,
            40.0,
            Background::Black
        ),
        "randomscene" => (
            640, 480,
            random_scene(),
            vec3(12.0, 2.0, 3.0),
            vec3(0.0, 0.1, 0.0),
            None,
            0.1,
            20.0,
            Background::Sky
        ),
        "sunrise" => (
            640, 360,
            sunrise_scene(),
            vec3(0.0, 1.0, 3.0),
            vec3(0.0, 2.0, -100.0),
            None,
            0.01,
            20.0,
            Background::Sky
        ),
        _ => (
            640, 640,
            final_scene(),
            vec3(420.0, 280.0, -440.0 -160.0),
            vec3(260.0, 280.0, 0.0),
            Some(10.0),
            0.0,
            40.0,
            Background::Black
        )
    };

    let dist_to_focus = dist_to_focus.unwrap_or((lookfrom - lookat).magnitude() * 0.7);

    let cam = Camera::new(lookfrom, lookat, vec3(0.0, 1.0, 0.0), vfov, (nx as f32) / (ny as f32), aperture, dist_to_focus, 0.0, 1.0);

    let mut pb = pbr::ProgressBar::new((nx * ny) as u64);
    pb.format("[=>-]");

    let mut buf = Vec::<u8>::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = vec3(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as f32) + rand::random::<f32>()) / (nx as f32);
                let v = ((j as f32) + rand::random::<f32>()) / (ny as f32);
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0, bg);
            }
            col /= ns as f32;
            col.x = col.x.min(1.0);
            col.y = col.y.min(1.0);
            col.z = col.z.min(1.0);
            col = vec3(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            buf.push((col.x * 255.0) as u8);
            buf.push((col.y * 255.0) as u8);
            buf.push((col.z * 255.0) as u8);
            pb.inc();
        }
    }

    pb.finish_print("done");

    write_image( "out.png", &buf, (nx, ny)).unwrap();
}
