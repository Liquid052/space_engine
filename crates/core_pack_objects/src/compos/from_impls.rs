use crate::compos::PackTag;

impl From<PackTag> for bool {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::Bool(val) => val,
            _ => panic!("Attempted to convert type to bool"),
        }
    }
}

impl From<&PackTag> for bool {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::Bool(val) => *val,
            _ => panic!("Attempted to convert type to bool"),
        }
    }
}

impl From<PackTag> for u8 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::U8(val) => val,
            _ => panic!("Attempted to convert type to u8"),
        }
    }
}

impl From<&PackTag> for u8 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::U8(val) => *val,
            _ => panic!("Attempted to convert type to u8"),
        }
    }
}

impl From<PackTag> for u16 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::U16(val) => val,
            _ => panic!("Attempted to convert type to u16"),
        }
    }
}

impl From<&PackTag> for u16 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::U16(val) => *val,
            _ => panic!("Attempted to convert type to u16"),
        }
    }
}

impl From<PackTag> for u32 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::U32(val) => val,
            _ => panic!("Attempted to convert type to u32"),
        }
    }
}

impl From<&PackTag> for u32 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::U32(val) => *val,
            _ => panic!("Attempted to convert type to u32"),
        }
    }
}

impl From<PackTag> for u64 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::U64(val) => val,
            _ => panic!("Attempted to convert type to u64"),
        }
    }
}

impl From<&PackTag> for u64 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::U64(val) => *val,
            _ => panic!("Attempted to convert type to u64"),
        }
    }
}

impl From<PackTag> for i8 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::I8(val) => val,
            _ => panic!("Attempted to convert type to i8"),
        }
    }
}

impl From<&PackTag> for i8 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::I8(val) => *val,
            _ => panic!("Attempted to convert type to i8"),
        }
    }
}

impl From<PackTag> for i16 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::I16(val) => val,
            _ => panic!("Attempted to convert type to i16"),
        }
    }
}

impl From<&PackTag> for i16 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::I16(val) => *val,
            _ => panic!("Attempted to convert type to i16"),
        }
    }
}

impl From<PackTag> for i32 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::I32(val) => val,
            _ => panic!("Attempted to convert type to i32"),
        }
    }
}

impl From<&PackTag> for i32 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::I32(val) => *val,
            _ => panic!("Attempted to convert type to i32"),
        }
    }
}

impl From<PackTag> for i64 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::I64(val) => val,
            _ => panic!("Attempted to convert type to i64"),
        }
    }
}

impl From<&PackTag> for i64 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::I64(val) => *val,
            _ => panic!("Attempted to convert type to i64"),
        }
    }
}

impl From<PackTag> for f32 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::F32(val) => val,
            _ => panic!("Attempted to convert type to f32"),
        }
    }
}

impl From<&PackTag> for f32 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::F32(val) => *val,
            _ => panic!("Attempted to convert type to f32"),
        }
    }
}

impl From<PackTag> for f64 {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::F64(val) => val,
            _ => panic!("Attempted to convert type to f64"),
        }
    }
}

impl From<&PackTag> for f64 {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::F64(val) => *val,
            _ => panic!("Attempted to convert type to f64"),
        }
    }
}

impl From<PackTag> for String {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::CowStr(cow) => cow.into_owned(),
            PackTag::String(s) => s,
            PackTag::OptString(Some(s)) => s,
            PackTag::OptString(None) => String::new(),
            _ => panic!("Attempted to convert a non-string Tag variant into String"),
        }
    }
}

impl From<&PackTag> for String {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::CowStr(cow) => cow.to_string(),
            PackTag::String(s) => s.clone(),
            PackTag::OptString(Some(s)) => s.clone(),
            PackTag::OptString(None) => String::new(),
            _ => panic!("Attempted to convert a non-string Tag variant into String"),
        }
    }
}

impl From<PackTag> for Option<i32> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptI32(val) => val,
            _ => panic!("Attempted to convert type to Option<i32>"),
        }
    }
}

impl From<&PackTag> for Option<i32> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptI32(val) => *val,
            _ => panic!("Attempted to convert type to Option<i32>"),
        }
    }
}

impl From<PackTag> for Option<i64> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptI64(val) => val,
            _ => panic!("Attempted to convert type to Option<i64>"),
        }
    }
}

impl From<&PackTag> for Option<i64> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptI64(val) => *val,
            _ => panic!("Attempted to convert type to Option<i64>"),
        }
    }
}

impl From<PackTag> for Option<u32> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptU32(val) => val,
            _ => panic!("Attempted to convert type to Option<u32>"),
        }
    }
}

impl From<&PackTag> for Option<u32> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptU32(val) => *val,
            _ => panic!("Attempted to convert type to Option<u32>"),
        }
    }
}

impl From<PackTag> for Option<u64> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptU64(val) => val,
            _ => panic!("Attempted to convert type to Option<u64>"),
        }
    }
}

impl From<&PackTag> for Option<u64> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptU64(val) => *val,
            _ => panic!("Attempted to convert type to Option<u64>"),
        }
    }
}

impl From<PackTag> for Option<f32> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptF32(val) => val,
            _ => panic!("Attempted to convert type to Option<f32>"),
        }
    }
}

impl From<&PackTag> for Option<f32> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptF32(val) => *val,
            _ => panic!("Attempted to convert type to Option<f32>"),
        }
    }
}

impl From<PackTag> for Option<f64> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptF64(val) => val,
            _ => panic!("Attempted to convert type to Option<f64>"),
        }
    }
}

impl From<&PackTag> for Option<f64> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptF64(val) => *val,
            _ => panic!("Attempted to convert type to Option<f64>"),
        }
    }
}

impl From<PackTag> for Option<bool> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptBool(val) => val,
            _ => panic!("Attempted to convert type to Option<bool>"),
        }
    }
}

impl From<&PackTag> for Option<bool> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptBool(val) => *val,
            _ => panic!("Attempted to convert type to Option<bool>"),
        }
    }
}

impl From<PackTag> for Option<String> {
    fn from(tag: PackTag) -> Self {
        match tag {
            PackTag::OptString(val) => val,
            _ => panic!("Attempted to convert type to Option<String>"),
        }
    }
}

impl From<&PackTag> for Option<String> {
    fn from(tag: &PackTag) -> Self {
        match tag {
            PackTag::OptString(val) => val.clone(),
            _ => panic!("Attempted to convert type to Option<String>"),
        }
    }
}