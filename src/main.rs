// General idea around the polynomial is that it is not divisible by x or x+1
// this can be done with 2 simle checks:
// 1. the polynomial(binary) may not end (LSB) with 0
// 2. the polynomial(binary) may not have an even amount of 1s
const CRC_32_POLYNOMIAL: u32 = 0x04C11DB7; // polynomial used by tcp, zip, and others
                                           // 0000010011000001000111011011011
                                           // x32 + x26 + x23 + x22 + x16 + x12 + x11 + x10 + x8 + x7 + x5 + x4 + x2 + x + 1
                                           // x(f), where f is the nth bit in the binary representation, is present if the bit is set
                                           // x(n), where n is length of the crc, is always present because it is actually the 33rd bit in this case which is always 1 (implied)
                                           //
                                           // lets assume x = 2 then calculating the polynonmial results in: 359
                                           // 359 can NOT be divided by 2 or 3(x+1) so its good to use

fn calculate_crc32(data: &[u8]) -> u32 {
    // initialize crc to 1s
    let mut crc = u32::MAX;

    // for each byte in data
    for b in data {
        println!("-----------------------------");
        println!("crc: {:032b}\tdata:{:08b}", crc, b);
        let mut ocrc = crc;
        // Byte is right-padded with 0s to match crc length
        // right pad with (n - len) zeroes where n is the CRC size (32) and len is the length the byte (8 bits in a byte)
        let rpadded = (*b as u32) << 24;

        // Initially XOR the CRC with the current byte
        crc ^= rpadded;
        println!("^    {:032b}\t={:032b} ^ {:032b}", crc, ocrc, rpadded);
        // for each bit in the byte
        for _ in 0..8 {
            // check if the MSB is 1 using the AND operator
            if crc & 0b10000000000000000000000000000000 != 0 {
                // when the MSB is set(1), shift crc left by one and XOR with the polynomial
                ocrc = crc;
                // shift left by one
                crc = crc << 1;
                println!("MSB1:{:032b}\t=({:032b} << 1)", crc, ocrc);
                ocrc = crc;
                // XOR with the polynomial
                crc ^= CRC_32_POLYNOMIAL;
                println!(
                    "^   :{:032b}\t=({:032b} ^ poly:{:032b})",
                    crc, ocrc, CRC_32_POLYNOMIAL
                );
            } else {
                // if MSB is not set(0), only shift left without xor
                ocrc = crc;
                // shift left by 1
                crc <<= 1;
                println!("MSB0:{:032b}\t=({:032b} << 1)", crc, ocrc);
            }
        }
    }
    let ocrc = crc;
    // Invert the final CRC (since we initialized with 1s)
    crc ^= u32::MAX;
    println!("-----------------------------");
    println!("fin :{:032b}\t=({:032b} ^ {:032b})", crc, ocrc, u32::MAX);
    crc
}

fn main() {
    let data = b"helo worl";
    let crc = calculate_crc32(data);
    println!();
    println!("polynomial: {:032b}", CRC_32_POLYNOMIAL);
    println!("CRC32:      {:032b}", crc);
}
