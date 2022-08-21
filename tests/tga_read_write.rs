use deer2::data::tga::*;
use deer2::formats::tga::*;

use std::io::Cursor;

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
