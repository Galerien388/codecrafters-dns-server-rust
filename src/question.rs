use crate::field::Field;

#[derive(Debug)]
pub struct Question {
    field: Field,
}

impl Question {
    pub fn new(name: String, f_type: u16, f_class: u16) -> Self {
        Question {
            field: Field::new(name, f_type, f_class),
        }
    }

    pub fn into_slice(&self, buf: &mut [u8]) -> usize {
        self.field.into_slice(buf)
    }

    pub fn from_slice(buf: &[u8]) -> (Question, usize) {
        let (field, len) = Field::from_slice(buf);
        let question = Question { field };
        (question, len)
    }
}
