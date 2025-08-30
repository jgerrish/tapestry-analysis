//! Example of drawing a shift register diagram or feedback shift
//! register diagram
//!
//! To run: cargo run --example shift_register_diagram
//!
use checksum_tapestry::crc::{BitWidth, CRCConfiguration, CRCEndianness};
use tapestry_analysis::visualization::shift_register_diagram::{
    CRCDiagram, Endianness, SimpleCRCDiagram,
};

/// Example from Implementing CRCs
/// x^4 + x^3 + 1
/// normal: 1001, reversed: 1001
/// form from Implementing CRCs: 11001
///   a 1-bit for every non-zero term, starting with the highest order
///
/// To get the equivalent shift register, we first turn the number
/// around, then shift it right one place to get 1001.
/// This number is the feedback factor that drives the generator.
/// Place a tap at the entrance to every bit that has a one in its
/// feedback factor.
///
///   x^0 ----- x^1 ----- x^2 -----   ->X x^3 ----- x^4
///  +--->| 3 |---->| 2 |---->| 1 |--/  O---->| 0 |-->--
///  |    -----     -----     -----   ->R     -----    |
///  |                                |                |
///  |                                |                |
///  +----------------<---------------+---------<------+
fn main() {
    let config = CRCConfiguration::<u16>::new(
        "CRC-4/CRENSHAW",
        BitWidth::Four,
        CRCEndianness::MSB,
        0b1001,
        false,
        None,
        None,
    );

    let diagram: SimpleCRCDiagram<u16> = SimpleCRCDiagram::new(config, Endianness::MSB, false);

    println!("{}", diagram);
}
