mod createWindow;

use std::io::Write;

use rfd::FileDialog;

use std::fs::File;
use std::io::BufWriter;

use image::{GenericImageView, Rgba};
use crate::createWindow::create_window;

use std::thread;
fn main() {
    let files = FileDialog::new()
        .add_filter("image", &["png"])
        .set_directory("/")
        .pick_file();

    let binding = files.unwrap();
    let path = binding.as_path().to_str().unwrap();

    let image = image::open(path).expect("Нету изображения");
    let (width, height) = image.dimensions();

    let gradient: Vec<char> = vec![' ','.',':','!','/','r','(','|','1','Z','4','H','9','W','8','$','@'];

/*    let crWin = thread::spawn(move ||{
        create_window(image.clone());
    });

    crWin.join().unwrap();*/

    let file = match File::create("example.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Ошибка при создании файла: {}", e);
            return;
        }
    };
    let mut writer = BufWriter::new(file);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let brightness = calculate_brightness(pixel);

            let element = (brightness / 16f32) as usize;
            println!("{}",brightness / 16f32);
            write!(&mut writer, "{}", gradient[element]).unwrap();
        }
        write!(&mut writer, "\n").unwrap();
    }

}

fn calculate_brightness(pixel: Rgba<u8>) -> f32 {
    let Rgba([r, g, b, _]) = pixel;
    0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32
}
