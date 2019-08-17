pub type ImageBuffer = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub struct Font<'a> {
    font: rusttype::Font<'a>,
    size: u32,
    scale: rusttype::Scale,
}

impl<'a> Font<'a> {
    pub fn new(size: u32) -> Font<'a> {
        let font_data: &[u8] = include_bytes!("../fonts/Bitter-Bold.ttf");
        let font =
            rusttype::Font::from_bytes(font_data).expect("Could not load the given font file");
        let scale = rusttype::Scale {
            x: (size as f32) * 1.5,
            y: (size as f32) * 1.5,
        };

        Font { font, size, scale }
    }
}

pub fn process<F>(img: ImageBuffer, txt: &str, font: &Font, inc: F) -> ImageBuffer
where
    F: Fn() -> (),
{
    let mut out = image::RgbImage::new(img.width() * font.size, img.height() * font.size);
    let mut txt_it = txt
        .chars()
        .chain(" ".chars())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .into_iter()
        .cycle();

    for ((x, y, rgb), c) in img.enumerate_pixels().zip(txt_it) {
        imageproc::drawing::draw_text_mut(
            &mut out,
            *rgb,
            x * font.size,
            y * font.size,
            font.scale,
            &font.font,
            &c,
        );
        inc();
    }

    out
}
