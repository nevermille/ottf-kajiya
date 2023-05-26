use ottf_kajiya::kajiya::Kajiya;

/// In this file, I'm testing the parsing of OpenSans-Regular file.
/// You'll see that I'm only testing the first and the last element
/// of an array, that's because I consider that there are no reasons
/// for the parsing to fail in between.

#[test]
fn parse() {
    let k = Kajiya::from_file("assets/OpenSans-Regular.ttf").unwrap();

    assert_eq!(k.sfnt_version, 0x00010000);

    check_table_records(&k);
}

fn check_table_records(k: &Kajiya) {
    assert!(k.tables.avar.is_none());
    assert_eq!(k.tables.unkn.len(), 18);
}
