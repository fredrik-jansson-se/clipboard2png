use std::io::BufRead;

use clap::Parser;

#[derive(Debug, Parser)]
struct Opts {
    image_dir: std::path::PathBuf,
}
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let mut clipboard = arboard::Clipboard::new()?;

    println!(
        "Will store png images in {}",
        opts.image_dir.to_str().unwrap()
    );

    loop {
        let mut input = String::new();
        println!("Enter image name (enter to quit)");
        std::io::stdin().lock().read_line(&mut input)?;
        if &input == "\n" {
            break;
        }
        let input = input.trim();

        let file_name = opts.image_dir.join(format!("{input}.png"));

        let Ok(image_data) = clipboard.get_image() else {
            eprintln!("No image in the clipboard");
            continue;
        };

        let image = image::RgbaImage::from_raw(
            image_data.width as _,
            image_data.height as _,
            image_data.bytes.as_ref().to_vec(),
        );

        let Some(image) = image else {
            eprintln!("Failed to crate image from clipboard" );
            continue;
        };

        image.save_with_format(&file_name, image::ImageFormat::Png)?;
        println!("Stored clipboard image in {}", file_name.to_str().unwrap());
    }
    Ok(())
}
