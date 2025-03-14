use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub mod sprite;

mod asset_finder;
use self::asset_finder::SpriteSheetData;

use std::cell::LazyCell;
use std::collections::HashMap;
use std::rc::Rc;

use log;

static ASSET_DIR: &str = "assets/";
const MISSING_TEXTURE: LazyCell<String> = LazyCell::new(|| "miku".to_string());

pub type EngineTex<'a> = Texture<'a>;
pub type SpriteSheet = Vec<Option<Rect>>;

impl From<SpriteSheetData> for Rect {
    fn from(value: SpriteSheetData) -> Self {
        Rect::new(value.x, value.y, value.width, value.height)
    }
}

struct Asset<'a> {
    texture: Rc<EngineTex<'a>>,
    sprite_sheets: Rc<SpriteSheet>,
    sprite_names: HashMap<String, usize>,
}

pub struct AssetCollection<'a> {
    assets: HashMap<String, Asset<'a>>,
}

impl<'a> AssetCollection<'a> {
    pub fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        let alist = asset_finder::load_asset_list(ASSET_DIR.into());

        let mut assets: HashMap<String, Asset<'a>> = HashMap::new();

        for a in alist {
            let texture = match a.0.texture {
                Some(tname) => match tc.load_texture(a.1.with_file_name(tname)) {
                    Ok(x) => Rc::new(x),
                    Err(e) => {
                        log!("AssetCollection::new(): Failed to load texture for asset: {}\n\tError message: {}", a.0.name, e);
                        continue;
                    }
                },
                None => {
                    log!(
                        "AssetCollection::new(): No texture exists for asset {}",
                        a.0.name
                    );
                    continue;
                }
            };

            let mut sprite_sheets_raw: Vec<Option<Rect>> = vec![None];
            let mut sprite_names: HashMap<String, usize> = HashMap::new();
            for mut ss in a.0.sprite_sheet {
                if ss.name.is_some() {
                    sprite_names.insert(ss.name.take().unwrap(), sprite_sheets_raw.len() - 1);
                }
                sprite_sheets_raw.push(Some(ss.into()));
            }

            let sprite_sheets = Rc::new(sprite_sheets_raw);

            assets.insert(
                a.0.name,
                Asset {
                    texture,
                    sprite_sheets,
                    sprite_names,
                },
            );
        }

        log!(
            "AssetCollection::new(): Done loading! {} Asset(s) successfully loaded!",
            assets.len()
        );

        Self { assets: assets }
    }

    pub fn texture(&self, asset_name: &String) -> Rc<Texture<'a>> {
        if self.assets.contains_key(asset_name) {
            self.assets[asset_name].texture.clone()
        } else {
            self.assets[&*MISSING_TEXTURE].texture.clone()
        }
    }

    pub fn sprite_sheet(&self, asset_name: &String) -> Rc<Vec<Option<Rect>>> {
        if self.assets.contains_key(asset_name) {
            self.assets[asset_name].sprite_sheets.clone()
        } else {
            self.assets[&*MISSING_TEXTURE].sprite_sheets.clone()
        }
    }

    pub fn sprite_id_from_name(&self, asset_name: &String, sprite_name: &String) -> Option<usize> {
        if self.assets.contains_key(asset_name) {
            if self.assets[asset_name]
                .sprite_names
                .contains_key(sprite_name)
            {
                Some(self.assets[asset_name].sprite_names[sprite_name])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_sprite_names(&self, asset_name: &String) -> Option<Vec<&String>> {
        if self.assets.contains_key(asset_name) {
            Some(self.assets[asset_name].sprite_names.keys().collect())
        } else {
            None
        }
    }
}
