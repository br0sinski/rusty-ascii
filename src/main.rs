mod utils;
use std::io::{stdin, stdout, Write};


fn main() {
    let mut path = String::new();
    print!("Enter Path! =)\n>");
    let _ = stdout().flush();
    stdin().read_line(&mut path).expect("Did not enter a correct string");
    let mut image = utils::load_image(path.trim()).expect("Failed to load image");


    let aspect_ratio = 0.5; 
    let width = image.width();
    let height = (image.height() as f32 * aspect_ratio) as u32;
    image = image.resize_exact(width, height, image::imageops::FilterType::Nearest);

    let width = image.width() as usize;
    let grey = utils::read_image(image);
    let ascii = utils::greyscale_to_ascii(grey);
    let ascii_art = utils::ascii_to_lines(ascii, width);
    println!("{}", ascii_art);
}