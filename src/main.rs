mod vec3;

use indicatif::ProgressIterator;

use vec3::{ Color, write_color };

fn main() {

    // Image
    let image_width = 256;
    let image_height = 256;
    let max_color = 256;

    // Render

    // Header
    println!("P3\n{} {}\n{}", image_width, image_height, max_color);

    // body
    for j in (0..image_height).rev().progress() {
        for i in 0..image_width {
            let pixel_color = Color {
                x: i as f64 / 255.0,
                y: j as f64 / 255.0,
                z: 0.25,
            };

            write_color(pixel_color);
        }
    }

    eprintln!("Done.");
}
