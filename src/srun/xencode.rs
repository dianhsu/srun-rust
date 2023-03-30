fn sencode(msg: &str, key: bool) -> Vec<usize> {
    let mut pwd = vec![];
    let msg = msg.as_bytes();
    for i in (0..msg.len()).step_by(4) {
        pwd.push(
            msg[i] as usize
                | (msg[i + 1] as usize) << 8
                | (msg[i + 2] as usize) << 16
                | (msg[i + 3] as usize) << 24,
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
