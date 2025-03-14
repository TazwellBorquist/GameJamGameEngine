extern crate serde;
extern crate serde_json;
use self::serde::{Deserialize, Serialize};

use std::fs::read_to_string;
use std::path::PathBuf;

use log;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteSheetData {
    pub name: Option<String>,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedAssetData {
    pub name: String,
    pub texture: Option<String>,
    pub sprite_sheet: Vec<SpriteSheetData>,
}

#[derive(Debug)]
pub struct AssetLocation(pub ParsedAssetData, pub PathBuf);

pub fn load_asset_list(asset_dir: PathBuf) -> Vec<AssetLocation> {
    fn rec_load(path: PathBuf, mut assets: Vec<AssetLocation>) -> Vec<AssetLocation> {
        // Read the contents of the passed directory
        let dir_iter = match path.read_dir() {
            Ok(dir_iter) => dir_iter,
            Err(e) => {
                log!("asset_finder::load_asset_list: Could not load assets from: {:?}\n\tError message: {}", path, e);
                return assets;
            }
        };

        // Iterate through DirEntries
        for dir_entry in dir_iter {
            let entry = match dir_entry {
                Ok(x) => x,
                _ => continue,
            };

            let entry_path = entry.path();

            let entry_type = match entry.file_type() {
                Ok(t) => t,
                _ => continue,
            };

            // Explore deeper directories
            if entry_type.is_dir() {
                assets = rec_load(entry_path, assets);

            // Or find out if file is a json file
            } else if entry_type.is_file() {
                if let Some(ext) = entry_path.extension() {
                    // Add the file to assets if it is
                    if ext == "json" {
                        let cont = match read_to_string(&entry_path) {
                            Ok(x) => x,
                            _ => {
                                log!("asset_finder::load_asset_list:: Failed to read json file: {:?}", entry_path);
                                continue;
                            }
                        };

                        let data: ParsedAssetData = match serde_json::from_slice(cont.as_bytes()) {
                            Ok(x) => x,
                            Err(e) => {
                                log!("asset_finder::load_asset_list: Failed to parse json file: {:?}\n\tError: {:?}", entry_path, e);
                                continue;
                            }
                        };

                        assets.push(AssetLocation(data, entry_path))
                    }
                }
            }
        }

        assets
    }

    let assets = Vec::new();

    rec_load(asset_dir, assets)
}
