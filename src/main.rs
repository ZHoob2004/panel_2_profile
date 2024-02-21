use image::{GenericImage, ImageBuffer};
use regex::Regex;
fn main() -> anyhow::Result<()> {
    // Grab the latest index html
    let html_content = reqwest::blocking::get("https://qwantz.com/index.php")?.text()?;

    // Define the regular expression patterns
    let random_regex =
        Regex::new(r#"<a href="(http://www\.qwantz\.com/index\.php\?comic=\d+)">.+?<\/a>"#)?;
    let img_regex =
        Regex::new(r#"<img src="(comics\/comic2-\d+?.png)" class="comic" title=".*?">"#)?;

    if let Some(captures) = random_regex.captures(&html_content) {
        // Extract the first captured substring
        if let Some(matched_str) = captures.get(1) {
            let html_content = reqwest::blocking::get(matched_str.as_str())?.text()?;
            if let Some(captures) = img_regex.captures(&html_content) {
                // Extract the first captured substring
                if let Some(matched_str) = captures.get(1) {
                    let image_url = format!("https://qwantz.com/{}", matched_str.as_str());

                    // Download the target comic and crop it on the fly because why not
                    let panel_2 =
                        image::load_from_memory(&reqwest::blocking::get(image_url)?.bytes()?)?
                            .crop(245, 1, 127, 240);

                    // Create a new blank square image buffer
                    let mut output_canvas = ImageBuffer::new(240, 240);

                    // fill the whole buffer in white
                    for pixel in output_canvas.enumerate_pixels_mut() {
                        *pixel.2 = image::Rgba([255, 255, 255, 255]);
                    }

                    // Copy the cropped image to the new enlarged buffer
                    output_canvas.copy_from(&panel_2, 0, 0)?;

                    // do some jank pixel maniupulation to fix some stray lines
                    for pixel in output_canvas.enumerate_pixels_mut() {
                        if (pixel.0 == 0 && pixel.1 < 127) | (pixel.0 < 128 && pixel.1 == 0) {
                            *pixel.2 = image::Rgba([255, 255, 255, 255]);
                        }
                    }

                    // write the output buffer to a file for testing purposes.
                    output_canvas.save("./profile.png")?;
                }
            }
        }
    }
    Ok(())
}
