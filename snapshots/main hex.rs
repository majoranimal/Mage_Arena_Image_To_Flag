use std::path::PathBuf;

use image::{imageops::{dither, ColorMap, FilterType::Nearest}, DynamicImage, ImageReader, Rgb};
use num::{pow};
use rfd::FileDialog;

fn main() {
    let flag_image = get_and_load_image();
    let flag_image = image_to_flag_format(flag_image);
        // Saves the image to "flag.png"
    flag_image.save("flag.png").expect("Image failed to save");
    println!("Image saved");
    let flag_image = image_to_flag_hex(flag_image);
}

fn get_and_load_image() -> DynamicImage {
    // Opens the system file selector for the user and limits it to image files
    println!("Select a file to open");
    let file_path = FileDialog::new()
    .add_filter("image", &["avif", "bmp", "ico", "cur", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "tif", "tiff", "webp"])
    .pick_file().expect("No file was provided");
    println!("File selected");

    // Opens the image and attempts to decode it
    let flag_image = ImageReader::open(file_path).expect("Failed to open the image")
    .decode().expect("Failed to read the image");
    println!("File opened");
    return flag_image;
}

fn image_to_flag_format(flag_image: DynamicImage) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    // // Convert the image to the correct resolution for the flag
    let mut flag_image = flag_image.resize_exact(100, 66, Nearest); //.to_rgb8();
    println!("Image resized");

    // Converts the image to an RGB image buffer
    let flag_image = flag_image.as_mut_rgb8().expect("Failed to convert image to RGB");
    println!("Image converted to RGB");
    //flag_pallete = 

    // Dithers the image using the custom colormap
    dither(flag_image, &FlagColorMap);
    println!("Image dithered");

    return flag_image.clone();
}

fn image_to_flag_hex(flag_image: image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
    
    let mut pixel_num = 1;
    let mut flag_ascii = "".to_owned();
    for pixel in flag_image.pixels() {
        println!("R{} G{} B{}", pixel[0], pixel[1], pixel[2]);
        
        let pixel_ascii= match pixel {
            Rgb([0, 0, 0])       => "302e39313a302e3131", // Grey 6 (Black)
            Rgb([70, 71, 70])    => "302e37343a302e3038", // Grey 5
            Rgb([126, 125, 126]) => "302e36353a302e3039", // Grey 4
            Rgb([189, 190, 189]) => "302e33323a302e3038", // Grey 3
            Rgb([222, 223, 222]) => "302e32333a302e3037", // Grey 2
            Rgb([255, 255, 255]) => "302e30353a302e3035", // Grey 1 (White)
            Rgb([57, 0, 74])     => "302e39363a302e3235", // Purple 5
            Rgb([99, 0, 132])    => "302e39393a302e3434", // Purple 4
            Rgb([145, 0, 192])   => "302e39353a302e3631", // Purple 3
            Rgb([156, 52, 189])  => "302e39383a302e3734", // Purple 2
            Rgb([169, 109, 189]) => "302e39353a302e3839", // Purple 1
            Rgb([0, 32, 74])     => "302e38313a302e3239", // Dark blue 5
            Rgb([0, 63, 148])    => "302e38333a302e3433", // Dark blue 4
            Rgb([0, 85, 198])    => "302e38333a302e3633", // Dark blue 3
            Rgb([86, 136, 206])  => "302e38323a302e3738", // Dark blue 2
            Rgb([148, 171, 203]) => "302e38333a302e3838", // Dark blue 1
            Rgb([0, 55, 55])     => "302e36333a302e3235", // Light blue 5
            Rgb([0, 124, 121])   => "302e36323a302e3434", // Light blue 4
            Rgb([0, 199, 198])   => "302e36353a302e3632", // Light blue 3
            Rgb([57, 193, 192])  => "302e36373a302e3736", // Light blue 2
            Rgb([121, 186, 187]) => "302e36333a302e3932", // Light blue 1
            Rgb([11, 65, 0])     => "302e35333a302e3236", // Green 5
            Rgb([27, 151, 0])    => "302e35343a302e3436", // Green 4
            Rgb([33, 199, 0])    => "302e35303a302e3631", // Green 3
            Rgb([96, 195, 77])   => "302e35353a302e3737", // Green 2
            Rgb([151, 195, 143]) => "302e35323a302e3931", // Green 1
            Rgb([57, 51, 0])     => "302e33393a302e3234", // Yellow 5
            Rgb([127, 113, 0])   => "302e33353a302e3430", // Yellow 4
            Rgb([198, 179, 0])   => "302e33373a302e3539", // Yellow 3
            Rgb([195, 181, 55])  => "302e33373a302e3738", // Yellow 2
            Rgb([187, 176, 110]) => "302e33383a302e3931", // Yellow 1
            Rgb([70, 44, 0])     => "302e32323a302e3232", // Orange 5
            Rgb([118, 77, 0])    => "302e32333a302e3434", // Orange 4
            Rgb([198, 127, 0])   => "302e32333a302e3539", // Orange 3
            Rgb([198, 149, 60])  => "302e32333a302e3736", // Orange 2
            Rgb([198, 168, 119]) => "302e32343a302e3839", // Orange 1
            Rgb([49, 0, 0])      => "302e30363a302e3236", // Red 5
            Rgb([110, 0, 0])     => "302e30373a302e3434", // Red 4
            Rgb([198, 0, 0])     => "302e30333a302e3538", // Red 3
            Rgb([198, 50, 49])   => "302e31303a302e3739", // Red 2
            Rgb([192, 113, 112]) => "302e30363a302e3837", // Red 1
            _ => panic!("An invalid RGB code was provided to the hex convertor")
        };

        flag_ascii.push_str(pixel_ascii);
        if pixel_num <= 65999 {
            flag_ascii.push_str("2c");
        }
        pixel_num += 1;
        if pixel_num > 10 {
            return;
        }
    }
    
    // // Convert the image to hex
    // let dyn_image = flag_image.to_ascii_lowercase();
    // println!("Image converted to hex");

    //let flag_palette = palette::parse(hex_pallete).expect("Failed to parse pallete");
    //let flag_quantizer = quantize(flag_palette);

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

// The pallete of colours taken from the game (Higher numbers are darker)
static PALLETE: [Rgb<u8>;41] = [
    Rgb([0, 0, 0]),       // Grey 6 (Black)
    Rgb([70, 71, 70]),    // Grey 5
    Rgb([126, 125, 126]), // Grey 4
    Rgb([189, 190, 189]), // Grey 3
    Rgb([222, 223, 222]), // Grey 2
    Rgb([255, 255, 255]), // Grey 1 (White)
    Rgb([57, 0, 74]),     // Purple 5
    Rgb([99, 0, 132]),    // Purple 4
    Rgb([145, 0, 192]),   // Purple 3
    Rgb([156, 52, 189]),  // Purple 2
    Rgb([169, 109, 189]), // Purple 1
    Rgb([0, 32, 74]),     // Dark blue 5
    Rgb([0, 63, 148]),    // Dark blue 4
    Rgb([0, 85, 198]),    // Dark blue 3
    Rgb([86, 136, 206]),  // Dark blue 2
    Rgb([148, 171, 203]), // Dark blue 1
    Rgb([0, 55, 55]),     // Light blue 5
    Rgb([0, 124, 121]),   // Light blue 4
    Rgb([0, 199, 198]),   // Light blue 3
    Rgb([57, 193, 192]),  // Light blue 2
    Rgb([121, 186, 187]), // Light blue 1
    Rgb([11, 65, 0]),     // Green 5
    Rgb([27, 151, 0]),    // Green 4
    Rgb([33, 199, 0]),    // Green 3
    Rgb([96, 195, 77]),   // Green 2
    Rgb([151, 195, 143]), // Green 1
    Rgb([57, 51, 0]),     // Yellow 5
    Rgb([127, 113, 0]),   // Yellow 4
    Rgb([198, 179, 0]),   // Yellow 3
    Rgb([195, 181, 55]),  // Yellow 2
    Rgb([187, 176, 110]), // Yellow 1
    Rgb([70, 44, 0]),     // Orange 5
    Rgb([118, 77, 0]),    // Orange 4
    Rgb([198, 127, 0]),   // Orange 3
    Rgb([198, 149, 60]),  // Orange 2
    Rgb([198, 168, 119]), // Orange 1
    Rgb([49, 0, 0]),      // Red 5
    Rgb([110, 0, 0]),     // Red 4
    Rgb([198, 0, 0]),     // Red 3
    Rgb([198, 50, 49]),   // Red 2
    Rgb([192, 113, 112])  // Red 1
];

// Defines a structure to store the color difference calculated in the colormap
struct ColorDiff(f64, &'static Rgb<u8>);

// Defines the colormap to be used for dithering
pub struct FlagColorMap;

impl ColorMap for FlagColorMap {
    type Color = Rgb<u8>;

    // Calculates the difference between each colour and finds the closest match
    fn index_of(&self, color: &Rgb<u8>) -> usize {
        // Creates an array of ColorDiff's to store the color differences
        let mut color_differences: Vec<ColorDiff> = vec![];
        // For each colour in the pallet
        for pallete_color in &PALLETE {
            // Find the color difference
            let difference = (pow((color[0] as f64 - pallete_color[0] as f64) * 0.3, 2) + 
                                   pow((color[1] as f64 - pallete_color[1] as f64) * 0.59, 2) + 
                                   pow((color[2] as f64 - pallete_color[2] as f64) * 0.11, 2)).sqrt();
            // And write push it into the color difference array
            color_differences.push(ColorDiff(difference, &pallete_color)); 
            
        }
        // Get the pallete colour that is closest to the input color
        let closest_color = color_differences.iter().min_by(|a, b| a.0.partial_cmp(&b.0).expect("Failed to compare colors")).unwrap();
        // Get the array index of the pallete colour that was closest
        return PALLETE.iter().position(|&i| &i == closest_color.1).expect("Failed to find index of color");
    }

    // Return the RGB value at the provided index or None if the index is invalid
    fn lookup(&self, idx: usize) -> Option<Self::Color> {
        if idx >! PALLETE.len() && idx <! 0 {
            return Some(PALLETE[idx]);
        } else {
            return None;
        };
    }

    // Tells the Ditherer that this colormap overrides the default lookup
    fn has_lookup(&self) -> bool {
        true
    }

    // Maps the input color to the closest colour in the pallete
    fn map_color(&self, color: &mut Rgb<u8>) {
        let color_index = self.index_of(color);
        color[0] = PALLETE[color_index][0];
        color[1] = PALLETE[color_index][1];
        color[2] = PALLETE[color_index][2];
    }
}