mod diff;

use base64::encode;
use diff::*;
use image::*;
use serde_derive::*;
use wasm_bindgen::prelude::*;

static RATE: f32 = 100.0 / 256.0;

fn compute_range(r: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut i = 0;
    let mut j = 0;
    let mut acc: usize;
    let mut y1: usize;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    while i < r.len() {
        y1 = r[i];
        acc = y1;
        i += 1;
        loop {
            if i >= r.len() {
                break;
            }
            let index = r[i];
            if acc + 1 != index {
                break;
            }
            acc = index;
            i += 1;
            j += 1;
        }
        let y2 = y1 + j;
        j = 0;
        ranges.push((y1, y2));
    }
    ranges
}

fn create_encoded_rows(buf: &[u8], width: usize) -> Vec<String> {
    buf.chunks(width * 4).map(|chunk| encode(chunk)).collect()
}

fn blend_diff_area<G>(img: &mut G, ranges: Vec<(usize, usize)>, rgb: (u8, u8, u8), rate: f32)
where
    G: GenericImage<Pixel = Rgba<u8>>,
{
    for (y1, y2) in ranges {
        for y in y1..(y2 + 1) {
            for x in 0..img.dimensions().0 {
                let p = img.get_pixel(x, y as u32);
                let blended = blend(p, rgb, rate);
                img.put_pixel(x, y as u32, blended);
            }
        }
    }
}

fn create_marked_image(
    base: &mut DynamicImage,
    color: (u8, u8, u8),
    rate: f32,
    indexes: &Vec<usize>,
) {
    let range = compute_range(indexes);
    blend_diff_area(base, range, color, rate);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Result {
    before: Vec<u8>,
    after: Vec<u8>,
}

fn blend(base: Rgba<u8>, rgb: (u8, u8, u8), rate: f32) -> Rgba<u8> {
    return Rgba {
        data: [
            (base.data[0] as f32 * (1.0 - rate) + rgb.0 as f32 * (rate)) as u8,
            (base.data[1] as f32 * (1.0 - rate) + rgb.1 as f32 * (rate)) as u8,
            (base.data[2] as f32 * (1.0 - rate) + rgb.2 as f32 * (rate)) as u8,
            base.data[3],
        ],
    };
}

fn to_png(img: &DynamicImage) -> Vec<u8> {
    let mut buf = Vec::new();
    img.write_to(&mut buf, ImageOutputFormat::PNG)
        .expect("Unable to write");
    buf
}

#[wasm_bindgen]
pub fn compare(before: &[u8], after: &[u8]) -> String {
    let mut before = load_from_memory(before).expect("Unable to load image from memory");
    let mut after = load_from_memory(after).expect("Unable to load image from memory");
    let encoded_before = create_encoded_rows(&before.raw_pixels(), before.dimensions().0 as usize);
    let encoded_after = create_encoded_rows(&after.raw_pixels(), after.dimensions().0 as usize);
    let result = diff(&encoded_before, &encoded_after);
    let mut added: Vec<usize> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();
    for d in result.iter() {
        match d {
            &DiffResult::Added(ref a) => added.push(a.new_index.unwrap()),
            &DiffResult::Removed(ref r) => removed.push(r.old_index.unwrap()),
            _ => (),
        }
    }
    create_marked_image(&mut after, (99, 195, 99), RATE, &added);
    create_marked_image(&mut before, (255, 119, 119), RATE, &removed);
    serde_json::to_string(&Result {
        after: to_png(&after),
        before: to_png(&before),
    })
    .unwrap()
}
