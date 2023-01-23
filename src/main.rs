use image::GenericImageView;

fn print_ascii(path: &str, scale: f32) {
    let img = image::open(path).unwrap();

    let (width, height) = img.dimensions();

    // Resize image before processing
    let resized_img = img.resize(
        (width as f32 * scale) as u32,
        (height as f32 * scale) as u32,
        image::imageops::FilterType::Nearest,
    );

    // let grayscaled = resized_img.grayscale();

    // A set of ascii chars to
    let ascii = [".", ",", ":", ";", "o", "x", "%", "#", "@", " "];

    // Simplified version of image-to-ascii library
    // Source: https://github.com/lnenad/image-to-ascii
    let mut last_y = 0;
    for pixel in resized_img.pixels() {
        if last_y != pixel.1 {
            // Print new line after the last y
            println!("");
            last_y = pixel.1;
        }

        let rgba = pixel.2;
        // Brigthness is calculated by average: (R+G+B)/3
        // Source: https://web.stanford.edu/class/cs101/image-6-grayscale.html
        let brigthness = (rgba[0] as u64 + rgba[1] as u64 + rgba[2] as u64) / 3;

        // Intensity of pixel/area is i = <0-255> then the replacement character will be map[(255-i)*(ascii len)/256]
        // Source: https://stackoverflow.com/a/32987834
        let intent = ((255 - brigthness) * (ascii.len() as u64) / 256) as usize;

        print!("{}", ascii[intent]);
    }
}

fn main() {
    let path = "rocket.png";
    print_ascii(path, 0.1);
}
