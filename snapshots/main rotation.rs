use std::{env::{self, home_dir}, path::Path};
use image::{imageops::{dither, ColorMap, FilterType::Nearest}, DynamicImage, ImageReader, Rgb, RgbImage};
use num::{pow, ToPrimitive};
use regashii::{Key, KeyKind, KeyName, Registry, ValueName};
use rfd::FileDialog;

fn main() {
    let flag_image = get_and_load_image();
    let flag_image = image_to_flag_format(flag_image);
        // Saves the image to "flag.png"
    flag_image.save("flag.png").expect("Image failed to save");
    println!("Image saved");
    let flag_ascii = image_to_flag_hex(flag_image);
    match env::consts::OS {
        "macos" => panic!("This script hasn't been adapted for Mac"),
        "windows" => write_to_windows_registry(),
        "linux" => write_to_proton_registry(flag_ascii.into_bytes()),
        _ => panic!("Operating system not recognised")
    }
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

fn image_to_flag_hex(flag_image: image::ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
    let mut flag_grid = Box::new([Rgb([0, 0, 0]);100*66]); //Box::new([0;100*66]);

    let mut pixel_num = 1;
    let mut array_loc = 1;

    for pixel in flag_image.pixels() {
        if array_loc < (pixel_num / 66) {
            array_loc += 1;
        }
        //println!("R{} G{} B{}", pixel[0], pixel[1], pixel[2]);

        let pixel_row = (pixel_num / 100) + 1;
        let mut pixel_column = (pixel_num % 100);

        if pixel_column == 0 {
            pixel_column += 10;
        }

        flag_grid[((pixel_column - 1) * 66) + pixel_row] = *pixel;

        pixel_num += 1;
    }

    // let mut img = RgbImage::new(100, 66);

    // for y in 1..66 {
    //     for x in 1..100 {
    //         img.put_pixel(x, y, flag_grid[(x+((y-1))*100).to_usize().expect("Location to usize failed")]);
    //     }
    // }

    img.save("test-flag.png").expect("Failed to save");

    //println!("{}", flag_ascii);
    let mut flag_ascii = "".to_owned();

    for pixel in flag_grid.iter() {
        let pixel_ascii= match pixel {
            Rgb([0, 0, 0])       => "0.91:0.11", // Grey 6 (Black)
            Rgb([70, 71, 70])    => "0.74:0.08", // Grey 5
            Rgb([126, 125, 126]) => "0.65:0.09", // Grey 4
            Rgb([189, 190, 189]) => "0.32:0.08", // Grey 3
            Rgb([222, 223, 222]) => "0.23:0.07", // Grey 2
            Rgb([255, 255, 255]) => "0.05:0.05", // Grey 1 (White)
            Rgb([57, 0, 74])     => "0.96:0.25", // Purple 5
            Rgb([99, 0, 132])    => "0.99:0.44", // Purple 4
            Rgb([145, 0, 192])   => "0.95:0.61", // Purple 3
            Rgb([156, 52, 189])  => "0.98:0.74", // Purple 2
            Rgb([169, 109, 189]) => "0.95:0.89", // Purple 1
            Rgb([0, 32, 74])     => "0.81:0.29", // Dark blue 5
            Rgb([0, 63, 148])    => "0.83:0.43", // Dark blue 4
            Rgb([0, 85, 198])    => "0.83:0.63", // Dark blue 3
            Rgb([86, 136, 206])  => "0.82:0.78", // Dark blue 2
            Rgb([148, 171, 203]) => "0.83:0.88", // Dark blue 1
            Rgb([0, 55, 55])     => "0.63:0.25", // Light blue 5
            Rgb([0, 124, 121])   => "0.62:0.44", // Light blue 4
            Rgb([0, 199, 198])   => "0.65:0.62", // Light blue 3
            Rgb([57, 193, 192])  => "0.67:0.76", // Light blue 2
            Rgb([121, 186, 187]) => "0.63:0.92", // Light blue 1
            Rgb([11, 65, 0])     => "0.53:0.26", // Green 5
            Rgb([27, 151, 0])    => "0.54:0.46", // Green 4
            Rgb([33, 199, 0])    => "0.50:0.61", // Green 3
            Rgb([96, 195, 77])   => "0.55:0.77", // Green 2
            Rgb([151, 195, 143]) => "0.52:0.91", // Green 1
            Rgb([57, 51, 0])     => "0.39:0.24", // Yellow 5
            Rgb([127, 113, 0])   => "0.35:0.40", // Yellow 4
            Rgb([198, 179, 0])   => "0.37:0.59", // Yellow 3
            Rgb([195, 181, 55])  => "0.37:0.78", // Yellow 2
            Rgb([187, 176, 110]) => "0.38:0.91", // Yellow 1
            Rgb([70, 44, 0])     => "0.22:0.22", // Orange 5
            Rgb([118, 77, 0])    => "0.23:0.44", // Orange 4
            Rgb([198, 127, 0])   => "0.23:0.59", // Orange 3
            Rgb([198, 149, 60])  => "0.23:0.76", // Orange 2
            Rgb([198, 168, 119]) => "0.24:0.89", // Orange 1
            Rgb([49, 0, 0])      => "0.06:0.26", // Red 5
            Rgb([110, 0, 0])     => "0.07:0.44", // Red 4
            Rgb([198, 0, 0])     => "0.03:0.58", // Red 3
            Rgb([198, 50, 49])   => "0.10:0.79", // Red 2
            Rgb([192, 113, 112]) => "0.06:0.87", // Red 1
            _ => panic!("An invalid RGB code was provided to the hex convertor")
        };

        flag_ascii.push_str(pixel_ascii);
        if pixel_num < 6600 {
            flag_ascii.push_str(",");
        //print!("{} ", pixel_num)
        }
    };

    return flag_ascii;
    
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

fn write_to_windows_registry() {
    // let registry = Registry::new(regashii::Format::Regedit5)
    // .with(
    //     r"HKEY_CURRENT_USER\Software\jrsjams\MageArena\flagGrid_h3042110417", 
    //     Key::new()
    //     .with(ValueName::Default, regashii::Value::Hex { kind: (), bytes: () })
    // );
    // registry.serialize_file("sample.reg");
}

fn write_to_proton_registry(flag_bytes: Vec<u8>) {
    let registry_path = home_dir().expect("Home directory could not be found").join(Path::new(".local/share/Steam/steamapps/compatdata/3716600/pfx/user.reg"));

    let registry = Registry::deserialize_file(registry_path.iter()).unwrap();

        registry.serialize_file("user.reg.backup").expect("Failed to serialize the proton registry backup");

    let registry = registry.with(
        KeyName::new(r"Software\\jrsjams\\MageArena"), 
        Key::new()
        .with(ValueName::named("flagGrid_h3042110417"), regashii::Value::Hex { kind: regashii::Kind::Binary, bytes: flag_bytes })
    );

    registry.serialize_file(registry_path).expect("Failed to serialize the proton registry");
    // for (key_name, key) in registry.keys() {
    //     match key.kind() {
    //         KeyKind::Delete => {
    //             println!("\n{key_name:?}  (delete)");
    //         }
    //         KeyKind::Add => {
    //             println!("\n{key_name:?}  (add)");
    //             for (value_name, value) in key.values() {
    //                 println!("  {value_name:?} = {value:?}");
    //             }
    //         }
    //         KeyKind::Replace => {
    //             println!("\n{key_name:?}  (replace)");
    //             let desired_key = match format!("{key_name:?}").as_ref() {
    //                 r"Software\\jrsjams\\MageArena" => 1,
    //                 _ => 0,
    //             };

    //             if desired_key == 1 {
    //                 for (value_name, value) in key.values() {
    //                     println!("  {value_name:?} = {value:?}");
    //                     match format!("{value_name:?}").as_ref() {
    //                         "flagGrid_h3042110417" => println!("{value_name:?}"),
    //                         _ => ()
    //                     };
    //                 }
    //             }
    //         }
    //    }
    //}

    // let registry = Registry::new(regashii::Format::Regedit5)
    // .with(
    //     r"HKEY_CURRENT_USER\Software\jrsjams\MageArena", 
    //     Key::new()
    //     .with(ValueName::named("flagGrid_h3042110417"), regashii::Value::Hex { kind: regashii::Kind::Binary, bytes: flag_bytes })
    // );
}

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