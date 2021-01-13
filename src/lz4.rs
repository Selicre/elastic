pub fn decompress(mut input: &[u8], output: &mut [u8]) {
    let mut offset = 0;
    while input.len() != 0 {
        // Read token
        let token = read_u8(&mut input);
        // Decode LSIC literal length
        let mut len = (token >> 4) as usize;
        if len == 15 {
            len += read_int(&mut input);
        }
        // Copy len to output and advance buffers
        output[offset..offset+len].copy_from_slice(&input[..len]);
        input = &input[len..];
        offset += len;
        if input.len() == 0 { break; }
        // Decode duplicates
        let match_offset = read_u16(&mut input);
        let mut len = (4 + (token & 0xF)) as usize;
        if len == 4 + 15 {
            len += read_int(&mut input);
        }
        // Copy duplicate section
        let start = offset.wrapping_sub(match_offset as usize);
        for i in 0..len {
            output[offset+i] = output[start+i];
        }
        offset += len;
    }
}

fn read_u8(input: &mut &[u8]) -> u8 {
    let c = input[0];
    *input = &(*input)[1..];
    c
}
fn read_u16(input: &mut &[u8]) -> u16 {
    u16::from_le_bytes([read_u8(input), read_u8(input)])
}

fn read_int(input: &mut &[u8]) -> usize {
    let mut sum = 0;
    loop {
        let c = read_u8(input);
        sum += c as usize;
        if c != 255 { break; }
    }
    sum
}
