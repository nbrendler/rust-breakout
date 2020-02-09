use std::path::Path;

use crate::types::{SpriteTexture, TextureId, TextureInfo};
use luminance::texture::{GenMipmaps, Sampler, Texture};
use luminance_windowing::Surface;

pub struct AssetManager {
    tex_storage: Vec<SpriteTexture>,
    tex_count: usize,
}

impl AssetManager {
    pub fn new() -> Self {
        AssetManager {
            tex_storage: vec![],
            tex_count: 0,
        }
    }

    pub fn load_texture_image<S: Surface, P: AsRef<Path>>(
        &mut self,
        surface: &mut S,
        path: P,
    ) -> Result<TextureInfo, image::ImageError> {
        println!("Loading texture ({})", path.as_ref().display(),);
        let img = image::open(path).map(|img| img.to_rgb())?;
        let (width, height) = img.dimensions();
        let texels = img.into_raw();
        println!("Loaded: {}x{}", width, height);

        // create the luminance texture; the third argument is the number of mipmaps we want (leave it
        // to 0 for now) and the latest is the sampler to use when sampling the texels in the
        // shader (we’ll just use the default one)
        let tex = Texture::new(surface, [width, height], 0, Sampler::default())
            .expect("luminance texture creation");

        // the first argument disables mipmap generation (we don’t care so far)
        tex.upload_raw(GenMipmaps::No, &texels).unwrap();

        self.tex_storage.push(tex);
        let id = self.tex_count;
        self.tex_count += 1;

        Ok(TextureInfo::new(id, width, height))
    }

    pub fn get(&self, id: TextureId) -> Option<&SpriteTexture> {
        self.tex_storage.get(id)
    }
}
