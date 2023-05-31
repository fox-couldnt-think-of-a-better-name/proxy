use std::io::Cursor;

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
};
use image::{ImageFormat, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub async fn verify_user_email(
    Path((user_id, image_hash)): Path<(u64, String)>,
) -> impl IntoResponse {
    let mut img = image::open("./assets/queers.png").unwrap();

    // Load a font.
    let font = Vec::from(include_bytes!("../assets/Carlito-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let font_size = 40.0;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    // Draw text to image and mutate it:
    draw_text_mut(
        &mut img,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        0, // Your x position in pixels
        0, // Your y position in pixels
        scale,
        &font,
        &image_hash, // Must be of type &str
    );

    let mut contents = Cursor::new(Vec::new());
    img.write_to(&mut contents, ImageFormat::Png)
        .expect("should be able to create PNG");
    return (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/png")],
        contents.get_ref().to_owned(),
    )
        .into_response();
}
