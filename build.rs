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
    let mut levels = 0;
    for i in 0.. {
        levels = i;
        if std::fs::File::open(&format!("assets/level{}.png", i)).is_err() { break; }
    }
    writeln!(f, "pub static LEVELS: [LevelDef; {}] = [", levels).unwrap();
    for id in 0..levels {
        let val = image::open(&format!("assets/level{}.png", id)).unwrap().into_rgba();
        embed_lvl(&val, &mut data).write_entry(&mut f);
    }
    writeln!(f, "];").unwrap();
    //data.extend_from_slice("Visit selic.re for details!  💙💙".as_bytes());

    let comp = lz4::block::compress(&data, lz4::block::CompressionMode::HIGHCOMPRESSION(12).into(), false).unwrap();

    std::fs::write(bin_path, &comp).unwrap();
    std::fs::write("test.bin", &comp).unwrap();
    std::fs::write("raw.bin", &data).unwrap();

    writeln!(f, r#"pub static mut GFX_DATA: [u8; {0:}] = [0; {0:}];"#, data.len()).unwrap();
    writeln!(f, r#"pub static GFX_DATA_LZ4: [u8; {}] = *include_bytes!(concat!(env!("OUT_DIR"), "/gfx.bin"));"#, comp.len()).unwrap();
    writeln!(f, "pub static PAL_DATA: [u32; {}] = {:#08X?};", pal.len(), pal).unwrap();
}
fn embed_lvl(image: &image::RgbaImage, data: &mut Vec<u8>) -> LevelDef {
    let offset = data.len();
    data.extend(image.pixels().map(|c| {
        c.0[0]
    }));
    LevelDef {
        offset,
        width: image.width() as _,
        height: image.height() as _
    }
}
fn embed_fg(image: &image::RgbaImage, data: &mut Vec<u8>, pal: &mut Vec<u32>) -> DataDef {
    let mut palette = HashMap::new();
    let offset = data.len();
    let pal_offset = pal.len();
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
