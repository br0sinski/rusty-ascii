use image::{GenericImage, GenericImageView, ImageBuffer, ImageError, DynamicImage, open};

pub const LETTERS: &[&str] = &[
    " ", ".", "`", "^", "\"", "\\", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_",
    "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u",
    "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p", "d", "b",
    "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "$", "@",
];


pub fn load_image(file_path: &str) -> Result<DynamicImage, ImageError> {
    open(file_path)
}

pub fn read_image(image: DynamicImage) -> Vec<u8> {
    let gray_image = image.grayscale();
    gray_image.to_luma8().into_vec()
}

pub fn greyscale_to_ascii(greyed_image: Vec<u8>) -> Vec<&'static str> {
    let scale = LETTERS.len() - 1;
    greyed_image
        .iter()
        .map(|&pixel| {
            let idx = (pixel as usize * scale) / 255;
            LETTERS[idx]
        })
        .collect()
}

pub fn ascii_to_lines(ascii: Vec<&str>, width: usize) -> String {
    ascii.chunks(width)
        .map(|line| line.concat())
        .collect::<Vec<String>>()
        .join("\n")
}