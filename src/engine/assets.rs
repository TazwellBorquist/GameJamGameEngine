extern crate sdl2;

use log;

use sdl2::{
    video::WindowContext,

    image::LoadTexture, 
    render::{
        Texture,
        TextureCreator,
    }
};

use std::{
    collections::HashMap, 
    path::PathBuf
};

const ASSETS_DIR: &str = "assets/";

fn find_all_assets(dir: PathBuf, mut file_names: Vec<String>) -> Vec<String> {
    // Read the contents of the passed directory
    let assets = match dir.read_dir() {
        Ok(dir_iter) => dir_iter,
        Err(e) => {
            log!("Error: Could not load assets from: {:?}\n\tError message: {}", dir, e);
            return file_names;
        }
    };

    // Iterate through entries of the directory
    for entry in assets {
        let file = match entry {
            Ok(t) => t,
            Err(e) => {
                log!("Error while loading asset: {}", e);
                continue;
            }
        };

        // Check the type of the file
        if let Ok(t) = file.file_type() {
            // Recurse with this function if it's another directory
            if t.is_dir() {
                file_names = find_all_assets(file.path(), file_names);
            
            // Add the file if it's a "png" or "jpg"
            } else if t.is_file() {
                if let Some(ext) = file.path().extension() {
                    if ext == "jpg" || ext == "png" {
                        let file_name = file.path().to_string_lossy().to_string();
                        file_names.push(file_name);
                    }
                }

            }
        }
    }

    file_names
}

pub struct Assets<'r> {
    //texture_creator: TextureCreator<WindowCanvas>,
    pub textures: HashMap<String, Texture<'r>>
}

impl<'r> Assets<'r> {
    pub fn new(texture_creator: &'r TextureCreator<WindowContext>) -> Self {
        let mut textures = HashMap::new();

        let texture_names = find_all_assets(PathBuf::from(ASSETS_DIR), Vec::new());

        for name in texture_names {
            let texture = match texture_creator.load_texture(&name) {
                Ok(t) => t,
                Err(e) => {
                    log!("Error loading texture: \"{}\" message: {}", name, e);
                    continue;
                }
            };

            textures.insert(name, texture);
        };

        Self {
            //texture_creator: texture_creator,
            textures: textures
        }
    }
}