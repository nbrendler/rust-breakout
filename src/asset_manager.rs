use std::ops::Deref;
use std::path::Path;

use image::{ImageBuffer, Pixel};

use crate::types::TextureInfo;

struct RawImageInfo {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl<P, Container> From<ImageBuffer<P, Container>> for RawImageInfo
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]> + Into<Vec<u8>>,
{
    fn from(img: ImageBuffer<P, Container>) -> RawImageInfo {
        let (width, height) = img.dimensions();
        RawImageInfo {
            width,
            height,
            data: img.into_raw().into(),
        }
    }
}

pub struct AssetManager {
    tex_storage: Vec<RawImageInfo>,
    tex_count: usize,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager {
            tex_storage: vec![],
            tex_count: 0,
        }
    }

    pub fn load_texture_image<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<TextureInfo, image::ImageError> {
        println!("Loading texture ({})", path.as_ref().display(),);
        let img = image::open(path).map(|img| img.to_rgb())?;
        let (width, height) = img.dimensions();

        self.tex_storage.push(RawImageInfo::from(img));
        let id = self.tex_count;
        self.tex_count += 1;

        Ok(TextureInfo::new(id, width, height))
    }

    pub fn upload_textures<F>(&mut self, mut callback: F)
    where
        F: FnMut(u32, u32, Vec<u8>) -> (),
    {
        for raw in self.tex_storage.drain(..) {
            callback(raw.width, raw.height, raw.data);
        }
    }
}
