use anyhow::{Result, anyhow};
use clap::Parser;
use show_image::{ImageView, ImageInfo, event, create_window};

use ppm_image_rs::ppm::reader::PpmReader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to a P6 ppm file 
    #[arg(short, long, default_value="")]
    file_path: String, 
}


#[show_image::main]
fn main() -> Result<()> {
    let args = Args::parse();
    
    let ppm_path = args.file_path;
    if ppm_path.is_empty() {
        return Err(anyhow!("expected a path to a valid P6 ppm file"));
    }

    let mut ppm_reader = PpmReader::new(&ppm_path);
    ppm_reader.read_file()?;

    let image_data = &ppm_reader.ppm_image.image_data.unwrap();

    let image = ImageView::new(
        ImageInfo::rgb8(
            ppm_reader.ppm_image.width as u32, 
            ppm_reader.ppm_image.height as u32),
        image_data.as_slice()
    );
    let window = create_window("ppm image", Default::default()).unwrap();
    window.set_image("ppm image", image).unwrap();
    
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }

    Ok(())
}

