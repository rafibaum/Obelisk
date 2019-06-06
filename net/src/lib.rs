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

/*macro_rules! variable_int_codec {
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
}*/

impl MCVariablePacketCodec for i32 {
    fn to_mc_variable_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut num = *self;

        loop {
            let mut value = num & 0b01111111;
            num >>= 7;
            if num == 0 {
                result.push(value as u8);
                break;
            } else {
                value |= 0b10000000;
                result.push(value as u8);
            }
        }

        result
    }

    fn from_mc_variable_bytes(bytes: &mut Vec<u8>) -> Result<Self, Error> {
        let mut result: i32 = 0;
        let mut bytes_read = 0;

        for byte in bytes.iter() {
            let value = (*byte & 0b01111111) as i32;
            result |= value << (7 * bytes_read);
            bytes_read += 1;

            if bytes_read > 5 {
                return Err(Error::new(ErrorKind::InvalidData, "VarInt was too long"));
            }

            if *byte & 0b10000000 == 0 {
                break;
            }
        }

        bytes.drain(..bytes_read);

        Ok(result)
    }
}

//variable_int_codec!(i32, 5);
//variable_int_codec!(i64, 10);


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
    use rand::Rng;
    use rand::distributions::Alphanumeric;

    macro_rules! test_encoding {
        ($Type:ident, $value:expr) => {
            let value = $value;
            let mut encoded = value.to_mc_bytes();
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
        test_encoding!(bool, true);
        test_encoding!(bool, false);
    }

    #[test]
    fn bytes() {
        test_encoding!(u8, rand::random::<u8>());
        test_encoding!(i8, rand::random::<i8>());
    }

    #[test]
    fn ints() {
        test_encoding!(u16, rand::random::<u16>());
        test_encoding!(i16, rand::random::<i16>());
        test_encoding!(u32, rand::random::<u32>());
        test_encoding!(i32, rand::random::<i32>());
        test_encoding!(u64, rand::random::<u64>());
        test_encoding!(i64, rand::random::<i64>());

        // Verify endianness
        let encoded = (0x38D4 as i16).to_mc_bytes();
        assert_eq!(encoded[0], 0x38 as u8);
    }

    #[test]
    fn floats() {
        test_encoding!(f32, rand::random::<f32>());
        test_encoding!(f64, rand::random::<f64>());
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
        test_encoding!(String, rand_string);
    }

}