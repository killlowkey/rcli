use rand::seq::SliceRandom;
use anyhow::Result;

// rust 声明常量，必须显示声明常量类型
// &[u8] 等价于 &'static [u8]，编译器会自动推导
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String>  {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    // 要保证生成的密码，每个类型都至少有一个
    if upper {
        chars.extend_from_slice(UPPER);
        // 这里将数据添加到 password，会进行类型推导，password 数据类型就被限定为 Vec<u8>
        password.push(*UPPER.choose(&mut rng).expect("upper won't be empty"))
    }

    if lower {
        chars.extend_from_slice(b"abcdefghijkmnpqrstuvwxyz");
        password.push(*LOWER.choose(&mut rng).expect("lower won't be empty"))
    }

    if number {
        chars.extend_from_slice(b"123456789");
        password.push(*NUMBER.choose(&mut rng).expect("number won't be empty"))
    }

    if symbol {
        chars.extend_from_slice(b"!@#$%^&*_");
        password.push(*SYMBOL.choose(&mut rng).expect("symbol won't be empty"))
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    // 重新混合 password
    password.shuffle(&mut rng);

    Ok(String::from_utf8(password)?)
}
