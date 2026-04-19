use crate::field::Field;

#[derive(Debug)]
pub struct Answer {
    pub field: Field,
    pub ttl: u32,
    pub length: u16,
    pub data: u32,
}

impl Answer {
    pub fn new(name: String, f_type: u16, f_class: u16, ttl: u32, length: u16, ip: String) -> Self {
        Self {
            field: Field {
                name,
                f_type,
                f_class,
            },
            ttl,
            data: Self::ip_to_u32(ip),
            length,
        }
    }

    fn ip_to_u32(ip: String) -> u32 {
        let mut octets = [0u8; 4];
        for (i, part) in ip.split('.').enumerate() {
            octets[i] = part.parse::<u8>().expect("invalid IP");
        }
        u32::from_be_bytes(octets)
    }

    pub fn from_slice(buf: &[u8]) -> (Self, usize) {
        let (field, mut start) = Field::from_slice(buf);
        let ttl = u32::from_be_bytes([buf[start], buf[start + 1], buf[start + 2], buf[start + 3]]);
        start += 4;
        let length = u16::from_be_bytes([buf[start], buf[start + 1]]);
        start += 2;
        let data = if field.f_type == 1 && length == 4 {
            let d =
                u32::from_be_bytes([buf[start], buf[start + 1], buf[start + 2], buf[start + 3]]);
            start += 4;
            d
        } else {
            // placeholder for now
            0
        };

        (
            Self {
                field,
                ttl,
                length,
                data,
            },
            start,
        )
    }

    pub fn into_slice(&self, buf: &mut [u8]) -> usize {
        let mut start = self.field.into_slice(buf);
        buf[start..start + 4].copy_from_slice(&self.ttl.to_be_bytes());
        start += 4;
        buf[start..start + 2].copy_from_slice(&self.length.to_be_bytes());
        start += 2;
        buf[start..start + 4].copy_from_slice(&self.data.to_be_bytes());
        start += 4;
        start
    }
}
