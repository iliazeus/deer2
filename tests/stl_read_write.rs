use deer2::formats::stl::*;

use std::io::Cursor;

const UTAH_TEAPOT: &[u8] = include_bytes!("../data/stl/utah_teapot.stl");

#[test]
fn utah_teapot() {
    let source = UTAH_TEAPOT;
    let mut result = Vec::<u8>::new();

    let mut reader = Cursor::new(source);
    let mut writer = Cursor::new(&mut result);

    let model = StlModel::read_from(&mut reader).unwrap();
    model.write_to(&mut writer).unwrap();

    assert_eq!(source, result);
}
