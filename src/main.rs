use tokio::io;

use raytracing::image::Image;
// use raytracing::vec3::Vec3;

#[tokio::main]
async fn main() -> io::Result<()> {
    let gradient = Image::gradient(500, 500);
    let export_gradient = gradient.export_to_ppm("out/gradient.ppm");

    let black = Image::black(500, 500);
    let export_black = black.export_to_ppm("out/black.ppm");

    let white = Image::white(500, 500);
    let export_white = white.export_to_ppm("out/white.ppm");

    tokio::try_join!(export_gradient, export_black, export_white)?;

    Ok(())
}
