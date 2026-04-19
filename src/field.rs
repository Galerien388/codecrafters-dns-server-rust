use std::str;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub f_type: u16,
    pub f_class: u16,
}

impl Field {
    pub fn new(name: String, f_type: u16, f_class: u16) -> Self {
        Field {
            name,
            f_type,
            f_class,
        }
    }

    pub fn from_slice(buf: &[u8]) -> (Self, usize) {
        let mut start = 0;
        let mut names = Vec::new();

        loop {
            let len = buf[start] as usize;
            start += 1;
            if len == 0 {
                break;
            }
            let word = str::from_utf8(&buf[start..start + len]).expect("invalid word");
            names.push(word);
            start += len;
        }

        let f_type = u16::from_be_bytes([buf[start], buf[start + 1]]);
        start += 2;
        let f_class = u16::from_be_bytes([buf[start], buf[start + 1]]);
        start += 2;

        (
            Field {
                name: names.join("."),
                f_type,
                f_class,
            },
            start,
        )
    }

    pub fn into_slice(&self, buf: &mut [u8]) -> usize {
        let names = self.name.split('.');
        let mut start = 0;
        for name in names {
            let len_as_u8 = name.len() as u8;
            buf[start] = len_as_u8;
            start += 1;
            for b in name.as_bytes() {
                buf[start] = *b;
                start += 1;
            }
        }
        buf[start] = b'\0';
        start += 1;
        buf[start..start + 2].copy_from_slice(&self.f_type.to_be_bytes());
        start += 2;
        buf[start..start + 2].copy_from_slice(&self.f_class.to_be_bytes());
        start += 2;
        start
    }
}
