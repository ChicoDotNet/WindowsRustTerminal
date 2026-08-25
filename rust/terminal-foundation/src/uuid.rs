/// Portable GUID value using the same field layout as the Windows `GUID` type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }

    fn network_bytes(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[0..4].copy_from_slice(&self.data1.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.data2.to_be_bytes());
        bytes[6..8].copy_from_slice(&self.data3.to_be_bytes());
        bytes[8..16].copy_from_slice(&self.data4);
        bytes
    }

    fn from_network_bytes(bytes: [u8; 16]) -> Self {
        Self {
            data1: u32::from_be_bytes(bytes[0..4].try_into().expect("four-byte GUID field")),
            data2: u16::from_be_bytes(bytes[4..6].try_into().expect("two-byte GUID field")),
            data3: u16::from_be_bytes(bytes[6..8].try_into().expect("two-byte GUID field")),
            data4: bytes[8..16].try_into().expect("eight-byte GUID field"),
        }
    }
}

/// Creates an RFC 4122 version-5 UUID from a namespace GUID and arbitrary name bytes.
///
/// The namespace is hashed in network byte order, matching Microsoft's `CreateV5Uuid`.
pub fn create_v5_uuid(namespace: Guid, name: &[u8]) -> Guid {
    let mut input = Vec::with_capacity(16 + name.len());
    input.extend_from_slice(&namespace.network_bytes());
    input.extend_from_slice(name);

    let digest = sha1(&input);
    let mut bytes: [u8; 16] = digest[..16]
        .try_into()
        .expect("SHA-1 prefix is sixteen bytes");
    bytes[6] = (bytes[6] & 0x0f) | 0x50;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Guid::from_network_bytes(bytes)
}

fn sha1(input: &[u8]) -> [u8; 20] {
    let bit_len = u64::try_from(input.len())
        .expect("input length must fit in u64")
        .checked_mul(8)
        .expect("input bit length must fit in u64");
    let mut message = input.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&bit_len.to_be_bytes());

    let mut h0 = 0x6745_2301u32;
    let mut h1 = 0xefcd_ab89u32;
    let mut h2 = 0x98ba_dcfeu32;
    let mut h3 = 0x1032_5476u32;
    let mut h4 = 0xc3d2_e1f0u32;

    for chunk in message.chunks_exact(64) {
        let mut words = [0u32; 80];
        for (index, word) in words[..16].iter_mut().enumerate() {
            let start = index * 4;
            *word = u32::from_be_bytes(chunk[start..start + 4].try_into().expect("SHA-1 word"));
        }
        for index in 16..80 {
            words[index] =
                (words[index - 3] ^ words[index - 8] ^ words[index - 14] ^ words[index - 16])
                    .rotate_left(1);
        }

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for (index, word) in words.into_iter().enumerate() {
            let (f, k) = match index {
                0..=19 => ((b & c) | ((!b) & d), 0x5a82_7999),
                20..=39 => (b ^ c ^ d, 0x6ed9_eba1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1b_bcdc),
                _ => (b ^ c ^ d, 0xca62_c1d6),
            };
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    let mut digest = [0u8; 20];
    for (chunk, word) in digest.chunks_exact_mut(4).zip([h0, h1, h2, h3, h4]) {
        chunk.copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::{Guid, create_v5_uuid};

    const TEST_NAMESPACE_GUID: Guid = Guid::new(
        0xad56_de9e,
        0x5167,
        0x41b6,
        [0x80, 0xeb, 0xfb, 0x19, 0xf7, 0x92, 0x7d, 0x1a],
    );

    #[test]
    fn microsoft_types_v5_uuid_u8_string_matches_source_contract() {
        let expected = Guid::new(
            0x8b9d_4336,
            0x0c82,
            0x54c4,
            [0xb3, 0x15, 0xf1, 0xd2, 0xd2, 0x7e, 0xc6, 0xda],
        );
        assert_eq!(expected, create_v5_uuid(TEST_NAMESPACE_GUID, b"testing"));
    }

    #[test]
    fn microsoft_types_v5_uuid_u16_string_matches_source_contract() {
        let name: Vec<u8> = "testing"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        let expected = Guid::new(
            0xe04f_b1f7,
            0x739d,
            0x5d63,
            [0xbb, 0x18, 0xe0, 0xea, 0x00, 0xb1, 0x9e, 0xe8],
        );
        assert_eq!(expected, create_v5_uuid(TEST_NAMESPACE_GUID, &name));
    }
}
