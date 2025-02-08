use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    encoding: Option<String>,
    as_bytes: bool,
    as_string: bool,
}

#[wasm_bindgen]
pub struct MD5State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl MD5State {
    fn new() -> MD5State {
        MD5State {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface MD5Options {
    encoding?: 'binary' | 'utf8';
    as_bytes?: boolean;
    as_string?: boolean;
}

export function md5(message: string | Uint8Array, options?: MD5Options): string | Uint8Array;
"#;

#[wasm_bindgen]
pub fn md5(message: &JsValue, options: JsValue) -> Result<JsValue, JsValue> {
    if message.is_null() || message.is_undefined() {
        return Err(JsValue::from_str(&format!("Illegal argument {:?}", message)));
    }

    let options = if options.is_undefined() || options.is_null() {
        JsValue::from(js_sys::Object::new())
    } else {
        options
    };

    let options: Option<Options> = from_value(options.clone()).unwrap_or(None);

    // 转换输入为字节数组
    let bytes = if message.is_string() {
        let s = message.as_string().unwrap();
        if let Some(opts) = &options {
            if opts.encoding == Some("binary".to_string()) {
                s.bytes().collect::<Vec<u8>>()
            } else {
                s.bytes().collect::<Vec<u8>>()
            }
        } else {
            s.bytes().collect::<Vec<u8>>()
        }
    } else if message.is_instance_of::<Uint8Array>() {
        let array = Uint8Array::from(message.clone());
        array.to_vec()
    } else {
        message
            .as_string()
            .unwrap_or_default()
            .bytes()
            .collect::<Vec<u8>>()
    };

    let digest = handle_md5(&bytes);

    // 根据选项返回不同格式
    if let Some(opts) = options {
        if opts.as_bytes {
            Ok(JsValue::from(Uint8Array::from(&digest[..])))
        } else if opts.as_string {
            Ok(JsValue::from_str(&String::from_utf8_lossy(&digest)))
        } else {
            Ok(JsValue::from_str(&hex_encode(&digest)))
        }
    } else {
        Ok(JsValue::from_str(&hex_encode(&digest)))
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// 核心MD5计算函数
fn handle_md5(input: &[u8]) -> Vec<u8> {
    let mut state = MD5State::new();
    let mut msg = input.to_vec();

    // 添加填充
    let initial_len = msg.len();
    let initial_bits = initial_len * 8;

    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }

    msg.extend_from_slice(&(initial_bits as u64).to_le_bytes());

    for chunk in msg.chunks(64) {
        process_block(&mut state, chunk);
    }

    let mut result = Vec::with_capacity(16);
    result.extend_from_slice(&state.a.to_le_bytes());
    result.extend_from_slice(&state.b.to_le_bytes());
    result.extend_from_slice(&state.c.to_le_bytes());
    result.extend_from_slice(&state.d.to_le_bytes());
    result
}

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

fn process_block(state: &mut MD5State, block: &[u8]) {
    let mut a = state.a;
    let mut b = state.b;
    let mut c = state.c;
    let mut d = state.d;

    // 将块转换为32位字数组
    let mut x = [0u32; 16];
    for i in 0..16 {
        let j = i * 4;
        x[i] = u32::from_le_bytes([block[j], block[j + 1], block[j + 2], block[j + 3]]);
    }

    // 主循环
    for i in 0..64 {
        let (mut f, g) = match i {
            0..=15 => ((b & c) | (!b & d), i),
            16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),
            32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
            _ => (c ^ (b | !d), (7 * i) % 16),
        };

        f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(x[g]);

        a = d;
        d = c;
        c = b;
        b = b.wrapping_add(f.rotate_left(S[i]));
    }

    state.a = state.a.wrapping_add(a);
    state.b = state.b.wrapping_add(b);
    state.c = state.c.wrapping_add(c);
    state.d = state.d.wrapping_add(d);
}

#[cfg(test)]
mod tests;

