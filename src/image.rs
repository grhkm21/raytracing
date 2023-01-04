use std::{fs::File, io::Write};

pub struct Image {
    height: u64,
    width: u64,
    data: Vec<Vec<(f64, f64, f64)>>
}

impl Image {
    pub fn new(height: u64, width: u64, data: Vec<Vec<(f64, f64, f64)>>) -> Image {
        Image {
            height,
            width,
            data,
        }
    }

    pub fn export_to_ppm<S>(&self, file_name: S) -> std::io::Result<()> where S: ToString {
        let mut file = File::create(file_name.to_string())?;
        
        file.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let (r, g, b) = self.data[i as usize][j as usize];
                let ir = (r * 255.999).floor() as u64;
                let ig = (g * 255.999).floor() as u64;
                let ib = (b * 255.999).floor() as u64;
                file.write_all(format!("{ir} {ig} {ib}\n").as_bytes())?;
            }
        }
        
        Ok(())
    }
}
