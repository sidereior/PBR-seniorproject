mod vector;
mod rays;
mod points;

use image::{RgbImage, ImageBuffer, Rgb};

use crate::vector::vec;
use crate::points::point;
use crate::rays::ray;

//things to come back to: 
//bounding boxes, transformations, 
fn color(r: ray) -> vec {
    let direction = r.d().unit();
    let t = 0.5 * (direction.y + 1.0);
    if(sphere(vec::new(0.0,0.0,-1.0), 0.5, r)){
        let ir = 0.0 as f64;
        let ig = 0.0 as f64;
        let ib = 255 as f64;
        return vec::new(ir, ig, ib);
    }
    return vec::add(vec::mult(vec::new(1.0, 1.0, 1.0),  (1.0 - t)), vec::mult(vec::new(0.5, 0.7, 1.0),  t));
}

fn main(){
    const width: u32 = 500;
    const height: u32 = 500;
    let mut buffer: RgbImage = ImageBuffer::new(width, height);
    let startingPoint: vec = vec::new(-2.0, -1.0, -1.0);
    let rayOrigin = vec::new(0.0, 0.0, 0.0);
    let horizontal = vec::new(4.0, 0.0, 0.0);
    let vertical = vec::new(0.0, 4.0, 0.0);

    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let ra:vec = vec::add(startingPoint, vec::mult(horizontal, (x as f64 / width as f64)));
        let r:ray = ray::new(rayOrigin, vec::add(ra, vec::mult(vertical, (y as f64 / height as f64))));  
        let col = color(r);
        let red = (255 as f64 * col.x) as u8;
        let green = (255 as f64 * col.y) as u8;
        let blue = (255 as f64 * col.z) as u8;
        *pixel = Rgb([red, green, blue]);
        /*
        let rayOrigin:vec = vec::new(0.0, 0.0, 0.0);
        let rayDirection:vec = vec::subtr(vec::subtr(
        vec::subtr(vec::new(0.0, 0.0, 0.0), vec::new((width as f64)/2.0, 0.0, 0.0)), 
        vec::new(0.0, (height as f64)/2.0, 0.0)), 
        vec::new(0.0,0.0,1.0)//the 1.0 is the focal length
        );
        let raytocast: ray= ray::new(rayOrigin, rayDirection);
        let col = color(raytocast);
        let ir = (255 as f64 * col.x) as u8;
        let ig = (255 as f64 * col.y) as u8;
        let ib = (255 as f64 * col.z) as u8;
        let mut r = 255;//(255.999 * (x as f64 / (width-1) as f64)) as u8;
        let mut g = 100;//(255.999 * (y as f64 / (height-1) as f64)) as u8;
        let mut b = 0 as u8;
        /*
        if(sphere(vec::new(0.0,0.0,-1.0), 0.5, raytocast)){
            r = 0 as u8;
            g = 0 as u8;
            b = 255 as u8;
            println!("{}", "here!");
        }
        */
        *pixel = Rgb([ir, ig, ib]);
         */
        
    }
    //i think the problem is somewhere in vector 
    buffer.save("image3.png").unwrap();
    //timestamp the image
     
    /*
    //write to file system
    std::fs::write("rawout.ppm", buffer).expect("error");
    let file = File::open("rawout.ppm").expect("error on file");

    let decoder = PNMDecoder::new(reader).expect("error on reader");
    let image = decoder.read_image().expect("error on image");
    let dynamic_image = DynamicImage::ImageRgb8(image.to_rgb8());
    let path = Path::new("output.jpg");
    let output = File::create(path).expect("error on output");
    let mut encoder = JpegEncoder::new(output);
    encoder
        .encode_image(&dynamic_image)
        .expect("Failed to encode image");

     */
    //intitalize options
    //declare vector string for filenames
    //pbr init
    //pbr cleanup
    //return result with error loggging
    //start with converting ppm image to jpg
    //multithreading so that it preforms better
   
}

fn sphere(center: vec, radius: f64, r: ray) -> bool {
    let oc = vec::subtr(r.o(), center); 
    let a = r.d().dot(r.d());
    let b = 2.0 * oc.dot(r.d());
    let c = vec::dot(oc, oc.clone()) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);
    return discriminant > 0.0
}