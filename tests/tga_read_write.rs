use deer2::formats::tga::*;

use std::io::Cursor;

const LENA: &[u8] = include_bytes!("../data/tga/lena.tga");

#[test]
fn lena() {
    let source = LENA;
    let mut result = Vec::<u8>::new();

    let mut reader = Cursor::new(source);
    let mut writer = Cursor::new(&mut result);

    let bitmap = TgaBitmap::read_from(&mut reader).unwrap();
    bitmap.write_to(&mut writer).unwrap();

    assert_eq!(source, result);
}
