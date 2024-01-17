use image::{GenericImageView, Pixel, Rgb};
use std::collections::HashMap;
use std::fmt::Write;
use wasm_bindgen::prelude::*;

// A simple function to convert RGB to HSB
fn rgb_to_hsb(rgb: Rgb<u8>) -> (f32, f32, f32) {
    let r = rgb[0] as f32 / 255.0;
    let g = rgb[1] as f32 / 255.0;
    let b = rgb[2] as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let saturation = if max == 0.0 { 0.0 } else { delta / max };

    (hue, saturation, max)
}

// Function to calculate difference in saturation, brightness, and hue
fn sbh_diff(hsb1: (f32, f32, f32), hsb2: (f32, f32, f32)) -> (f32, f32, f32) {
    let hue_diff = (hsb1.0 - hsb2.0).abs();
    let saturation_diff = (hsb1.1 - hsb2.1).abs();
    let brightness_diff = (hsb1.2 - hsb2.2).abs();
    (saturation_diff, brightness_diff, hue_diff)
}

#[wasm_bindgen]
pub fn process_image(image_data: &[u8]) -> Vec<String> {
    let img = image::load_from_memory(image_data).expect("Failed to load image");
    let mut color_count: HashMap<[u8; 3], u32> = HashMap::new();

    for (_, _, pixel) in img.pixels() {
        let rgb = pixel.to_rgb().0;
        *color_count.entry(rgb).or_insert(0) += 1;
    }

    let all_colors = color_count.keys().collect::<Vec<_>>();

    let mut unique_colors = Vec::new();

    for &color in all_colors {
        let color_hsb = rgb_to_hsb(Rgb(color)); // Corrected this line
        if unique_colors.iter().all(|&unique| {
            let (sat_diff, bri_diff, hue_diff) = sbh_diff(color_hsb, rgb_to_hsb(Rgb(unique)));
            sat_diff > 0.1 && bri_diff > 0.1 && hue_diff > 10.0 // Adjust thresholds as needed
        }) {
            unique_colors.push(color);
            if unique_colors.len() >= 20 {
                break;
            } // Limit to 5 unique colors
        }
    }

    let mut results = Vec::new();

    for color in unique_colors {
        let mut hex_color = String::new();
        write!(
            &mut hex_color,
            "#{:02X}{:02X}{:02X}",
            color[0], color[1], color[2]
        )
        .unwrap();
        results.push(format!("Unique color: {}", hex_color));
    }

    results
}
