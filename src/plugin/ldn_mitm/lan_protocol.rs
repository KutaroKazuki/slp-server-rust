// int LanSocket::decompress(const void *input, size_t input_size, uint8_t *output, size_t *output_size) {
//     const uint8_t *in = (decltype(in))input;
//     const uint8_t *in_end = in + input_size;
//     uint8_t *out = output;
//     uint8_t *out_end = output + *output_size;

//     while (in < in_end && out < out_end) {
//         uint8_t c = *in++;

//         *out++ = c;
//         if (c == 0) {
//             if (in == in_end) {
//                 return -1;
//             }
//             uint8_t count = *in++;
//             for (int i = 0; i < count; i++) {
//                 if (out == out_end) {
//                     break;
//                 }
//                 *out++ = c;
//             }
//         }
//     }

//     *output_size = out - output;

//     return in == in_end ? 0 : -1;
// }

struct DecompressState<'a> {
    lead_zero: bool,
    pos: usize,
    output: &'a mut [u8],

}
fn decompress(input: &[u8], output: &mut [u8]) -> Option<usize> {
    // let mut lead_zero = false;
    let state = DecompressState {
        lead_zero: false,
        pos: 0,
        output,
    };
    let pos = input.iter().scan(state, |state, i| {
        let pos = state.pos;
        if state.lead_zero {
            let size = *i as usize;
            state.output[pos..pos + size].swap_with_slice(&mut vec![0u8 ; size]);
        } else {
            state.output[pos] = *i;
            if *i == 0 {
                state.lead_zero = true;
            }
        }
        Some(state.pos)
    }).last();

    pos
}

