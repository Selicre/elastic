use std::path::Path;

use std::collections::HashMap;
use image::GenericImageView;
use std::io;
use std::io::Write;
use std::fs::File;
use std::env;

#[derive(Debug)]
struct DataDef {
    offset: usize,
    pal: usize,
}
impl DataDef {
    fn write(&self, mut into: impl io::Write, name: &str) {
        writeln!(into, "pub const {}: DataDef = {:?};", name, self).unwrap();
    }
}

#[derive(Debug)]
struct LevelDef {
    offset: usize,
    width: u8,
    height: u8
}
impl LevelDef {
    #[allow(unused)]
    fn write(&self, mut into: impl io::Write, name: &str) {
        writeln!(into, "pub const {}: LevelDef = {:?};", name, self).unwrap();
    }
    fn write_entry(&self, mut into: impl io::Write) {
        writeln!(into, "{:?},", self).unwrap();
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gfx.rs");
    let bin_path = Path::new(&out_dir).join("gfx.bin");
    let mut f = File::create(&dest_path).unwrap();

    let mut data = vec![];
    let mut pal = vec![];

    let val = image::open("assets/gfx.png").unwrap().into_rgba();
    embed_fg(&val, &mut data, &mut pal).write(&mut f, "MAIN_GFX");
    let val = image::open("assets/thinface.png").unwrap().into_rgba();
    embed_font(&val, &mut f, "THINFACE");
    let levels = [
        "level01",
        "level02",
        "level03",
        "level04",
        "level06",
        "level07",
        "level08",
        "level14",
        "level05",
        "level09",
        "level12",
        "level11",
        "level13",
        "level15",
        "level10",
    ];
    writeln!(f, "pub static LEVELS: [LevelDef; {}] = [", levels.len()).unwrap();
    //let aa = std::fs::read("assets/map.json").unwrap();
    //embed_lvl_json(&aa, &mut data).write_entry(&mut f);
    for id in levels.iter() {
        //let val = image::open(&format!("assets/level{}.png", id)).unwrap().into_rgba();
        //embed_lvl(&aa, &format!("assets/level{}.json", id), &val, &mut data).write_entry(&mut f);

        let val = std::fs::read(&format!("levels/{}.json", id)).unwrap();
        embed_lvl_json(&val, &mut data).write_entry(&mut f);
    }
    writeln!(f, "];").unwrap();

    let comp = lz4::block::compress(&data, lz4::block::CompressionMode::HIGHCOMPRESSION(12).into(), false).unwrap();

    std::fs::write(bin_path, &comp).unwrap();
    std::fs::write("test.bin", &comp).unwrap();
    std::fs::write("raw.bin", &data).unwrap();

    writeln!(f, r#"pub static mut GFX_DATA: [u8; {0:}] = [0; {0:}];"#, data.len()).unwrap();
    writeln!(f, r#"pub static GFX_DATA_LZ4: [u8; {}] = *include_bytes!(concat!(env!("OUT_DIR"), "/gfx.bin"));"#, comp.len()).unwrap();
    writeln!(f, "pub static PAL_DATA: [u32; {}] = {:#08X?};", pal.len(), pal).unwrap();
}

fn embed_lvl_json(json: &[u8], data: &mut Vec<u8>) -> LevelDef {
    let offset = data.len();
    let v: serde_json::Value = serde_json::from_slice(json).unwrap();
    let l = v["layers"][0]["data"].as_array().unwrap();
    data.extend(l.iter().map(|gid| {
        let gid = gid.as_u64().unwrap();
        let c = gid & 0x1F;
        let hflip = gid & 0x80000000 != 0;
        let vflip = gid & 0x40000000 != 0;
        let transpose = gid & 0x20000000 != 0;
        let mask = ((transpose as u8) << 7) | ((hflip as u8) << 6) | ((vflip as u8) << 5);
        mask | (c - 1) as u8
    }));
    let width = v["width"].as_u64().unwrap() as _;
    let height = v["height"].as_u64().unwrap() as _;
    LevelDef {
        offset,
        width,
        height
    }
}
fn embed_fg(image: &image::RgbaImage, data: &mut Vec<u8>, pal: &mut Vec<u32>) -> DataDef {
    let mut palette = HashMap::new();
    let offset = data.len();
    let pal_offset = pal.len();
    let bg = 0x3c245dffu32.swap_bytes();
    let bg_w = 0x1896e9ffu32.swap_bytes();
    palette.insert(bg, 0);
    pal.push(bg);
    palette.insert(bg_w, 1);
    pal.push(bg_w);
    for ty in 0..image.height()/16 {
        for tx in 0..image.width()/16 {
            data.extend(image.view(tx*16, ty*16, 16, 16).pixels().map(|(_,_,c)| {
                let c = u32::from_le_bytes(c.0);
                let len = palette.len();
                let id = palette.entry(c).or_insert_with(|| {
                    pal.push(c);
                    len
                });
                *id as u8
            }));
        }
    }
    DataDef {
        offset,
        pal: pal_offset,
    }
}
fn embed_font(image: &image::RgbaImage, mut f: impl io::Write, name: &str) {
    writeln!(f, "pub static {}: [u16; {}] = [", name, image.width() / 4).unwrap();
    for ty in 0..image.height()/8 {
        for tx in 0..image.width()/4 {
            let val = image.view(tx*4, ty*8, 3, 5).pixels().map(|(_,_,c)| {
                 u32::from_le_bytes(c.0) == 0xFF000000
            }).fold(0u16, |a,e| (a << 1) | e as u16);
            writeln!(f, "    {:#018b},", val).unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}
