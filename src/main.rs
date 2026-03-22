//! A command line application to either read a valid P6 PPM file and show the image in a window; 
//! or, write a test gradient image at a specified location - both using the reader and writer from
//! this crate. 

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use show_image::{ImageView, ImageInfo, event, create_window};

use ppm_image_rs::ppm::{reader::PpmReader, writer::{PpmWriter, RgbColor}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Read a PPM file 
    Read {
        /// path to a P6 ppm file 
        #[arg(short, long, default_value="")]
        file_path: String, 
    },
    
    /// Write a PPM file 
    WriteGradient {
        /// path to write the  P6 ppm file to 
        #[arg(short, long, default_value="")]
        file_path: String, 
    },
}


#[show_image::main]
fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Read { file_path } => {
            if file_path.is_empty() {
                eprintln!("expected a non-empty path to a valid P6 PPM file!");
                return Err(anyhow!("expected a path to a valid P6 ppm file"));
            }

            let mut ppm_reader = PpmReader::new(file_path);
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
        },

        Commands::WriteGradient { file_path } => {
            if file_path.is_empty() {
                eprintln!("expected a non-empty path to write a valid P6 PPM file to!");
                return Err(anyhow!("expected a non-empty path to write a valid P6 PPM file to"));
            }
    
            let width: u16 = 200;
            let height: u16 = 100; 
            let data_size: usize = 200 * 100 * 3;
            
            let mut writer = PpmWriter::new(width as usize, height as usize, 255, &file_path.clone());
            writer.set_empty_image_data(data_size);

            // generate some image content - in this case a 200x100 gradient
            for y in (0..height).rev() {
                for x in 0..width {
                    let r: f32 = f32::from(x) / f32::from(width);
                    let g: f32 = f32::from(y) / f32::from(height);
                    let b: f32 = 0.2;
                    let ir: u8 = (255.99 * r) as u8;
                    let ig: u8 = (255.99 * g) as u8;
                    let ib: u8 = (255.99 * b) as u8;
                    
                    writer.set_rgb_at_coordinate(RgbColor{
                        r: ir, g: ig, b: ib,
                    }, x as usize, y as usize);
                }
            }

            writer.write_output_file().context("error writing the output PPM file")?;
            println!("wrote a P6 PPM file, with a gradient, to: {:?}", &file_path);
        },
    }

    Ok(())
}

