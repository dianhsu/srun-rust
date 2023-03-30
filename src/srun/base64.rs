// const PADCHAR: u8 = b'=';
// const ALPHA: &[u8] = "LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA".as_bytes();

// pub fn get_base64(value: &str) -> String {
//     if value.is_empty() {
//         return value.to_string();
//     }
//     let value = value.as_bytes();
//     let imax = value.len() - value.len() % 3;
//     let mut bytes = vec![];
//     for i in (0..imax).step_by(3) {
//         let b10 = ((value[i] as usize) << 16) | ((value[i+1] as usize)  << 8) | (value[i+2] as usize);
//         bytes.push(ALPHA[b10>>18]);
//         bytes.push(ALPHA[(b10>>12)&63]);
//         bytes.push(ALPHA[(b10>>6)&63]);
//         bytes.push(ALPHA[(b10>>0)&63]);
//     }
//     if value.len() - imax == 1 {
//         let b10 = (value[imax] as usize) << 16;
//         bytes.push(ALPHA[b10>>18]);
//         bytes.push(ALPHA[(b10>>12)&63]);
//         bytes.push(PADCHAR);
//         bytes.push(PADCHAR);

//     } else if value.len() - imax == 2 {
//         let b10 = ((value[imax] as usize) << 16) | ((value[imax+1] as usize)  << 8) ;
//         bytes.push(ALPHA[b10>>18]);
//         bytes.push(ALPHA[(b10>>12)&63]);
//         bytes.push(ALPHA[(b10>>6)&63]);
//         bytes.push(PADCHAR);
//     }
//     String::from_utf8(bytes).unwrap()
// }

// 用库也一样
use base64::{alphabet, engine, Engine as _};

pub fn get_base64(value: &str) -> String {
    let alphabet =
        alphabet::Alphabet::new("LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA")
            .unwrap();
    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(true)
        .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);

    let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

    crazy_engine.encode(value)
}
