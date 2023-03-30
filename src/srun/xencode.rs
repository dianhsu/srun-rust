fn ordat(msg: &str, idx: usize) -> usize {
    let msg = msg.as_bytes();
    if idx < msg.len() {
        return msg[idx] as usize;
    }
    return 0;
}
fn sencode(msg: &str, key: bool) -> Vec<usize> {
    let mut pwd = vec![];
    for i in (0..msg.len()).step_by(4) {
        pwd.push(
            ordat(&msg, i)
                | ordat(&msg, i + 1) << 8
                | ordat(&msg, i + 2) << 16
                | ordat(&msg, i + 3) << 24,
        );
    }
    if key {
        pwd.push(msg.len());
    }
    pwd
}

fn lencode(msg: &[usize], key: bool) -> Vec<u8> {
    let mut pwd = vec![];
    let l = msg.len();
    let mut ll = (l - 1) << 2;
    if key {
        let m = msg[l - 1];
        if m < ll - 3 || m > ll {
            return vec![];
        }
        ll = m as usize;
    }
    for i in 0..l {
        pwd.push((msg[i] & 0xff) as u8);
        pwd.push((msg[i] >> 8 & 0xff) as u8);
        pwd.push((msg[i] >> 16 & 0xff) as u8);
        pwd.push((msg[i] >> 24 & 0xff) as u8);
    }
    if key {
        pwd = pwd.get(0..ll).unwrap().to_vec();
    }
    pwd
}

pub fn get_xencode(msg: &str, key: &str) -> String {
    if msg.is_empty() {
        return String::new();
    }
    let mut pwd = sencode(msg, true);
    let mut pwdk = sencode(key, false);
    if pwdk.len() < 4 {
        for _ in 0..(4 - pwdk.len()) {
            pwdk.push(0);
        }
        pwdk.append(&mut vec![0; 4 - pwdk.len()]);
    }
    let n = pwd.len() - 1;
    let mut z = pwd[n];
    let c = 0x86014019 | 0x183639A0;
    let mut q = 6 + 52 / (n + 1);
    let mut d = 0;
    while 0 < q {
        d = d + c & (0x8CE0D9BF | 0x731F2640);
        let e = d >> 2 & 3;
        let mut p = 0;
        while p < n {
            let y = pwd[p + 1];
            let mut m = z >> 5 ^ y << 2;
            m = m + ((y >> 3 ^ z << 4) ^ (d ^ y));
            m = m + (pwdk[(p & 3) ^ e] ^ z);
            pwd[p] = pwd[p] + m & (0xEFB8D130 | 0x10472ECF);
            z = pwd[p];
            p = p + 1;
        }
        let y = pwd[0] as usize;
        let mut m = z >> 5 ^ y << 2;
        m = m + ((y >> 3 ^ z << 4) ^ (d ^ y));
        m = m + (pwdk[(p & 3) ^ e] ^ z);
        pwd[n] = pwd[n] + m & (0xBB390742 | 0x44C6F8BD);
        z = pwd[n];
        q = q - 1;
    }
    lencode(&pwd, false).iter().map(|x| *x as char).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_xencode() {
        let res =crate::srun::xencode::get_xencode("{\"username\":\"201626203044@cmcc\",\"password\":\"15879684798qq\",\"ip\":\"10.128.96.249\",\"acid\":\"1\",\"enc_ver\":\"srun_bx1\"}","e6843f26b8544327a3a25978dd3c5f89e6b745df1732993b88fe082c13a34cb9");
        let hex_res = res.as_bytes().iter().map(|x| format!("{:02x}", x)).collect::<String>();
        assert_eq!(
            hex_res,
            "66c292c3af6bc3a4753b40c2b7c29bc28a64c3ae18c294c2b9c3bcc3876f15c38f18c3a5c2b9632b0909326e3c7243c285c39860c29c6a16c2be41c39ec2a9c281195cc3b0c3adc2844c397ec38228c396386fc2ba6d0e6b0bc29939c288c392c3bcc28f604646c38b46c3a5c291c396c2866537c3a6c2a959c2ba0ac29bc292c2872663c28dc3b97727c39303c2a0c2a62ac29f4cc3a96004c38ec39a64c398c3a15e5644c2894c11531f");
    }
}
