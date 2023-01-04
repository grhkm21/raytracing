use raytracing::image::Image;

fn test_image_ppm() -> std::io::Result<()> {
    let WIDTH = 256;
    let HEIGHT = 256;
    let mut buf = vec![vec![(0.0, 0.0, 0.0); WIDTH]; HEIGHT];
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            buf[i][j] = ((i as f64) / (WIDTH as f64 - 1.0), (j as f64) / (HEIGHT as f64 - 1.0), 0.25);
        }
    }

    let image = Image::new(HEIGHT as u64, WIDTH as u64, buf);
    image.export_to_ppm("out.ppm")
}

fn main () {
    test_image_ppm();
}
