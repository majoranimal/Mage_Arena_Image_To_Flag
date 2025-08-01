//use std::path::PathBuf;

use image::{imageops::FilterType::CatmullRom, DynamicImage, ImageReader};
use rfd::FileDialog;
use color_reducer::prelude::ColorReducer;


fn main() {
    image_to_hex();
    //hex_to_image();
}

// fn hex_to_image() {

// }

fn image_to_hex() {
    // Opens the system file selector for the user (Limited to supported image formats)
    println!("Select a file to open");
    let file_path = FileDialog::new()
    .add_filter("image", &["avif", "bmp", "ico", "cur", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "tif", "tiff", "webp"])
    .pick_file().expect("No file was provided");
    println!("File selected");

    // Open and decode the image
    let dyn_image = ImageReader::open(file_path).expect("Failed to open the image")
    .decode().expect("Failed to read the image");
    println!("File opened");

    // Define the pallet of usable colors for the image 
    let palette: Vec<[u8; 3]> = vec![
        [0, 0, 0],
        [70, 71, 70],
        [126, 125, 126],
        [189, 190, 189],
        [222, 223, 222],
        [255, 255, 255],
        [57, 0, 74],
        [99, 0, 132],
        [145, 0, 192],
        [156, 52, 189],
        [169, 109, 189],
        [0, 32, 74],
        [0, 63, 148],
        [0, 85, 198],
        [86, 136, 206],
        [148, 171, 203],
        [0, 55, 55],
        [0, 124, 121],
        [0, 199, 198],
        [57, 193, 192],
        [121, 186, 187],
        [11, 65, 0],
        [27, 151, 0],
        [33, 199, 0],
        [96, 195, 77],
        [151, 195, 143],
        [57, 51, 0],
        [127, 113, 0],
        [198, 179, 0],
        [195, 181, 55],
        [187, 176, 110],
        [70, 44, 0],
        [118, 77, 0],
        [198, 127, 0],
        [198, 149, 60],
        [198, 168, 119],
        [49, 0, 0],
        [198, 0, 0],
        [198, 0, 0],
        [198, 50, 49],
        [192, 113, 112]
    ];

    // Create a ColorReducer instance
    let reducer = ColorReducer::new(palette);

    // Reduce the colors in the image
    //let dyn_image = reducer.reduce(&dyn_image).expect("Image color reduction failed");
    println!("Colours reduced");

    // Convert the image to the correct resolution for the flag
    let dyn_image = dyn_image.resize_exact(100, 66, CatmullRom); //.to_rgb8();
    println!("Image resized");
    
    // Reduce the colors a second time (The resize likely blended them)
    let dyn_image = reducer.reduce(&dyn_image).expect("Second image color reduction failed");
    println!("Colours reduced again");

    // Convert the image to hex
    //let dyn_image = dyn_image.to_rgb8().to_ascii_lowercase();
    //println!("Image converted to hex");

    dyn_image.save("flag.png").expect("Image failed to save")

    // println!("{}", hex);

    // let mut flag_hex = "".to_owned();

    // let mut byte_num = 1;
    // for b in dyn_image {
    //     if byte_num % 16 == 0 {
    //         flag_hex.push_str(&format!("{b:02x}"));
    //         // flag_hex.push_str(&format!("{b:08b},\\\n"));
    //     } else {
    //         flag_hex.push_str(&format!("{b:02x}"));
    //         // flag_hex.push_str(&format!("{b:08b},"));
    //     }
    //     byte_num += 1;
    // }
    // println!("{}", flag_hex);
}