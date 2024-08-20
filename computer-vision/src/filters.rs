pub mod filters {
    use opencv::core::Size;
    use opencv::imgproc;
    use opencv::prelude::*;

    pub fn median_blur(img_ori: &Mat, ksize: i32) -> Result<Mat, opencv::Error> {
        let mut img = Mat::default();
        imgproc::median_blur(img_ori, &mut img, ksize)?;
        Ok(img)
    }
    pub fn guassian_blur(
        img_ori: &Mat,
        ksize: Size,
        sigma_x: f64,
        sigma_y: f64,
        border_type: i32,
    ) -> Result<Mat, opencv::Error> {
        let mut img = Mat::default();
        imgproc::gaussian_blur(img_ori, &mut img, ksize, sigma_x, sigma_y, border_type)?;
        Ok(img)
    }
}
