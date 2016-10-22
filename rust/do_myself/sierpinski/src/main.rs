extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;

struct Point{
    x: u32,
    y: u32,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 =600;

fn main() {
    let mut img =image::ImageBuffer::from_fn(WIDTH,HEIGHT,|x,y|{
        if x==0 && y==0{
            image::Luma([0u8])
        }else{
            image::Luma([255u8])
        }
    });
    let ref mut before=File::create("before.png").unwrap();
    let _ = image::ImageLuma8(img.clone()).save(before,image::PNG);
    let mut cnt: u32=1_000_000;

    let pts: [Point; 3]=[
        Point {x: WIDTH/2,y: 0},
        Point {x: 0, y: HEIGHT},
        Point {x: WIDTH, y: HEIGHT},
    ];

    let mut p=Point{x: rand::thread_rng().gen_range(0, WIDTH),
                    y: rand::thread_rng().gen_range(0,HEIGHT),
    };

    let pixel=img[(0,0)];
    while cnt>0{
        cnt=cnt-1;
        let num = rand::thread_rng().gen_range(0,3);
        p.x=(p.x+pts[num].x)/2;
        p.y=(p.y+pts[num].y)/2;
        img.put_pixel(p.x, p.y, pixel);
    }

    let ref mut fout=File::create("tri.png").unwrap();
    let _ = image::ImageLuma8(img).save(fout,image::PNG);
}
