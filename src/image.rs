use std::io::{stderr, Write};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

use crate::vec3::Vec3;

pub struct Image {
    height: u64,
    width: u64,
    data: Vec<Vec<Vec3>>,
}

impl Image {
    pub fn new(height: u64, width: u64, data: Vec<Vec<Vec3>>) -> Image {
        Image {
            height,
            width,
            data,
        }
    }

    pub fn gradient(height: u64, width: u64) -> Image {
        let mut buf = Vec::new();
        for i in 0..width {
            let row = (0..height)
                .map(|j| {
                    let px = (i as f64) / (width as f64 - 1.0);
                    let py = (j as f64) / (height as f64 - 1.0);
                    let pz = 0.25;
                    Vec3::new(px, py, pz)
                })
                .collect::<Vec<Vec3>>();
            buf.push(row);
        }
        Image::new(height, width, buf)
    }

    pub fn white(height: u64, width: u64) -> Image {
        let buf = vec![vec![Vec3::new(255.0, 255.0, 255.0); width as usize]; height as usize];
        Image::new(height, width, buf)
    }

    pub fn black(height: u64, width: u64) -> Image {
        // Fix vec3! macro
        let buf = vec![vec![vec3![0.0, 0.0, 0.0]; width as usize]; height as usize];
        Image::new(height, width, buf)
    }

    pub async fn export_to_ppm(&self, file_name: impl AsRef<Path>) -> io::Result<()> {
        let mut file = File::create(file_name.as_ref()).await?;
        let mut stderr_lock = stderr().lock();

        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(header.as_bytes()).await?;
        for j in (0..self.height).rev() {
            if (self.height - 1 - j) % 100 == 0 {
                stderr_lock.write_all(
                    format!(
                        "[{:?}] Progress: {} / {} lines\n",
                        file_name.as_ref(),
                        self.height - 1 - j,
                        self.height
                    )
                    .as_bytes(),
                )?;
            }
            for i in 0..self.width {
                file.write_all(self.data[i as usize][j as usize].to_string().as_bytes())
                    .await?;
            }
        }

        stderr_lock.write_all(format!("\nExported to {:?}.\n", file_name.as_ref()).as_bytes())?;

        Ok(())
    }
}
