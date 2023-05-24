use ottf_kajiya::kajiya::Kajiya;

/// In this file, I'm testing the parsing of OpenSans-Regular file.
/// You'll see that I'm only testing the first and the last element
/// of an array, that's because I consider that there are no reasons
/// for the parsing to fail in between.

#[test]
fn parse() {
    let k = Kajiya::parse("assets/OpenSans-Regular.ttf").unwrap();

    assert_eq!(k.sfnt_version, 0x00010000);
    assert_eq!(k.num_tables, 18);
    assert_eq!(k.search_range, 256);
    assert_eq!(k.entry_selector, 4);
    assert_eq!(k.range_shift, 32);
    assert_eq!(k.table_records.len(), 18);

    check_table_records(&k);
}

fn check_table_records(k: &Kajiya) {
    assert_eq!(k.table_records[0].tag, "GDEF");
    assert_eq!(k.table_records[0].checksum, 0x9eb3a424);
    assert_eq!(k.table_records[0].offset, 0x00000374);
    assert_eq!(k.table_records[0].length, 0x000001aa);

    assert_eq!(k.table_records[17].tag, "prep");
    assert_eq!(k.table_records[17].checksum, 0x85fd7be9);
    assert_eq!(k.table_records[17].offset, 0x00000798);
    assert_eq!(k.table_records[17].length, 0x0000029f);
}
