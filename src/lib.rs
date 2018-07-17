extern crate libc;

use std::mem::size_of;

//const LP_HDR_SIZE: i32 = 6;
//const LP_INTBUF_SIZE: usize = 21;

pub const BEFORE: libc::c_int = 0;
pub const AFTER: libc::c_int = 1;
pub const REPLACE: libc::c_int = 2;

/// Return the existing Rax allocator.
pub unsafe fn allocator() -> (
    extern "C" fn(size: libc::size_t) -> *mut u8,
    extern "C" fn(ptr: *mut libc::c_void, size: libc::size_t) -> *mut u8,
    extern "C" fn(ptr: *mut libc::c_void)) {
    (lp_malloc, lp_realloc, lp_free)
}

/// Listpack internally makes calls to "malloc", "realloc" and "free" for all of it's
/// heap memory needs. These calls can be patched with the supplied hooks.
/// Do not call this method after Listpack has been used at all. This must
/// be called before using or calling any Listpack API function.
pub unsafe fn set_allocator(
    malloc: extern "C" fn(size: libc::size_t) -> *mut u8,
    realloc: extern "C" fn(ptr: *mut libc::c_void, size: libc::size_t) -> *mut u8,
    free: extern "C" fn(ptr: *mut libc::c_void)) {
    lp_malloc = malloc;
    lp_realloc = realloc;
    lp_free = free;
}

pub struct Listpack {
    lp: *mut listpack
}

pub type Element = *mut u8;

pub enum Value {
    Int(i64),
    Str(*const u8, usize),
}

impl Value {
    pub fn as_str<'a>(&self) -> &'a str {
        match *self {
            Value::Int(v) => {
                unsafe {
                    std::str::from_utf8_unchecked(
                        std::slice::from_raw_parts(
                            &v as *const _ as *const u8,
                            std::mem::size_of::<i64>(),
                        )
                    )
                }
            }
            Value::Str(ptr, len) => {
                unsafe {
                    std::str::from_utf8_unchecked(
                        std::slice::from_raw_parts(ptr, len)
                    )
                }
            }
        }
    }
}

pub trait Num: Clone + Default + std::fmt::Debug {
    fn to_int64(self) -> i64;
}

impl Num for isize {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for usize {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for f32 {
    #[inline]
    fn to_int64(self) -> i64 {
        self.to_bits() as i64
    }
}

impl Num for f64 {
    #[inline]
    fn to_int64(self) -> i64 {
        self.to_bits() as i64
    }
}

impl Num for i8 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for u8 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for i16 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for u16 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for i32 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for u32 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Num for i64 {
    #[inline]
    fn to_int64(self) -> i64 {
        self
    }
}

impl Num for u64 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

pub trait Str<RHS = Self>: Clone + Default + std::fmt::Debug {
    type Output: Str;

    fn encode(self) -> Self::Output;

    fn to_buf(&self) -> (*const u8, usize);

    fn from_buf(ptr: *const u8, len: usize) -> RHS;
}

impl Str for f32 {
    type Output = u32;

    #[inline]
    fn encode(self) -> Self::Output {
        // Encode as u32 Big Endian
        self.to_bits().to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        // This should never get called since we represent as a u32
        (self as *const _ as *const u8, std::mem::size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> f32 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe {
            // We used a BigEndian u32 to encode so let's reverse it
            f32::from_bits(
                u32::from_le(
                    *(ptr as *mut [u8; std::mem::size_of::<Self::Output>()] as *mut u32)
                )
            )
        }
    }
}

impl Str for f64 {
    type Output = u64;

    #[inline]
    fn encode(self) -> Self::Output {
        // Encode as u64 Big Endian
        self.to_bits().to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        // This should never get called since we represent as a u64
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> f64 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe {
            // We used a BigEndian u64 to encode so let's reverse it
            f64::from_bits(
                u64::from_le(
                    *(ptr as *mut [u8; size_of::<Self::Output>()] as *mut u64)
                )
            )
        }
    }
}

impl Str for isize {
    type Output = isize;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> isize {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { isize::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut isize)) }
    }
}

impl Str for usize {
    type Output = usize;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, std::mem::size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> usize {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { usize::from_le(*(ptr as *mut [u8; std::mem::size_of::<Self::Output>()] as *mut usize)) }
    }
}

impl Str for i8 {
    type Output = i8;

    #[inline]
    fn encode(self) -> Self::Output {
        self
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> Self {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe {
            *ptr as Self::Output
        }
    }
}

impl Str for u8 {
    type Output = u8;

    #[inline]
    fn encode(self) -> Self::Output {
        self
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> Self {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe {
            *ptr as Self::Output
        }
    }
}

impl Str for i16 {
    type Output = i16;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> Self {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { i16::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut i16)) }
    }
}

impl Str for u16 {
    type Output = u16;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> u16 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { u16::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut u16)) }
    }
}

impl Str for i32 {
    type Output = i32;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> i32 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { i32::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut i32)) }
    }
}

impl Str for u32 {
    type Output = u32;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, std::mem::size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> u32 {
        if len != std::mem::size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { u32::from_le(*(ptr as *mut [u8; std::mem::size_of::<Self::Output>()] as *mut u32)) }
    }
}

impl Str for i64 {
    type Output = i64;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> i64 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { i64::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut i64)) }
    }
}

impl Str for u64 {
    type Output = u64;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> u64 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { u64::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut u64)) }
    }
}

impl Str for i128 {
    type Output = i128;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> i128 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { i128::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut i128)) }
    }
}

impl Str for u128 {
    type Output = u128;

    #[inline]
    fn encode(self) -> Self::Output {
        self.to_le()
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self as *const _ as *const u8, size_of::<Self::Output>())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> u128 {
        if len != size_of::<Self::Output>() {
            return Self::default();
        }
        unsafe { u128::from_le(*(ptr as *mut [u8; size_of::<Self::Output>()] as *mut u128)) }
    }
}

impl Str for Vec<u8> {
    type Output = Vec<u8>;

    #[inline]
    fn encode(self) -> Vec<u8> {
        self
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self.as_ptr(), self.len())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> Vec<u8> {
        unsafe { Vec::from_raw_parts(ptr as *mut u8, len, len) }
    }
}

impl<'a> Str for &'a [u8] {
    type Output = &'a [u8];

    #[inline]
    fn encode(self) -> &'a [u8] {
        self
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        (self.as_ptr(), self.len())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

impl<'a> Str for &'a str {
    type Output = &'a str;

    #[inline]
    fn encode(self) -> Self::Output {
        self
    }

    #[inline]
    fn to_buf(&self) -> (*const u8, usize) {
        ((*self).as_ptr(), self.len())
    }

    #[inline]
    fn from_buf(ptr: *const u8, len: usize) -> &'a str {
        unsafe {
            std::str::from_utf8(
                std::slice::from_raw_parts(ptr, len)
            ).unwrap_or_default()
        }
    }
}


/// Provides a minimal wrapper to Listpack.
impl Listpack {
    pub fn new() -> Listpack {
        return Listpack { lp: unsafe { lpNew() } };
    }

    /// Return the number of elements inside the listpack. This function attempts
    /// to use the cached value when within range, otherwise a full scan is
    /// needed. As a side effect of calling this function, the listpack header
    /// could be modified, because if the count is found to be already within
    /// the 'numele' header field range, the new value is set.
    pub fn len(&self) -> u32 {
        unsafe { lpLength(self.lp) as u32 }
    }

    /// Return the total number of bytes the listpack is composed of.
    pub fn size(&self) -> u32 {
        unsafe { lpBytes(self.lp) }
    }

    /// Decodes and returns the entry value of the element.
    ///
    /// If the function is called against a badly encoded listpack, so that there
    /// is no valid way to parse it, the function returns like if there was an
    /// integer encoded with value 12345678900000000 + <unrecognized byte>, this may
    /// be an hint to understand that something is wrong. To crash in this case is
    /// not sensible because of the different requirements of the application using
    /// this lib.
    ///
    /// Similarly, there is no error returned since the listpack normally can be
    /// assumed to be valid, so that would be a very high API cost. However a function
    /// in order to check the integrity of the listpack at load time is provided,
    /// check is_valid().
    pub fn get(&self, ele: Element) -> Value {
        unsafe {
            let mut count: libc::int64_t = 0;
            let val = lpGet(ele, &mut count, std::ptr::null_mut());

            if val.is_null() {
                Value::Int(count)
            } else {
                Value::Str(val, count as usize)
            }
        }
    }

    ///
    #[inline]
    pub fn get_int(&self, ele: Element) -> i64 {
        self.get_int_or(ele, 0)
    }

    ///
    #[inline]
    pub fn get_int_or(&self, ele: Element, default: i64) -> i64 {
        match self.get(ele) {
            Value::Int(v) => v,
            _ => default
        }
    }

    ///
    #[inline]
    pub fn get_str(&self, ele: Element) -> &str {
        self.get_str_or(ele, "")
    }

    ///
    #[inline]
    pub fn get_str_or<'a>(&self, ele: Element, default: &'a str) -> &'a str {
        match self.get(ele) {
            Value::Str(ptr, len) => {
                unsafe {
                    std::str::from_utf8_unchecked(
                        std::slice::from_raw_parts(ptr, len)
                    )
                }
            }
            _ => default
        }
    }

    /// Insert, delete or replace the specified element 'ele' of length 'len' at
    /// the specified position 'p', with 'p' being a listpack element pointer
    /// obtained with first(), last(), index(), next(), prev() or
    /// seek().
    ///
    /// The element is inserted before, after, or replaces the element pointed
    /// by 'p' depending on the 'where' argument, that can be BEFORE, AFTER
    /// or REPLACE.
    ///
    /// If 'ele' is set to NULL, the function removes the element pointed by 'p'
    /// instead of inserting one.
    ///
    /// Returns None on out of memory or when the listpack total length would exceed
    /// the max allowed size of 2^32-1, otherwise the new pointer to the listpack
    /// holding the new element is returned (and the old pointer passed is no longer
    /// considered valid)
    ///
    /// If 'newp' is not NULL, at the end of a successful call '*newp' will be set
    /// to the address of the element just added, so that it will be possible to
    /// continue an iteration with next() and prev().
    ///
    /// For deletion operations ('ele' set to None) 'newp' is set to the next
    /// element, on the right of the deleted one, or to NULL if the deleted element
    /// was the last one.
    pub fn insert(
        &mut self,
        value: Value,
        p: Element,
        action: libc::c_int,
    ) -> Option<Element> {
        match value {
            Value::Int(v) => self.insert_int(v, p, action),
            Value::Str(ptr, len) => {
                let newp: &mut *mut u8 = &mut std::ptr::null_mut();
                let result = unsafe {
                    lpInsert(
                        self.lp,
                        ptr, len as u32,
                        p,
                        action as libc::c_int,
                        newp,
                    )
                };

                if !result.is_null() {
                    self.lp = result;
                }

                if newp.is_null() {
                    None
                } else {
                    Some(*newp as Element)
                }
            }
        }
    }

    /// Append the specified element 'ele' of length 'len' at the end of the
    /// listpack. It is implemented in terms of insert(), so the return value is
    /// the same as insert().
    pub fn append(&mut self, value: Value) {
        match value {
            Value::Int(v) => self.append_int(v),
            Value::Str(ptr, len) => {
                let result = unsafe {
                    lpAppend(
                        self.lp,
                        ptr, len as u32
                    )
                };

                if !result.is_null() {
                    self.lp = result;
                }
            }
        }
    }

    ///
    pub fn replace(&mut self, value: Value, pos: Element) {
        match value {
            Value::Int(v) => self.replace_int(v, pos),
            Value::Str(ptr, len) => unsafe {
                let newp: &mut *mut u8 = &mut std::ptr::null_mut();
                let result = lpInsert(
                    self.lp,
                    ptr, len as u32,
                    pos,
                    REPLACE,
                    newp,
                );

                if !result.is_null() {
                    self.lp = result;
                }
            }
        }
    }

    /// Insert, delete or replace the specified element 'ele' of length 'len' at
    /// the specified position 'p', with 'p' being a listpack element pointer
    /// obtained with first(), last(), index(), next(), prev() or
    /// seek().
    ///
    /// The element is inserted before, after, or replaces the element pointed
    /// by 'p' depending on the 'where' argument, that can be BEFORE, AFTER
    /// or REPLACE.
    ///
    /// If 'ele' is set to NULL, the function removes the element pointed by 'p'
    /// instead of inserting one.
    ///
    /// Returns None on out of memory or when the listpack total length would exceed
    /// the max allowed size of 2^32-1, otherwise the new pointer to the listpack
    /// holding the new element is returned (and the old pointer passed is no longer
    /// considered valid)
    ///
    /// If 'newp' is not NULL, at the end of a successful call '*newp' will be set
    /// to the address of the element just added, so that it will be possible to
    /// continue an iteration with next() and prev().
    ///
    /// For deletion operations ('ele' set to None) 'newp' is set to the next
    /// element, on the right of the deleted one, or to NULL if the deleted element
    /// was the last one.
    pub fn insert_int<V>(
        &mut self,
        value: V,
        p: Element,
        action: libc::c_int,
    ) -> Option<Element>
        where V: Num {
        unsafe {
            let i = value.to_int64();
            let newp: &mut *mut u8 = &mut std::ptr::null_mut();
            let result = lpInsertInt64(
                self.lp,
                i,
                p,
                action as libc::c_int,
                newp,
            );

            if !result.is_null() {
                self.lp = result;
            }

            if newp.is_null() {
                None
            } else {
                Some(*newp as Element)
            }
        }
    }

    /// Append the specified element 'ele' of length 'len' at the end of the
    /// listpack. It is implemented in terms of insert(), so the return value is
    /// the same as insert().
    pub fn append_int<V>(&mut self, value: V) where V: Num {
        unsafe {
            let i = value.to_int64();
            let result = lpAppendInt64(self.lp, i);
            if !result.is_null() {
                self.lp = result;
            }
        }
    }

    pub fn replace_int<V>(&mut self, value: V, pos: Element) where V: Num {
        unsafe {
            let i = value.to_int64();
            let result = lpReplaceInt64(self.lp, pos as *mut *mut _ as *mut *mut u8, i);
            if !result.is_null() {
                self.lp = result;
            }
        }
    }


    /// Insert, delete or replace the specified element 'ele' of length 'len' at
    /// the specified position 'p', with 'p' being a listpack element pointer
    /// obtained with first(), last(), index(), next(), prev() or
    /// seek().
    ///
    /// The element is inserted before, after, or replaces the element pointed
    /// by 'p' depending on the 'where' argument, that can be BEFORE, AFTER
    /// or REPLACE.
    ///
    /// If 'ele' is set to NULL, the function removes the element pointed by 'p'
    /// instead of inserting one.
    ///
    /// Returns None on out of memory or when the listpack total length would exceed
    /// the max allowed size of 2^32-1, otherwise the new pointer to the listpack
    /// holding the new element is returned (and the old pointer passed is no longer
    /// considered valid)
    ///
    /// If 'newp' is not NULL, at the end of a successful call '*newp' will be set
    /// to the address of the element just added, so that it will be possible to
    /// continue an iteration with next() and prev().
    ///
    /// For deletion operations ('ele' set to None) 'newp' is set to the next
    /// element, on the right of the deleted one, or to NULL if the deleted element
    /// was the last one.
    pub fn insert_str<V>(
        &mut self,
        value: V,
        p: Element,
        action: libc::c_int,
    ) -> Option<Element>
        where V: Str {
        unsafe {
            let v = value.encode();
            let (ele, size) = v.to_buf();
            let newp: &mut *mut u8 = &mut std::ptr::null_mut();
            let result = lpInsert(
                self.lp,
                ele, size as u32,
                p,
                action as libc::c_int,
                newp,
            );

            if !result.is_null() {
                self.lp = result;
            }

            if newp.is_null() {
                None
            } else {
                Some(*newp as Element)
            }
        }
    }

    /// Append the specified element 'ele' of length 'len' at the end of the
    /// listpack. It is implemented in terms of insert(), so the return value is
    /// the same as insert().
    pub fn append_str<V>(&mut self, value: V) where V: Str {
        unsafe {
            let v = value.encode();
            let (ele, size) = v.to_buf();
            let result = lpAppend(self.lp, ele, size as u32);
            if !result.is_null() {
                self.lp = result;
            }
        }
    }

    /// Replace the specified element with the specified value
    pub fn replace_str<V>(&mut self, value: V, ele: Element) where V: Str {
        self.insert_str(value, ele, REPLACE);
    }

    /// Remove the element pointed by 'p', and return the resulting listpack.
    /// If 'newp' is not NULL, the next element pointer (to the right of the
    /// deleted one) is returned by reference. If the deleted element was the
    /// last one, '*newp' is set to None.
    pub fn delete(&mut self, p: Element) -> Option<Element> {
        unsafe {
            let newp: &mut *mut u8 = &mut std::ptr::null_mut();

            let result = lpDelete(self.lp, p, newp);
            if result.is_null() {
                if newp.is_null() {
                    None
                } else {
                    Some(*newp)
                }
            } else {
                self.lp = result;

                if newp.is_null() {
                    None
                } else {
                    Some(*newp)
                }
            }
        }
    }

    /// Return a pointer to the first element of the listpack, or None if the
    /// listpack has no elements.
    pub fn first(&self) -> Option<Element> {
        unsafe {
            let ele = lpFirst(self.lp);
            if ele.is_null() {
                None
            } else {
                Some(ele)
            }
        }
    }

    /// Return a pointer to the last element of the listpack, or None if the
    /// listpack has no elements.
    pub fn last(&self) -> Option<Element> {
        unsafe {
            let ele = lpLast(self.lp);
            if ele.is_null() {
                None
            } else {
                Some(ele)
            }
        }
    }

    pub fn start(&self) -> Element {
        std::ptr::null_mut()
    }

    pub fn first_or_next(&self, after: Element) -> Option<Element> {
        if after.is_null() {
            self.first()
        } else {
            self.next(after)
        }
    }

    /// /* If 'after' points to an element of the listpack, calling next() will return
    /// the pointer to the next element (the one on the right), or None if 'after'
    /// already pointed to the last element of the listpack. */
    pub fn next(&self, after: Element) -> Option<Element> {
        unsafe {
            let ele = lpNext(self.lp, after);
            if ele.is_null() {
                None
            } else {
                Some(ele)
            }
        }
    }

    pub fn last_or_prev(&self, after: Element) -> Option<Element> {
        if after.is_null() {
            self.last()
        } else {
            self.prev(after)
        }
    }

    /// If 'p' points to an element of the listpack, calling prev() will return
    /// the pointer to the previous element (the one on the left), or None if 'before'
    /// already pointed to the first element of the listpack.
    pub fn prev(&self, before: Element) -> Option<Element> {
        unsafe {
            let ele = lpPrev(self.lp, before);
            if ele.is_null() {
                None
            } else {
                Some(ele)
            }
        }
    }

    /// Seek the specified element and returns the pointer to the seeked element.
    /// Positive indexes specify the zero-based element to seek from the head to
    /// the tail, negative indexes specify elements starting from the tail, where
    /// -1 means the last element, -2 the penultimate and so forth. If the index
    /// is out of range, NULL is returned.
    pub fn seek(&self, index: i64) -> Option<Element> {
        unsafe {
            let data = lpSeek(self.lp, index as libc::c_long);
            if data.is_null() {
                None
            } else {
                Some(data)
            }
        }
    }
}

// Map Drop -> "lpFree"
impl Drop for Listpack {
    fn drop(&mut self) {
        unsafe { lpFree(self.lp) }
    }
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct listpack;

#[allow(improper_ctypes)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[link(name = "listpack", kind = "static")]
extern "C" {
    #[no_mangle]
    pub static mut lp_malloc: extern "C" fn(size: libc::size_t) -> *mut u8;
    #[no_mangle]
    pub static mut lp_realloc: extern "C" fn(ptr: *mut libc::c_void, size: libc::size_t) -> *mut u8;
    #[no_mangle]
    pub static mut lp_free: extern "C" fn(ptr: *mut libc::c_void);

    pub fn lp_ll2string(
        dst: *mut u8,
        dstlen: libc::size_t,
        svalue: libc::c_longlong,
    ) -> libc::c_int;

    pub fn lpNew() -> *mut listpack;

    pub fn lpFree(lp: *mut listpack);

    pub fn lpInsert(
        lp: *mut listpack,
        ele: *const u8,
        size: libc::uint32_t,
        p: *mut u8,
        wh: libc::c_int,
        newp: *mut *mut u8,
    ) -> *mut listpack;

    pub fn lpAppend(
        lp: *mut listpack,
        ele: *const u8,
        size: libc::uint32_t) -> *mut listpack;

    pub fn lpDelete(
        lp: *mut listpack,
        p: *mut u8,
        newp: *mut *mut u8,
    ) -> *mut listpack;

    pub fn lpLength(
        lp: *mut listpack
    ) -> libc::uint32_t;

    pub fn lpGet(
        p: *mut u8,
        count: *mut libc::int64_t,
        intbuf: *mut u8,
    ) -> *mut u8;

    pub fn lpGetInteger(
        ele: *mut u8
    ) -> libc::int64_t;

    pub fn lpFirst(lp: *mut listpack) -> *mut u8;

    pub fn lpLast(lp: *mut listpack) -> *mut u8;

    pub fn lpNext(
        lp: *mut listpack,
        p: *mut u8,
    ) -> *mut u8;

    pub fn lpPrev(
        lp: *mut listpack,
        p: *mut u8,
    ) -> *mut u8;

    pub fn lpBytes(
        lp: *mut listpack
    ) -> libc::uint32_t;

    pub fn lpSeek(
        lp: *mut listpack,
        index: libc::c_long,
    ) -> *mut u8;

    pub fn lpInsertInt64(
        lp: *mut listpack,
        value: libc::int64_t,
        p: *mut u8,
        wh: libc::c_int,
        newp: *mut *mut u8,
    ) -> *mut listpack;

    pub fn lpAppendInt64(
        lp: *mut listpack,
        value: libc::int64_t,
    ) -> *mut listpack;

    pub fn lpReplaceInt64(
        lp: *mut listpack,
        pos: *mut *mut u8,
        value: libc::int64_t,
    ) -> *mut listpack;
}


#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn it_works() {
        let mut lp = Listpack::new();

        for i in 0..25 {
            lp.append_int(i as u8);
        }

        lp.append(Value::Int(0));
        lp.append_str("hello");

        println!("Iterate forward...");
        let mut ele = lp.start();
        while let Some(v) = lp.first_or_next(ele) {
            ele = v;
            let val = lp.get(ele);
            match val {
                Value::Int(v) => {
                    println!("Int Value     -> {}", v);
                }
                Value::Str(_v, _len) => {
                    println!("String Value  -> {}", val.as_str());
                }
            }
        }

        println!();
        println!("Iterate backward...");
        let mut ele = lp.start();
        while let Some(v) = lp.last_or_prev(ele) {
            ele = v;
            let val = lp.get(ele);
            match val {
                Value::Int(v) => {
                    println!("Int Value     -> {}", v);
                }
                Value::Str(_v, _len) => {
                    println!("String Value  -> {}", val.as_str());
                }
            }
        }

        println!("Listpack Bytes:   {}", lp.size());
        println!("Listpack Length:  {}", lp.len());
//        assert_eq!(lp.len(), 3);
    }
}