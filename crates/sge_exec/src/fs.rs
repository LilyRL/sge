use futures::channel::oneshot;
use glium::texture::RawImage2d;
use sge_audio::{SoundLoadError, SoundRef};
use sge_image::{ImageRef, LoadImageError};
use sge_programs::{LoadProgramError, ProgramRef};
use sge_text::{FontRef, LoadFontError};
use sge_textures::{ImageFormat, LoadTextureError, SgeTexture, TextureRef};
use std::io::Cursor;

pub type LoadingTexture = Option<Result<TextureRef, LoadTextureError>>;

struct DecodedImage {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

struct ProgramSources {
    vertex: String,
    fragment: String,
}

fn decode_image(bytes: &[u8], format: ImageFormat) -> Result<DecodedImage, LoadTextureError> {
    let image = image::load(Cursor::new(bytes), format)
        .map_err(LoadTextureError::Image)?
        .to_rgba8();
    let (width, height) = image.dimensions();
    Ok(DecodedImage {
        rgba: image.into_raw(),
        width,
        height,
    })
}

fn upload_image(img: DecodedImage) -> Result<TextureRef, LoadTextureError> {
    let raw = RawImage2d::from_raw_rgba(img.rgba, (img.width, img.height));
    SgeTexture::from_raw(raw)
        .map(|t| t.create())
        .map_err(LoadTextureError::Engine)
}

pub async fn load_texture(
    path: impl AsRef<str> + Send + 'static,
) -> Result<TextureRef, LoadTextureError> {
    let format = ImageFormat::from_path(path.as_ref()).unwrap_or(ImageFormat::Png);
    let (tx, rx) = oneshot::channel::<Result<DecodedImage, LoadTextureError>>();
    rayon::spawn(move || {
        let result = std::fs::read(path.as_ref())
            .map_err(LoadTextureError::Io)
            .and_then(|bytes| decode_image(&bytes, format));
        let _ = tx.send(result);
    });

    let decoded = rx
        .await
        .map_err(|_| LoadTextureError::Other("sender dropped"))??;
    upload_image(decoded)
}

pub async fn load_texture_from_bytes(
    bytes: impl Into<Vec<u8>> + Send + 'static,
    format: ImageFormat,
) -> Result<TextureRef, LoadTextureError> {
    let (tx, rx) = oneshot::channel::<Result<DecodedImage, LoadTextureError>>();
    let bytes = bytes.into();
    rayon::spawn(move || {
        let _ = tx.send(decode_image(&bytes, format));
    });
    let decoded = rx
        .await
        .map_err(|_| LoadTextureError::Other("sender dropped"))??;
    upload_image(decoded)
}

pub async fn load_program(
    vertex: impl AsRef<str> + Send + 'static,
    fragment: impl AsRef<str> + Send + 'static,
) -> Result<ProgramRef, LoadProgramError> {
    let (tx, rx) = oneshot::channel::<Result<ProgramSources, LoadProgramError>>();
    rayon::spawn(move || {
        let result = (|| {
            let vertex = std::fs::read_to_string(vertex.as_ref()).map_err(LoadProgramError::Io)?;
            let fragment =
                std::fs::read_to_string(fragment.as_ref()).map_err(LoadProgramError::Io)?;
            Ok(ProgramSources { vertex, fragment })
        })();
        let _ = tx.send(result);
    });

    let sources = rx
        .await
        .map_err(|_| LoadProgramError::Other("sender dropped"))??;
    sge_programs::load_program_sync(&sources.vertex, &sources.fragment)
        .map_err(LoadProgramError::Create)
}

pub async fn load_program_from_strings(
    vertex: impl Into<String> + Send + 'static,
    fragment: impl Into<String> + Send + 'static,
) -> Result<ProgramRef, LoadProgramError> {
    sge_programs::load_program_sync(&vertex.into(), &fragment.into())
        .map_err(LoadProgramError::Create)
}

pub async fn load_file(path: impl AsRef<str> + Send + 'static) -> Result<Vec<u8>, std::io::Error> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let _ = tx.send(std::fs::read(path.as_ref()));
    });
    rx.await
        .map_err(|_| std::io::Error::other("sender dropped"))?
}

pub async fn load_image(
    path: impl AsRef<str> + Send + 'static,
) -> Result<ImageRef, LoadImageError> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let result = std::fs::read(path.as_ref())
            .map_err(LoadImageError::Io)
            .and_then(|bytes| sge_image::load_image_sync(&bytes, ImageFormat::Png));
        let _ = tx.send(result);
    });
    rx.await
        .map_err(|_| LoadImageError::Other("sender dropped"))?
}

pub async fn load_sound(
    path: impl AsRef<str> + Send + 'static,
) -> Result<SoundRef, SoundLoadError> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let _ = tx.send(sge_audio::load_sound_sync(path.as_ref()));
    });
    rx.await
        .map_err(|_| SoundLoadError::Other("sender dropped"))?
}

pub async fn load_sound_from_bytes(
    bytes: impl Into<Vec<u8>> + Send + 'static,
) -> Result<SoundRef, SoundLoadError> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let _ = tx.send(sge_audio::load_sound_from_bytes_sync(bytes.into()));
    });
    rx.await
        .map_err(|_| SoundLoadError::Other("sender dropped"))?
}

pub async fn load_font(path: impl AsRef<str> + Send + 'static) -> Result<FontRef, LoadFontError> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let result = std::fs::read(path.as_ref())
            .map_err(LoadFontError::Io)
            .and_then(|bytes| sge_text::load_font_sync(&bytes));
        let _ = tx.send(result);
    });
    rx.await
        .map_err(|_| LoadFontError::Other("sender dropped"))?
}

pub async fn load_font_from_bytes(
    bytes: impl Into<Vec<u8>> + Send + 'static,
) -> Result<FontRef, LoadFontError> {
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        let _ = tx.send(sge_text::load_font_sync(&bytes.into()));
    });
    rx.await
        .map_err(|_| LoadFontError::Other("sender dropped"))?
}
