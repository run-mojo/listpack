use raw::*;

pub mod raw;
pub mod segment;

pub trait ListpackLike {}

pub struct Listpack(listpack);

impl Listpack {
    pub fn new() -> Listpack {
        Listpack(new(ALLOCATOR))
    }

    #[inline]
    pub fn insert<V: Into<Value>>(
        &mut self,
        v: V,
        place: Placement,
        target: element,
    ) -> Option<element> {
        match insert(ALLOCATOR, self.0, v.into(), place, target) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn insert_val(
        &mut self,
        v: Value,
        place: Placement,
        target: element,
    ) -> Option<element> {
        match insert(ALLOCATOR, self.0, v, place, target) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn insert_int<T>(
        &mut self,
        v: T,
        place: Placement,
        target: element
    ) -> Option<element>
        where
            T: Int {
        match insert_int(ALLOCATOR, self.0, v, place, target) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn insert_signed_int<T>(
        &mut self,
        v: T,
        place: Placement,
        target: element
    ) -> Option<element>
        where
            T: Int {
        match insert_signed_int(ALLOCATOR, self.0, v, place, target) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn insert_string<T>(
        &mut self,
        v: T,
        place: Placement,
        target: element
    ) -> Option<element>
        where
            T: Str {
        match insert_string(ALLOCATOR, self.0, v, place, target) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn replace<V: Into<Value>>(
        &mut self,
        p: element,
        v: V
    ) -> Option<element> {
        match replace(ALLOCATOR, self.0, p, v.into()) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn replace_val(
        &mut self,
        p: element,
        v: Value
    ) -> Option<element> {
        match replace(ALLOCATOR, self.0, p, v) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn replace_int<T>(
        &mut self,
        p: element,
        v: T
    ) -> Option<element>
        where
            T: Int {
        match replace_int(ALLOCATOR, self.0, p, v) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn replace_signed_int<T>(
        &mut self,
        p: element,
        v: T
    ) -> Option<element>
        where
            T: Int {
        match replace_signed_int(ALLOCATOR, self.0, p, v) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn replace_string<T>(
        &mut self,
        p: element,
        v: T
    ) -> Option<element>
        where
            T: Str {
        match replace_string(ALLOCATOR, self.0, p, v) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn append_val(
        &mut self,
        v: Value
    ) -> bool {
        match append(ALLOCATOR, self.0, v) {
            Some(lp) => {
                self.0 = lp;
                true
            }
            None => false
        }
    }

    #[inline]
    pub fn append<V: Into<Value>>(
        &mut self,
        v: V
    ) -> bool {
        match append(ALLOCATOR, self.0, v.into()) {
            Some(lp) => {
                self.0 = lp;
                true
            }
            None => false
        }
    }

    #[inline]
    pub fn append_int<T>(
        &mut self,
        v: T
    ) -> bool
        where
            T: Int {
        match append_int(ALLOCATOR, self.0, v) {
            Some(lp) => {
                self.0 = lp;
                true
            }
            None => false
        }
    }

    #[inline]
    pub fn append_signed_int<T>(
        &mut self,
        v: T
    ) -> bool
        where
            T: Int {
        match append_signed_int(ALLOCATOR, self.0, v) {
            Some(lp) => {
                self.0 = lp;
                true
            }
            None => false
        }
    }

    #[inline]
    pub fn append_string<T>(
        &mut self,
        v: T
    ) -> bool
        where
            T: Str {
        match append_string(ALLOCATOR, self.0, v) {
            Some(lp) => {
                self.0 = lp;
                true
            }
            None => false
        }
    }

    #[inline]
    pub fn delete(
        &mut self,
        p: element
    ) -> Option<element> {
        match delete(ALLOCATOR, self.0, p) {
            Some((lp, ele)) => {
                self.0 = lp;
                Some(ele)
            }
            None => None
        }
    }

    #[inline]
    pub fn get(&self, ele: element) -> Option<Value> {
        if ele.is_null() {
            None
        } else {
            Some(get(ele))
        }
    }

    #[inline(always)]
    pub fn get_i8(&self, ele: element) -> i8 {
        get_i8(ele)
    }

    #[inline(always)]
    pub fn get_u8(&self, ele: element) -> u8 {
        get_u8(ele)
    }

    #[inline(always)]
    pub fn get_i16(&self, ele: element) -> i16 {
        get_i16(ele)
    }

    #[inline(always)]
    pub fn get_u16(&self, ele: element) -> u16 {
        get_u16(ele)
    }

    #[inline(always)]
    pub fn get_i32(&self, ele: element) -> i32 {
        get_i32(ele)
    }

    #[inline(always)]
    pub fn get_u32(&self, ele: element) -> u32 {
        get_u32(ele)
    }

    #[inline(always)]
    pub fn get_i64(&self, ele: element) -> i64 {
        get_i64(ele)
    }

    #[inline(always)]
    pub fn get_u64(&self, ele: element) -> u64 {
        get_u64(ele)
    }

    #[inline(always)]
    pub fn get_i128(&self, ele: element) -> i128 {
        get_i128(ele)
    }

    #[inline(always)]
    pub fn get_u128(&self, ele: element) -> u128 {
        get_u128(ele)
    }

    #[inline(always)]
    pub fn get_f32(&self, ele: element) -> f32 {
        get_f32(ele)
    }

    #[inline(always)]
    pub fn get_f64(&self, ele: element) -> f64 {
        get_f64(ele)
    }

    #[inline(always)]
    pub fn get_isize(&self, ele: element) -> isize {
        get_isize(ele)
    }

    #[inline(always)]
    pub fn get_usize(&self, ele: element) -> usize {
        get_usize(ele)
    }

    #[inline(always)]
    pub fn get_int(&self, ele: element) -> i64 {
        get_int(ele)
    }

    #[inline(always)]
    pub fn get_signed_int(&self, ele: element) -> i64 {
        get_signed_int(ele)
    }

    #[inline(always)]
    pub fn get_str<'a>(&self, ele: element) -> &'a str {
        get_str(ele)
    }

    #[inline(always)]
    pub fn get_bytes<'a>(&self, ele: element) -> &'a [u8] {
        get_bytes(ele)
    }

    #[inline(always)]
    pub fn seek() {

    }
}

impl Drop for Listpack {
    fn drop(&mut self) {
        ALLOCATOR.dealloc(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append() {
        use std;

        let mut lp = Listpack::new();

        for i in 0..25 {
            lp.append(i);
            lp.append(format!("{}", i));
        }

        lp.append("hello");

        iter(lp.0, |ele, value| {

            match value {
                Value::Int(int) => {
                    println!("Int:    {}", int);
                }
                Value::String(string, slen) => {
                    println!("String: {}", value.as_str());
                }
            }
            true
        });
    }
}


