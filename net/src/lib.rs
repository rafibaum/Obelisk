use std::io::{Error, ErrorKind};

pub trait MCPacketCodec
where
    Self: Sized,
{
    fn to_mc_bytes(&self) -> Vec<u8>;
    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error>;
}

pub trait MCVariablePacketCodec
where Self: Sized {
    fn to_mc_variable_bytes(&self) -> Vec<u8>;
    fn from_mc_variable_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error>;
}

macro_rules! min_length {
    ($buffer:ident, $size:expr) => {
        if $buffer.len() < $size {
            return Err(Error::new(
            ErrorKind::InvalidData,
            "Byte buffer too short to parse data"
            ));
        }
    };
}

impl MCPacketCodec for u8 {
    fn to_mc_bytes(&self) -> Vec<u8> {
        vec![*self]
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        min_length!(bytes, 1);

        Ok(bytes.drain(0..1).next().unwrap())
    }
}

impl MCPacketCodec for i8 {
    fn to_mc_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        min_length!(bytes, 1);

        Ok(bytes.drain(0..1).next().unwrap() as i8)
    }
}

macro_rules! int_base {
    ($Type:ident, $size:expr) => {
        impl MCPacketCodec for $Type {
            fn to_mc_bytes(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }

            fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
                min_length!(bytes, $size);

                let mut num_bytes = [0u8; $size];
                let mut index = 0;
                for byte in bytes.drain(0..$size) {
                    num_bytes[index] = byte;
                    index += 1;
                }

                Ok($Type::from_be_bytes(num_bytes))
            }
        }
    };
}

macro_rules! int_codec {
    ($Type:ident) => {
        int_base!($Type, std::mem::size_of::<$Type>());
    };
}

int_codec!(u16);
int_codec!(u32);
int_codec!(u64);
int_codec!(i16);
int_codec!(i32);
int_codec!(i64);

impl MCPacketCodec for f32 {
    fn to_mc_bytes(&self) -> Vec<u8> {
        self.to_bits().to_mc_bytes()
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        Ok(f32::from_bits(u32::from_mc_bytes(bytes)?))
    }
}

impl MCPacketCodec for f64 {
    fn to_mc_bytes(&self) -> Vec<u8> {
        self.to_bits().to_mc_bytes()
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        Ok(f64::from_bits(u64::from_mc_bytes(bytes)?))
    }
}

macro_rules! variable_int_codec {
    ($Type:ident, $max_size:expr) => {
        impl MCVariablePacketCodec for $Type {
            fn to_mc_variable_bytes(&self) -> Vec<u8> {
                let mut data = Vec::new();
                let mut num = *self;

                loop {
                    let mut byte = num as u8 & 0b01111111;
                    num >>= 7;

                    if num == 0 {
                        data.push(byte);
                        break;
                    } else {
                        byte |= 0b10000000;
                        data.push(byte);
                    }
                }

                data
            }

            fn from_mc_variable_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
                let mut bytes_read: $Type = 0;
                let mut result: $Type = 0;

                for byte in bytes.into_iter() {
                    let value: $Type = (*byte & 0b01111111) as $Type;
                    result |= value << 7 * bytes_read;

                    bytes_read += 1;

                    if bytes_read > $max_size {
                        return Err(Error::new(ErrorKind::InvalidData, "VarInt is too long"));
                    }

                    if *byte & 0b10000000 == 0 {
                        break;
                    }
                }

                bytes.drain(0..bytes_read as usize);

                Ok(result)
            }
        }
    };
}

variable_int_codec!(i32, 5);
variable_int_codec!(i64, 10);


impl MCPacketCodec for bool {
    fn to_mc_bytes(&self) -> Vec<u8> {
        if *self {
            1u8.to_mc_bytes()
        } else {
            0u8.to_mc_bytes()
        }
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let num = u8::from_mc_bytes(bytes)?;
        if num == 1 {
            Ok(true)
        } else if num == 0 {
            Ok(false)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Received invalid boolean value"))
        }
    }
}

impl MCPacketCodec for String {
    fn to_mc_bytes(&self) -> Vec<u8> {
        let bytes = self.as_bytes();
        let mut length = i32::to_mc_variable_bytes(&(bytes.len() as i32));
        let mut data = Vec::new();
        data.append(&mut length);
        data.extend_from_slice(bytes);
        data
    }

    fn from_mc_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let length = i32::from_mc_variable_bytes(bytes)? as usize;
        if length > bytes.len() {
            return Err(Error::new(ErrorKind::InvalidData, "Not enough data for string"));
        }
        let str_bytes = bytes.drain(0..length).collect();
        let string = match String::from_utf8(str_bytes) {
            Ok(string) => string,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "String contains malformed data"))
        };

        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use crate::{MCPacketCodec, MCVariablePacketCodec};

    macro_rules! test_encoding {
        ($Type:ident, $value:expr, $expected:expr) => {
            let value = $value;
            let expected = $expected;
            let mut encoded = value.to_mc_bytes();
            assert_eq!(encoded, expected);
            assert_eq!($Type::from_mc_bytes(&mut encoded).unwrap(), value);
        };
    }

    macro_rules! test_variable_encoding {
        ($Type:ident, $value:expr) => {
            let value = $value;
            let mut encoded = value.to_mc_variable_bytes();
            assert_eq!($Type::from_mc_variable_bytes(&mut encoded).unwrap(), value);
        };
    }

    #[test]
    fn bools() {
        test_encoding!(bool, true, vec![1]);
        test_encoding!(bool, false, vec![0]);
    }

    #[test]
    fn bytes() {
        test_encoding!(u8, 238, vec![238]);
        test_encoding!(i8, -93, vec![163]);
    }

    #[test]
    fn ints() {
        test_encoding!(u16, 55244, vec![0xD7, 0xCC]);
        test_encoding!(i16, -22827, vec![0xA6, 0xD5]);
        test_encoding!(u32, 2221292349, vec![0x84, 0x66, 0x3b, 0x3d]);
        test_encoding!(i32, -1758047738, vec![0x97, 0x36, 0x52, 0x06]);
        test_encoding!(u64, 4889273536974385248, vec![0x43, 0xda, 0x30, 0x60, 0x9d, 0x13, 0x70, 0x60]);
        test_encoding!(i64, -3736352032367538272, vec![0xCC, 0x25, 0xCF, 0x9F, 0x62, 0xEC, 0x8F, 0xA0]);

        // Verify endianness
        let encoded = (0x38D4 as i16).to_mc_bytes();
        assert_eq!(encoded[0], 0x38 as u8);
    }

    #[test]
    fn floats() {
        test_encoding!(f32, 89013.18686, vec![0x47, 0xad, 0xda, 0x98]);
        test_encoding!(f64, 9525289013.1872418686, vec![0x42, 0x01, 0xBE, 0x03, 0x01, 0xA9, 0x7F, 0x79]);
        // Endianness guaranteed through integrity of int endianness
    }

    #[test]
    fn varints() {
        let mut encoded = 2147483647.to_mc_variable_bytes();
        assert_eq!(encoded, vec![0xff, 0xff, 0xff, 0xff, 0x07]);
        let decoded = i32::from_mc_variable_bytes(&mut encoded).unwrap();
        assert_eq!(decoded, 2147483647);

        let value = 335551513;
        let mut encoded = (value as i32).to_mc_variable_bytes();
        let decoded = i32::from_mc_variable_bytes(&mut encoded).unwrap();
        assert_eq!(decoded, value as i32);
    }

    #[test]
    fn strings() {
        let rand_string: String = String::from("DTRWFUYGIUFHINJEfiwuhfwehngviuBYU8739280r");
        //test_encoding!(String, rand_string);
    }

}