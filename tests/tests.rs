mod common;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use pretty_assertions::assert_eq;
    use raytracing::image::Image;
    use raytracing::vec3::Vec3;
    use tempfile::NamedTempFile;
    use super::common::utils::*;
    
    #[test]
    fn test_image_ppm() -> std::io::Result<()> {
        let img_width = 2;
        let img_height = 2;

        let buf = vec![vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)], vec![(0.0, 1.0, 0.0), (1.0, 1.0, 0.0)]];

        let image = Image::new(img_height as u64, img_width as u64, buf);

        // Generate temporary directory
        let file = NamedTempFile::new()?;
        let file_path = file.into_temp_path();

        image.export_to_ppm(&file_path)?;

        let mut buf = String::new();
        let mut file = File::open(file_path)?;
        file.sync_all()?;
        file.read_to_string(&mut buf)?;

        assert_eq!(buf, "P3\n2 2\n255\n255 0 0\n255 255 0\n0 0 0\n0 255 0\n");
        Ok(())
    }

}
