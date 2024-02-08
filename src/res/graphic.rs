use crate::wad::DoomWadLump;
use crate::res::{self, ToImage, Image, ImageDimension, IndexedBuffer};
use std::{
    io::{Read, Cursor, Seek, SeekFrom, Error as IOError},
    array,
};
use binrw::{BinRead, Error as BinReadError};
use thiserror::Error;

/// Offsets for a Doom picture.
///
/// A "picture" can be literally anything that isn't a flat. For example,
/// sprites and wall textures. Offsets are only relevant for sprites.
///
/// Without offsets, a picture is drawn at the actor's position, starting from
/// the top left corner. This means a sprite will appear submerged into the
/// ground, unless offsets are given so that the sprite is drawn above ground.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PictureOffsets {
    /// X (horizontal) offset.
    ///
    /// Positive values move the picture left, negative values move the picture
    /// right.
    pub x: i32,
    /// Y (vertical) offset.
    ///
    /// Positive values move the picture up, negative values move the picture
    /// down.
    pub y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct GenericPicture {
    pub pixels: Vec<Option<u8>>,
    pub width: ImageDimension,
    pub height: ImageDimension,
    pub offsets: PictureOffsets,
    pub opaque: bool
}

impl GenericPicture {
    pub const OPAQUE_ALPHA: u8 = 255;
}

#[deprecated(since="0.2.0", note="Use GenericPicture (owned) instead")]
#[derive(Debug, Clone)]
pub struct DoomPicture<'wad> {
    lump: &'wad DoomWadLump
}

#[derive(Debug, Error)]
pub enum PictureConvertError {
    #[error("Error reading header: {0}")]
    HeaderReadError(BinReadError),
    #[error("Not enough offsets for all columns")]
    ColumnOffsets,
    #[error("Error reading a post of column {0}: {1}")]
    PostIO(usize, IOError),
    #[error("Error reading a post of column {0}: {1}")]
    PostBR(usize, BinReadError),
}

impl PictureConvertError {
    fn header_read_error(binread_error: BinReadError) -> PictureConvertError {
        PictureConvertError::HeaderReadError(binread_error)
    }
}

#[derive(Debug, Error)]
pub enum FlatConvertError {
    #[error("Too short! Minimum is 4096 bytes, lump has {0} bytes")]
    TooShort(usize),
}

impl GenericPicture {
    pub fn try_from_picture(lump: DoomWadLump) -> Result<Self, PictureConvertError> {
        struct DoomPicturePost {
            column: ImageDimension,
            top_delta: u8,
            pixels: Vec<u8>
        }

        #[derive(BinRead)]
        #[br(little)]
        struct DoomPicturePostPixels {
            // Post height
            height: u8,
            // Padding byte to prevent underflow
            _pre_padding: u8,
            #[br(count = height)]
            pixels: Vec<u8>,
            // Padding byte to prevent overflow
            _post_padding: u8,
        }

        #[derive(BinRead)]
        #[br(little)]
        struct PictureHeader {
            width: u16,
            height: u16,
            x_offset: i16,
            y_offset: i16,
        }

        let mut long_buffer: [u8; 4] = [0; 4];
        let mut pos = Cursor::new(&lump.data);

        let PictureHeader {width, height, x_offset, y_offset} =
            PictureHeader::read(&mut pos)
                .map_err(PictureConvertError::header_read_error)?;
        let width = width as ImageDimension;
        let height = height as ImageDimension;

        // Column offsets are relative to the start of the lump
        let column_offsets: Result<Vec<_>, PictureConvertError> = (0..width).map(|_| {
            pos.read_exact(&mut long_buffer)
                .map_err(|_| PictureConvertError::ColumnOffsets)?;
            Ok(u32::from_le_bytes(long_buffer) as u64)
        }).collect();

        let column_offsets = column_offsets?;

        let image_pixels = (width * height) as usize;
        let mut pixels = vec![None; image_pixels];
        let mut opaque_pixels: usize = 0;

        column_offsets.iter().enumerate().map(
        |(column, &offset)| -> Result<Vec<DoomPicturePost>, PictureConvertError> {
            pos.seek(SeekFrom::Start(offset))
                .map_err(|e| PictureConvertError::PostIO(column, e))?;
            let mut cur_byte = 0;
            let mut posts: Vec<DoomPicturePost> = Vec::new();
            loop {
                // Downward offset from top of patch or bottom of previous post
                pos.read_exact(array::from_mut(&mut cur_byte))
                    .map_err(|e| PictureConvertError::PostIO(column, e))?;
                let top_delta = cur_byte;
                // A "top delta" of 255 marks the end of the column
                if top_delta == 255 { break }

                let DoomPicturePostPixels { pixels, .. } =
                    DoomPicturePostPixels::read(&mut pos)
                    .map_err(|e| PictureConvertError::PostBR(column, e))?;
                posts.push(DoomPicturePost {
                    column: column as ImageDimension, top_delta, pixels
                });
            }
            Ok(posts)
        }).try_for_each(|col_posts| {
            let mut coly = 0 as ImageDimension;
            col_posts?.iter().for_each(|post| {
                let top_delta = post.top_delta as ImageDimension;
                let y = if top_delta <= coly {
                    coly + top_delta
                } else {
                    top_delta
                };
                coly = y;
                post.pixels.iter().enumerate()
                .for_each(|(pixpos, &pixel)| {
                    let pixpos = pixpos as ImageDimension;
                    if let Some(bp) = res::xy_to_bufpos(
                            post.column, y + pixpos, width, height, 1) {
                        // pixel_count[pixel as usize] += 1;
                        pixels[bp] = Some(pixel); // Index
                        opaque_pixels += 1;
                    }
                });
            });
            Ok(())
        })?;
        let opaque = opaque_pixels == image_pixels;
        Ok(GenericPicture {
            pixels,
            width,
            height,
            opaque,
            offsets: PictureOffsets { x: x_offset as i32, y: y_offset as i32 }
        })
    }
    pub fn try_from_flat(lump: DoomWadLump) -> Result<Self, FlatConvertError> {
        let opaque = true;
        let width = 64;
        let height = {
            let lump_len = lump.data.len();
            if lump_len < 4096 {
                return Err(FlatConvertError::TooShort(lump_len));
            }
            64 + (lump_len - 4096).wrapping_shr(6)
        };
        let offsets = PictureOffsets::default();
        let pixels = lump.data.iter().map(|&b| Some(b)).collect();
        Ok(Self { pixels, width, height, offsets, opaque })
    }
}

impl ToImage for GenericPicture {
    fn to_image(&self) -> Image {
        Image {
            width: self.width,
            height: self.height,
            indexed: Some(IndexedBuffer {
                buffer: {
                    if self.opaque {
                        self.pixels.iter().map(|px| px.unwrap()).collect()
                    } else {
                        self.pixels.iter().map(|px| {
                            match px.as_ref().copied() {
                                Some(i) => [i, Self::OPAQUE_ALPHA],
                                None => [0, 0],
                            }
                        }).flatten().collect()
                    }
                },
                alpha: !self.opaque
            }),
            truecolor: None,
            x: self.offsets.x,
            y: self.offsets.y,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::wad::LumpName;

    #[test]
    fn converts_opaque_patches_correctly() {
        let patch_lump = DoomWadLump {
            name: LumpName::try_from("MOSSBRK8").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/MOSSBRK8.lmp").as_slice())
        };
        let expected = Image {
            width: 128,
            height: 128,
            x: 64,
            y: 123,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/MOSSBRK8.raw").as_slice()),
                alpha: false
            }),
            truecolor: None,
        };

        let picture = GenericPicture::try_from_picture(patch_lump).unwrap();
        assert_eq!(picture.opaque, true);
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_transparent_patches_correctly() {
        let patch_lump = DoomWadLump {
            name: LumpName::try_from("GRATE").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/GRATE.lmp").as_slice())
        };
        let expected = Image {
            width: 128,
            height: 128,
            x: 64,
            y: 123,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/GRATE.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
        };

        let picture = GenericPicture::try_from_picture(patch_lump).unwrap();
        assert_eq!(picture.opaque, false);
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_tall_patches_correctly() {
        let patch_lump = DoomWadLump {
            name: LumpName::try_from("SHTGC0").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/SHTGC0.lmp").as_slice())
        };
        let expected = Image {
            width: 98,
            height: 146,
            x: -27,
            y: -22,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/SHTGC0.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
        };

        let picture = GenericPicture::try_from_picture(patch_lump).unwrap();
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_deepsea_tall_patches_correctly() {
        let patch_lump = DoomWadLump {
            name: LumpName::try_from("CYBRE1").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/CYBRE1.lmp").as_slice())
        };
        let expected = Image {
            width: 277,
            height: 335,
            x: 138,
            y: 331,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/CYBRE1.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None,
        };

        let picture = GenericPicture::try_from_picture(patch_lump).unwrap();
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }

    #[test]
    fn converts_tswgb0_correctly() {
        let patch_lump = DoomWadLump {
            name: LumpName::try_from("TSWGB0").unwrap(),
            data: Vec::from(include_bytes!("../../tests/data/TSWGB0.lmp").as_slice())
        };
        let expected = Image {
            width: 179,
            height: 333,
            x: -249,
            y: 155,
            indexed: Some(IndexedBuffer {
                buffer: Box::from(include_bytes!(
                    "../../tests/data/TSWGB0.raw").as_slice()),
                alpha: true,
            }),
            truecolor: None
        };

        let picture = GenericPicture::try_from_picture(patch_lump).unwrap();
        let image = picture.to_image();

        assert_eq!(image.width, expected.width);
        assert_eq!(image.height, expected.height);
        assert_eq!(image.x, expected.x);
        assert_eq!(image.y, expected.y);
        assert_eq!(image.indexed, expected.indexed);
        assert_eq!(image.truecolor, expected.truecolor);
    }
}
