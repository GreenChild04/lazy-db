use std::os::unix::prelude::OsStrExt;

use super::*;

macro_rules! new_number {
    (($name:ident) $type:ty = $lazy_type:expr) => {
        /// Creates a new `LazyData` file with an unsigned integer and type
        pub fn $name(mut file: FileWrapper, value: $type) -> Result<(), LDBError> {
            let bytes = value.to_be_bytes();
            file.write(&[$lazy_type.into()])?;
            file.write(&bytes)?;
            Ok(())
        }
    };

    (signed ($name:ident) $type:ty = $lazy_type:expr) => {
        /// Creates a new `LazyData` file with a signed integer and type
        pub fn $name(mut file: FileWrapper, value: $type) -> Result<(), LDBError> {
            let bytes = value.to_be_bytes();
            file.write(&[$lazy_type.into()])?;
            file.write(&bytes)?;
            Ok(())
        }
    };
}

macro_rules! new_array {
    (($name:ident) $type:ty = $lazy_type:ident) => {
        /// Creates a new `LazyData` file with an array type and value
        pub fn $name(mut file: FileWrapper, value: &[$type]) -> Result<(), LDBError> {
            file.write(&[LazyType::Array.into(), LazyType::$lazy_type.into()])?;
            for i in value {
                let bytes = i.to_be_bytes();
                file.write(&bytes)?;
            }
            Ok(())
        }
    }
}

impl LazyData {
    /// Creates a new `LazyData` file with the type of `LazyType::Void`
    pub fn new_void(mut file: FileWrapper, _value: ()) -> Result<(), LDBError> {
        file.write(&[LazyType::Void.into()])?;
        Ok(())
    }

    /// Creates a new `LazyData` file with a `String` value and type
    pub fn new_string(mut file: FileWrapper, value: &str) -> Result<(), LDBError> {
        let bytes = value.as_bytes();
        file.write(&[LazyType::String.into()])?;
        file.write(bytes)?;
        Ok(())
    }

    // Signed Integers
    new_number!(signed (new_i8) i8 = LazyType::I8);
    new_number!(signed (new_i16) i16 = LazyType::I16);
    new_number!(signed (new_i32) i32 = LazyType::I32);
    new_number!(signed (new_i64) i64 = LazyType::I64);
    new_number!(signed (new_i128) i128 = LazyType::I128);

    // Unsigned Integers
    new_number!((new_u8) u8 = LazyType::U8);
    new_number!((new_u16) u16 = LazyType::U16);
    new_number!((new_u32) u32 = LazyType::U32);
    new_number!((new_u64) u64 = LazyType::U64);
    new_number!((new_u128) u128 = LazyType::U128);

    // Arrays
    new_array!((new_u8_array) u8 = U8);
    new_array!((new_u16_array) u16 = U16);
    new_array!((new_u32_array) u32 = U32);
    new_array!((new_u64_array) u64 = U64);
    new_array!((new_u128_array) u128 = U128);
    new_array!((new_i8_array) i8 = I8);
    new_array!((new_i16_array) i16 = I16);
    new_array!((new_i32_array) i32 = I32);
    new_array!((new_i64_array) i64 = I64);
    new_array!((new_i128_array) i128 = I128);
    new_array!((new_f32_array) f32 = F32);
    new_array!((new_f64_array) f64 = F64);

    /* Floating point numbers */

    /// Creates a new `LazyData` file with an `f32` value and type
    pub fn new_f32(mut file: FileWrapper, value: f32) -> Result<(), LDBError> {
        let bytes = value.to_be_bytes();
        file.write(&[LazyType::F32.into()])?;
        file.write(&bytes)?;
        Ok(())
    }

    /// Creates a new `LazyData` file with an `f64` value and type
    pub fn new_f64(mut file: FileWrapper, value: f64) -> Result<(), LDBError> {
        let bytes = value.to_be_bytes();
        file.write(&[LazyType::F64.into()])?;
        file.write(&bytes)?;
        Ok(())
    }

    /// Creates a new `LazyData` file with a `binary` value and type
    pub fn new_binary(mut file: FileWrapper, value: &[u8]) -> Result<(), LDBError> {
        file.write(&[LazyType::Binary.into()])?;
        file.write(value)
    }

    /// Creates a new `LazyData` file with a `bool` value and type
    pub fn new_bool(mut file: FileWrapper, value: bool) -> Result<(), LDBError> {
        if value {
            file.write(&[LazyType::True.into()])
        } else {
            file.write(&[LazyType::False.into()])
        }
    }

    /// Creates a new `LazyData` file with a link (it's like a reference) value and type
    pub fn new_link(mut file: FileWrapper, data: impl AsRef<Path>) -> Result<(), LDBError> {
        file.write(&[LazyType::Link.into()])?;
        file.write(data.as_ref().as_os_str().as_bytes())
    }
}