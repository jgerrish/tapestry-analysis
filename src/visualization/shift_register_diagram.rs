//! Show an ASCII shift-register diagram of a CRC

use std::fmt::{Display, Formatter, Result};

use checksum_tapestry::crc::{CRCConfiguration, Width};

/// CRC Diagram bit-endianness / bit ordering
///
/// Digrams can be displayed with the Most Significant Bit (MSB) first
/// or the Least Significant Bit (LSB) first
///
///
/// This is an MSB diagram, in the style of "Implementing CRCs" by
/// Jack W. Crenshaw:
///
///  x^0 ----- x^1 ----- x^2 -----   ->X x^3 ----- x^4
/// +--->| 4 |---->| 3 |---->| 2 |--/  O---->| 1 |---->+
/// |    -----     -----     -----   ->R     -----     |
/// |                                |                 |
/// |                                |                 |
/// +--------------------------------+-----------------+
///
/// This is a LSB diagram, in the style of Phil Koopman's YouTube
/// example:
///
///  x^4 -----   ->X x^3 ----- x^2 ----- x^1 ----- x^0
/// +--->| 1 |--/  O---->| 2 |---->| 3 |---->| 4 |---->+
/// |    -----   ->R     -----     -----     -----     |
/// |            |                   |                 |
/// |            |                   |                 |
/// +------------+-------------------+-----------------+
///
#[derive(Clone, Copy)]
pub enum Endianness {
    /// Most-significant bit first, big-endian
    MSB,
    /// Least-significant bit first, little-endian
    LSB,
}

/// A trait defining a CRCDiagram
///
/// The only current implementation is an ASCII diagram, but future
/// implementations might be mermaid diagrams or SVG or Postscript.
pub trait CRCDiagram<'config, BITWIDTH: Width> {
    /// Create a new diagram with a given CRCConfiguration
    ///
    /// # Arguments
    ///
    /// * `crc_configuration` - The CRCConfiguration for this diagram
    /// * `endianness` - Which order to display the diagram.
    ///   Using MSB has MSB bit on the left
    ///   Using LSB has the LSB on the left
    /// * `_include_msb_tap` - Whether to include a tap at the very beginning
    ///   As in Phil Koopman's YouTube example
    ///   Or not include a tap as in Jack Crenshaw's paper
    ///
    /// # Returns
    ///
    /// A new CRCDiagram
    fn new(
        crc_configuration: CRCConfiguration<'config, BITWIDTH>,
        endianness: Endianness,
        _include_msb_tap: bool,
    ) -> Self;

    /// The feedback factor of the CRC
    /// Feedback factor is explained in
    /// [Implementing CRCs by Jack W. Crenshaw](https://archive.org/details/Sundry-ErrorDetectionandCorrection-CrenshawImplementingCRCsOCR)
    /// There may be other resources that describe it better
    ///
    /// # Arguments
    ///
    /// * `&self` - The diagram structure
    ///
    /// # Returns
    ///
    /// The feedback factor of the CRC
    fn feedback_factor(&self) -> u32;

    /// Draws an individual register cell into an existing String array
    ///
    /// # Arguments
    ///
    /// * `&self` - The diagram structure
    /// * `bit` - The current bit index, from zero to the width of the CRC
    /// * `value` - The value of the bit in the register cell
    ///   This is a bool Option, if it is set, we can display a CRC as it goes through data
    ///   If it is not set, just use the inverted bit index.
    /// * `endianness` - The bit order of the diagram
    /// * `bw` - The bitwidth of the CRC
    /// * `reversed_gates` - A u32 bitvector that tells which bits should have taps
    /// * `diagram` - The String array containing the diagram up to this point
    ///
    /// # Returns
    ///
    /// Nothing
    // TODO: Is register cell the correct term for a CRC shift register hardware "cell" here?
    fn draw_register_cell(
        &self,
        bit: u8,
        value: Option<bool>,
        endianness: Endianness,
        bw: u8,
        reversed_gates: u32,
        diagram: &mut [String; 6],
    );

    /// Draw a CRC Diagram into a String
    ///
    /// # Arguments
    ///
    /// * `&self` - The diagram structure
    /// * `value` - The value of the data in the CRC shift register
    ///   This is a bool Option, if it is set, we can display a CRC as it goes through data
    ///   If it is not set, just display bit indexex in register cells.
    ///
    /// # Returns
    ///
    /// The String containing the CRCDiagram
    fn draw(&self, value: Option<u8>) -> String;
}

/// A simple ASCII diagram that doesn't rely on other diagram crates
/// or standards.
pub struct SimpleCRCDiagram<'config, BITWIDTH: Width> {
    /// The CRCConfiguration that will be diagrammed
    crc_configuration: CRCConfiguration<'config, BITWIDTH>,

    /// The direction to show to the diagram, with MSB at the left
    /// (MSB) and arrows pointing right, or MSB at the right and
    /// arrows pointing left
    endianness: Endianness,

    /// Whether to include a tap / XOR gate at the beginning of the
    /// shift register.
    /// This makes it clear when feeding in a bit at a time from a
    /// code word what is happening, but it is sometimes left out of
    /// diagrams.
    ///
    /// TODO: Implement this
    _include_msb_tap: bool,
}

impl<'config, BITWIDTH: Width> Display for SimpleCRCDiagram<'config, BITWIDTH>
where
    u32: From<BITWIDTH>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let diagram = self.draw(None);
        // let lines = diagram.split("\n");
        // for line in lines {
        //     write!(f, "{}", line)?;
        // }
        writeln!(f, "{}", diagram)
    }
}

impl<'config, BITWIDTH: Width> CRCDiagram<'config, BITWIDTH> for SimpleCRCDiagram<'config, BITWIDTH>
where
    u32: From<BITWIDTH>,
{
    fn new(
        crc_configuration: CRCConfiguration<'config, BITWIDTH>,
        endianness: Endianness,
        include_msb_tap: bool,
    ) -> Self {
        SimpleCRCDiagram {
            crc_configuration,
            endianness,
            _include_msb_tap: include_msb_tap,
        }
    }

    fn feedback_factor(&self) -> u32 {
        let bw = self.crc_configuration.width as u8;

        let poly: u32 = self.crc_configuration.poly.into();

        // TODO: This depends on the representation, e.g. MSB or LSB.
        // TOOD: Add tests for this
        // Reverse the bits and shift once
        let mut ff = poly.reverse_bits() >> 1;

        // Shift according to the bitwidth and return
        // println!("bitwidth: {}", bw);
        ff >>= 32 - bw - 1;
        ff
    }

    fn draw_register_cell(
        &self,
        bit: u8,
        value: Option<bool>,
        endianness: Endianness,
        bw: u8,
        reversed_gates: u32,
        diagram: &mut [String; 6],
    ) {
        let pow = 2_u32.pow(bit.into());
        let check = reversed_gates & pow;
        // println!("bit: {}, value: {:?}, 2^bit: {}, reversed_gates & 2^bit: {}", bit, value, pow, check);

        let first = match endianness {
            Endianness::MSB => bit == 0,
            Endianness::LSB => bit == bw,
        };
        let last = match endianness {
            Endianness::MSB => bit == bw,
            Endianness::LSB => bit == 0,
        };

        let width: usize = if bit == 0 {
            1
        } else {
            (<u32 as TryInto<usize>>::try_into(bit.ilog10())).unwrap() + 1_usize
        };

        let data = if let Some(data) = value {
            if data {
                1
            } else {
                0
            }
        } else {
            bw - bit
        };

        let box_width: usize = if data == 0 {
            1
        } else {
            (<u32 as TryInto<usize>>::try_into((data).ilog10())).unwrap() + 1_usize
        };

        // println!("data: {}, first: {}, width: {}, box_width: {}", data, first, width, box_width);

        if check != 0 {
            // This register should have a tap
            diagram[3].push_str("   |  ");
            diagram[4].push_str("   |  ");
            diagram[5].push_str("---+--");
        }

        if first {
            // diagram[2].push_str(&format!("|    {}", "-".repeat(box_width + 4)));
            diagram[3].push_str(&format!("|       {}", " ".repeat(box_width + width)));
            diagram[4].push_str(&format!("|       {}", " ".repeat(box_width + width)));
            diagram[5].push_str(&format!("+-------{}", "-".repeat(box_width + width)));
        } else if last {
            diagram[3].push_str(&format!("   {}", " ".repeat(box_width + width)));
            diagram[4].push_str(&format!("   {}", " ".repeat(box_width + width)));
            diagram[5].push_str(&format!("---{}", "-".repeat(box_width + width)));
        } else {
            diagram[3].push_str(&format!("        {}", " ".repeat(box_width + width)));
            diagram[4].push_str(&format!("        {}", " ".repeat(box_width + width)));
            diagram[5].push_str(&format!("--------{}", "-".repeat(box_width + width)));
        }

        if check != 0 {
            // There is a gate / tap for this field
            diagram[0].push_str(&format!("   ->X x^{} ", bit));
            diagram[1].push_str(&format!("--/  O-{}-->", "-".repeat(width)));
            diagram[2].push_str(&format!("   ->R  {}  ", " ".repeat(width)));

            diagram[0].push_str(&"-".repeat(box_width + 4).to_string());
            diagram[1].push_str(&format!("| {} |", data));
            diagram[2].push_str(&"-".repeat(box_width + 4).to_string());
        } else {
            if last {
                diagram[0].push_str(&format!(" x^{}", bit));
                diagram[1].push_str(&format!("--{}->", "-".repeat(width)));
            } else {
                diagram[0].push_str(&format!(" x^{} {}", bit, "-".repeat(box_width + 4)));
                if first {
                    diagram[1].push_str(&format!("+-{}->| {} |", "-".repeat(width), data));
                } else {
                    diagram[1].push_str(&format!("--{}->| {} |", "-".repeat(width), data));
                }
            }
            if first {
                diagram[2].push_str(&format!(
                    "| {}  {}",
                    " ".repeat(width),
                    "-".repeat(box_width + 4)
                ));
            } else if last {
                diagram[2].push_str(&format!("    {}", " ".repeat(width)));
            } else {
                diagram[2].push_str(&format!(
                    "  {}  {}",
                    " ".repeat(width),
                    "-".repeat(box_width + 4)
                ));
            }
        }
    }

    // TODO: This function includes a lot of logic that should be
    // separated into helper functions.  Things like generating the
    // gate bitvector and reversed_gates bit vector
    fn draw(&self, value: Option<u8>) -> String {
        let mut diagram: [String; 6] = [
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ];

        let config = self.crc_configuration;
        // Maximum bitwidth of 256
        // 256 should be enough for anybody in our post-quantum world* **
        // * Supposedly Bill Gates didn't say this.
        // ** 256 probably won't be enough, but for the CRCs I'm
        // working with to learn it's enough.
        let bw = config.width as u8;
        let ff = self.feedback_factor();

        // println!("feedback factor: {:#08X}, {:#032b}", ff, ff);

        // TODO: Make sure the mask is right,
        // We want to knock out the highest-order bit in the feedback factor
        // The MSB might be considered a "tap", but it's not visually represented
        // as an XOR tap
        let gates = ff & ((2_u32.pow((bw).into()) - 1) >> 1);
        // println!("gates: {:#032b}", gates);

        let reversed_gates = gates.reverse_bits() >> (32 - bw);

        // TODO: Make sure the order is right
        // TODO: Make sure there's not an off-by-one (I think there is)
        // println!("reversed_gates: {:#032b}", reversed_gates);
        // println!("updated gates: {:#032b}", gates);

        // This isn't optimized, but Rev<RangeInclusive<_>> is a different type
        // from RangeInclusive<_>
        // AND simple ranges specified with syntax bw..=0 build empty ranges
        let diagram_range = match self.endianness {
            Endianness::MSB => (0..=bw).collect::<Vec<u8>>(),
            Endianness::LSB => (0..=bw).rev().collect::<Vec<u8>>(),
        };

        for i in diagram_range.iter() {
            if let Some(data) = value {
                let cell_value: bool = ((data as u32) & 2_u32.pow((*i).into())) != 0;
                self.draw_register_cell(
                    *i,
                    Some(cell_value),
                    self.endianness,
                    bw,
                    reversed_gates,
                    &mut diagram,
                );
            } else {
                self.draw_register_cell(
                    *i,
                    None,
                    self.endianness,
                    bw,
                    reversed_gates,
                    &mut diagram,
                );
            }
        }
        diagram[1].push('+');
        diagram[2].push('|');
        diagram[3].push('|');
        diagram[4].push('|');
        diagram[5].push('+');

        diagram.join("\n")
    }
}

/// Tests for shift register diagram visualizations
///
/// TODO: Each test should compare the generated string with what we expect
/// the diagram to look like
/// TODO: We should also test the other trait functions
mod tests {
    #[allow(unused_imports)]
    use super::{CRCDiagram, Endianness, SimpleCRCDiagram};
    #[allow(unused_imports)]
    use checksum_tapestry::{
        crc::{BitWidth, CRCConfiguration, CRCEndianness, CRC},
        Checksum,
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
    ///
    /// TODO: Review this diagram in Crenshaw, review the diagrams on Wikipedia
    #[test]
    fn crc_4_crenshaw_diagram_works() {
        // This will also be a start to tests of bitwidths other than
        // 16 and 32
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
    ///  x^4 -----   ->X x^3 ----- x^2 ----- x^1 ----- x^0
    /// +--->| 1 |--/  O---->| 2 |---->| 3 |---->| 4 |---->+
    /// |    -----   ->R     -----     -----     -----     |
    /// |            |                   |                 |
    /// |            |                   |                 |
    /// +------------+-------------------+-----------------+
    #[test]
    fn crc_4_crenshaw_lsb_diagram_works() {
        // This will also be a start to tests of bitwidths other than
        // 16 and 32
        let config = CRCConfiguration::<u16>::new(
            "CRC-4/CRENSHAW",
            BitWidth::Four,
            CRCEndianness::MSB,
            0b1001,
            false,
            None,
            None,
        );

        let diagram: SimpleCRCDiagram<u16> = SimpleCRCDiagram::new(config, Endianness::LSB, false);

        println!("{}", diagram);
    }

    /// Crenshaw 4-bit CRC with data in the register
    /// The Crenshaw CRC generates the following pseudorandom sequence:
    /// 1,9,13,15,14,7,10,5,11,12,6,3,8,4,2
    ///
    /// Here we show it with 11 in the registers
    #[test]
    fn crc_4_crenshaw_diagram_with_data_works() {
        // This will also be a start to tests of bitwidths other than
        // 16 and 32
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

        let ds = diagram.draw(Some(11));

        println!("{}", ds);

        // Now let's experiment with the CRC
        let mut crc = CRC::<u16>::new(
            CRCConfiguration::<u16>::new(
                "CRC-4/CRENSHAW",
                BitWidth::Four,
                CRCEndianness::LSB,
                0b1001,
                true,
                None,
                None,
            ),
            false,
        );

        let mut result: u16 = 1;
        for i in 0..15 {
            print!("{}", result);
            if i >= 14 {
                println!();
            } else {
                print!(", ");
            }
            result = crc.compute(&[result.try_into().unwrap()]);
        }
        // assert_eq!(result, 12);
    }

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
    ///
    /// TODO: Review this diagram in Crenshaw, review the diagrams on Wikipedia
    #[test]
    fn crc_8_lrcc_diagram_works() {
        // This will also be a start to tests of bitwidths other than
        // 16 and 32
        let config = CRCConfiguration::<u16>::new(
            "CRC-8/LRCC",
            BitWidth::Eight,
            CRCEndianness::MSB,
            0b00000001,
            false,
            None,
            None,
        );

        let diagram: SimpleCRCDiagram<u16> = SimpleCRCDiagram::new(config, Endianness::MSB, false);

        println!("{}", diagram);
    }

    /// CRC-3-GSM
    /// x^3 + x + 1
    /// normal: 11, reversed: 110, reciprocal: 101, reversed reciprocal: 101
    /// form from Implementing CRCs: 1011
    #[test]
    fn crc_3_gsm_diagram_works() {
        let config = CRCConfiguration::<u16>::new(
            "CRC-3/GSM",
            BitWidth::Three,
            CRCEndianness::MSB,
            0b011,
            false,
            None,
            Some(0b111),
        );

        let diagram: SimpleCRCDiagram<u16> = SimpleCRCDiagram::new(config, Endianness::MSB, false);

        println!("{}", diagram);
    }

    // TODO: Implement this
    // CRC-4-ITU
    // x^4 + x + 1
    // normal: 11, reversed: 1100, reciprocal: 1001, reversed reciprocal: 1001
    // form from Implementing CRCs: 10011

    /// CRC-12
    /// x^12 + x^11 + x^3 + x^2 + x + 1
    /// normal: 100000001111, reversed: 111100000001,
    /// reciprocal: 111000000011, reversed reciprocal: 110000000111
    #[test]
    fn crc_12_umts_diagram_works() {
        let config = CRCConfiguration::<u16>::new(
            "CRC-12/UMTS",
            BitWidth::Twelve,
            CRCEndianness::MSB,
            0x80F,
            true,
            None,
            None,
        );
        let diagram: SimpleCRCDiagram<u16> = SimpleCRCDiagram::new(config, Endianness::MSB, false);

        println!("{}", diagram);
    }

    // Test example from Phil Koopman's YouTube tutorial:
    // [L600 CRC Computation Examples -- Polynomial Division & Hardware Shift Register](https://www.youtube.com/watch?v=1t3DacyL5HA)
    //
    // CRC-5-USB
    // x^5 + x^2 + 1
    // normal: 101, reversed: 1110, reciprocal: 1001, reversed reciprocal: 1001
    //
    // TODO: Update the checksum_tapestry crate with a new BitWidth variant: BitWidth::Five
    //
    // #[test]
    // fn crc_5_usb_diagram_works() {
    //     let config = CRCConfiguration::<u16>::new(
    //         "CRC-5/USB",
    //         BitWidth::Five,
    //         CRCEndianness::MSB,
    //         0x25,
    //         true,
    //         None,
    //         None,
    //     );
    //     let diagram: SimpleCRCDiagram<u16> = SimpleCRCDiagram::new(config, Endianness::LSB, true);

    //     println!("{}", diagram);
    // }

    // TODO: Test where the second item has a tap
    // TODO: Test where the last item has a tap
}
