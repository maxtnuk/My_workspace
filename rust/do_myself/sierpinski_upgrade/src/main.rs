extern crate image;
extern crate rand;

use rand::Rng;
use std::io;
use std::fs::File;
use image::Pixel;

struct Point{
    x: u32,
    y: u32,
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn draw_shape<P>(img: &mut image::ImageBuffer<P,Vec<P::Subpixel>>)
where P: Pixel + 'static
{

let mut cnt: u32=1_000_000;
let mut ptr_str=String::new();
let mut ptr_set: [u32; 4]=[0;4];
for i in 0..4{
    io::stdin().read_line(&mut ptr_str)
        .expect("failed to readline");
    ptr_set[i]=match ptr_str.trim().parse(){
        Ok(num) =>num,
        Err(_) => continue,
    };
    ptr_str.clear();
}
let pts: [Point; 3]=[
    Point {x: WIDTH/ptr_set[0],y: 0},
    Point {x: 0,y: HEIGHT/ptr_set[1]},
    Point {x: WIDTH/ptr_set[2],y: HEIGHT/ptr_set[2]},
];

let mut p=Point{x: rand::thread_rng().gen_range(0, WIDTH),
                y: rand::thread_rng().gen_range(0,HEIGHT),
};
let pixel=img[(0,0)];
while cnt>0{
    cnt=cnt-1;
    let num = rand::thread_rng().gen_range(0,3);
    p.x=(p.x+pts[num].x)/ptr_set[3];
    p.y=(p.y+pts[num].y)/ptr_set[3];
    img.put_pixel(p.x, p.y, pixel);
}
}
fn main() {
    let mut img=image::ImageBuffer::from_fn(WIDTH, HEIGHT,|x,y|{
        if x==0 && y==0{
            image::Luma([0u8])
        }else{
            image::Luma([255u8])
        }
    });
    draw_shape(&mut img);
    let ref mut result=File::create("result.png").unwrap();
    let _ = image::ImageLuma8(img).save(result,image::PNG);
}
