use crate::sprite::SpriteType;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{BlendMode, Texture, TextureCreator};
use sdl2::video:: WindowContext;
use std::fs::File;
use std::collections::HashMap;

//Uses png crate to load bytes from a PNG file and then
//copies those bytes into a texture
//Works only on PNG images
pub fn load_texture<'a>(
    path: &str,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let img_file = File::open(&path);

    match img_file {
        Ok(file) => {
            let decoder = png::Decoder::new(file);
            let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
            //Bytes read in from the image, copoy this into the SDL texture
            let mut png_buffer = vec![0u8; reader.output_buffer_size()];
            // Read the next frame. An APNG might contain multiple frames.
            let info = reader
                .next_frame(&mut png_buffer)
                .map_err(|e| e.to_string())?;

            //Create empty SDL texture
            let mut tex = texture_creator
                .create_texture_streaming(PixelFormatEnum::ARGB8888, info.width, info.height)
                .map_err(|e| e.to_string())?;
            //Enable Alpha so we can have transparency
            tex.set_blend_mode(BlendMode::Blend);

            //Write pixels into texture
            tex.with_lock(None, |pixels: &mut [u8], _pitch: usize| {
                for i in 0..(reader.output_buffer_size() as usize) {
                    if i % 4 == 0 {
                        pixels[i] = png_buffer[i + 2];
                        pixels[i + 1] = png_buffer[i + 1];
                        pixels[i + 2] = png_buffer[i];
                        pixels[i + 3] = png_buffer[i + 3];
                    }
                }
            })
            .map_err(|e| e.to_string())?;

            Ok(tex)
        }
        Err(msg) => {
            eprintln!("{msg}");
            eprintln!("Cannot open image: {path}");
            Err(String::from("Failed to load texture"))
        }
    }
}

pub fn load_assets<'a>(
	texture_creator: &'a TextureCreator<WindowContext>
	) -> Result<HashMap<SpriteType, Texture<'a>>, String> {	
	let mut sprite_images = HashMap::<SpriteType, Texture<'a>>::new();

	sprite_images.insert(SpriteType::Kart1,
						 load_texture("assets/images/kart1.png", texture_creator)
						 .map_err(|e| e.to_string())?
						 );

	sprite_images.insert(SpriteType::Kart2,
						 load_texture("assets/images/kart2.png", texture_creator)
						 .map_err(|e| e.to_string())?
						 );

	sprite_images.insert(SpriteType::Checkpoint1,
						 load_texture("assets/images/kart-checkpoint1.png", texture_creator)
						 .map_err(|e| e.to_string())?
						 );

	sprite_images.insert(SpriteType::Checkpoint2,
						 load_texture("assets/images/kart-checkpoint2.png", texture_creator)
						 .map_err(|e| e.to_string())?
						 );

	sprite_images.insert(SpriteType::Banana,
						 load_texture("assets/images/enemies/banana.png", texture_creator)
						 .map_err(|e| e.to_string())?
						 );

	Ok(sprite_images)
}
