use winnow::winnow;

use std::fs::read_to_string;

#[test]
fn winnow_source_1() {
    let winnow_size = 50;
    // let us = read_to_string("tests/data/msp2/us.c").unwrap();
    // let them = read_to_string("tests/data/msp2/them.c").unwrap();
    let us = read_to_string("tests/data/play.scm").unwrap();
    let them = read_to_string("tests/data/play1.scm").unwrap();
    let us = winnow(&us, winnow_size);
    let them = winnow(&them, winnow_size);
    println!("{:?}", us);
    println!("{:?}", them);
    println!("{:?}", us.len());
    println!("{:?}", them.len());
}
