use godot::{engine::{image::Format, Image, ImageTexture}, prelude::*};

pub fn generate_sprite() -> Gd<ImageTexture> {
    let mut img = image::ImageBuffer::from_pixel(128, 128, image::Rgba::from([255_u8, 255, 255, 0]));
    imageproc::drawing::draw_filled_circle_mut(&mut img, (64, 64), 32, image::Rgba::from([255_u8, 0, 0, 255]));

    let dyn_img = image::DynamicImage::from(img);

    let img_data = PackedByteArray::from(dyn_img.as_bytes());
    let gd_image = Image::create_from_data(128, 128, false, Format::RGBA8, img_data)
        .unwrap();

    ImageTexture::create_from_image(gd_image).unwrap()
}
