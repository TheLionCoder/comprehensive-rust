/// A wire type as seen on the wire
enum WireType {
    /// The Varint WireType indicates the value is a single VARINT.
    Varint,
    // The I64 WireType indicates that the value is precisely 8 bytes in
    // little-endian order containing a 64-bit signed integer or double type.
    //I-64, -- not needed for this exercise
    /// The len WireType indicates that the value is a length represented as a
    /// VARINT followed by exactly that number of bytes.
    Len,
    // The I32 WireType indicates that the value is precisely 4 bytes in
    // little endian order containing a 32-bit signed integer or float type.
    //I-32, -- not needed for this exercise
}

#[derive(Debug)]
/// A field's value, typed based on the wiretype
enum FieldValue<'a> {
    Varint(u64),
    // I64 not needed in this exercise
    Len(&'a [u8]),
}

#[derive(Debug)]
/// A field, containing the field number and its value
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            1 => WireType::Len,
            _ => panic!("Invalid wire type: {value}"),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_str(&self) -> &'a str {
        let FieldValue::Len(data) = self else {
            panic!("Expected string to be a `len` field")
        };
        std::str::from_utf8(data).expect("Invalid string")
    }

    fn as_bytes(&self) -> &'a [u8] {
        let FieldValue::Len(data) = self else {
            panic!("Expected bytes to be a `len` field")
        };
        data
    }

    fn as_u64(&self) -> u64 {
        let FieldValue::Varint(value) = self else {
            panic!("Expected u64 to be a `varint`field")
        };
        *value
    }
}

/// Parse a VARINT, returning the parsed value and the remaining bytes
fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    for i in 0..7 {
        let Some(byte) = data.get(i) else {
            panic!("Not enough bytes for varint|")
        };
        if byte & 0x80 == 0 {
            // This is the last byte of the VARINT, so convert it to
            // a u64 and return it
            let mut value: u64 = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return (value, &data[i + 1..]);
        }
    }
    panic!("Too many bytes for varint")
}

/// Convert a tag into a field number and WireType.
fn unpack_tag(tag: u64) -> (u64, WireType) {
    let field_num: u64 = tag >> 3;
    let wire_type: WireType = WireType::from(tag & 0x7);
    (field_num, wire_type)
}

/// Parse a field, returning the remain bytes
fn parse_field(data: &[u8]) -> (Field, &[u8]) {
    let (tag, remainder) = parse_varint(data);
    let (field_num, wire_type) = unpack_tag(tag);
    let (field_value, remainder) = match wire_type {
        WireType::Varint => {
            let (value, remainder) = parse_varint(remainder);
            (FieldValue::Varint(value), remainder)
        }
        WireType::Len => {
            let (len, remainder) = parse_varint(remainder);
            let len: usize = len.try_into().expect("len not a valid `usize`");
            if remainder.len() < len {
                panic!("Unexpected EOF")
            }
            let (value, remainder) = remainder.split_at(len);
            (FieldValue::Len(value), remainder)
        }
    };
    (
        Field {
            field_num,
            value: field_value,
        },
        remainder,
    )
}

/// Parse a message in the given data, calling `T::add_field` for each field
/// in the message
///
/// The entire input is consumed
fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> T {
    let mut result: T = T::default();
    while !data.is_empty() {
        let parsed = parse_field(data);
        result.add_field(parsed.0);
        data = parsed.1;
    }
    result
}

#[derive(Debug, Default, PartialEq)]
struct PhoneNumber<'a> {
    number: &'a str,
    kind: &'a str,
}

#[derive(Debug, Default, PartialEq)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.name = field.value.as_str(),
            2 => self.id = field.value.as_u64(),
            3 => self.phone.push(parse_message(field.value.as_bytes())),
            _ => {} // Skip everything else
        }
    }
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.number = field.value.as_str(),
            2 => self.kind = field.value.as_str(),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let first_person: Person = parse_message(&[0x10, 0x2a]);
        assert_eq!(
            first_person,
            Person {
                name: "",
                id: 42,
                phone: vec![]
            }
        )
    }
}
