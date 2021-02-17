pub fn decompress(input: &[u8], output: &mut [u8]) {
    unsafe {
        let mut output = output.as_mut_ptr();
        let core::ops::Range { start: mut input, end } = input.as_ptr_range();
        let mut match_offset = 0;
        let mut token = 0;
        let mut len;
        let mut i = 0;
        'main: loop {
            i += 1;
            if i&1 == 1 {
                // Stage 1: read token and use top bits of the token as length
                token = read_u8(&mut input);
                len = (token >> 4) as usize;
            } else {
                // Stage 2: read match offset and use bottom bits
                match_offset = read_u16(&mut input);
                len = (token & 0xF) as usize;
            }
            // Decode LSIC literal length
            if len == 15 {
                len += read_int(&mut input);
            }
            let src;
            if i&1 == 1 {
                // Stage 1: Copy literal section
                src = input;
                input = input.add(len);
            } else {
                // Stage 2: Copy duplicate section
                len += 4;
                src = output.sub(match_offset);
            }
            crate::copy_fwd(src, output, len);
            // Advance output buffer
            output = output.add(len);
            // If we're done, exit
            if input == end { break 'main; }
        }
    }
}
unsafe fn read_u8(input: &mut *const u8) -> u8 {
    let c = **input;
    *input = input.add(1);
    c
}
unsafe fn read_u16(input: &mut *const u8) -> usize {
    let c = *(*input as *const u16) as _;
    *input = input.add(2);
    c
}

unsafe fn read_int(input: &mut *const u8) -> usize {
    let mut sum = 0;
    loop {
        let c = read_u8(input);
        sum += c as usize;
        if c != 255 { break; }
    }
    sum
}
