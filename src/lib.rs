#![allow(dead_code)]

extern crate libc;

use std::mem::size_of;

pub const HDR_SIZE: i32 = 6;
const INTBUF_SIZE: usize = 21;

pub const BEFORE: libc::c_int = 0;
pub const AFTER: libc::c_int = 1;
pub const REPLACE: libc::c_int = 2;

pub const EMPTY: &'static [u8] = &[];

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

impl Into<Value> for u8 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for u8 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len == 0 || ptr.is_null() {
                    u8::default()
                } else {
                    *ptr
                }
            }
        }
    }
}

impl Into<Value> for i8 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for i8 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len == 0 {
                    i8::default()
                } else {
                    *ptr as Self
                }
            }
        }
    }
}

impl Into<Value> for u16 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for u16 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for i16 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for i16 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for u32 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for u32 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for i32 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for i32 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for u64 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for u64 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for i64 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for i64 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for isize {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for isize {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for usize {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self as i64)
    }
}

impl From<Value> for usize {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for f32 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self.to_bits() as i64)
    }
}

impl From<Value> for f32 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => Self::from_bits(i as u32),
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    // We used a Little Endian to encode so let's reverse it
                    f32::from_bits(
                        u32::from_le(
                            *(ptr as *mut [u8; std::mem::size_of::<Self>()] as *mut u32)
                        )
                    )
                }
            }
        }
    }
}

impl Into<Value> for f64 {
    #[inline]
    fn into(self) -> Value {
        Value::Int(self.to_bits() as i64)
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => Self::from_bits(i as u64),
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    // Maybe f32 was used?
                    if len == std::mem::size_of::<f32>() {
                        f32::from_bits(
                            u32::from_le(
                                *(ptr as *mut [u8; std::mem::size_of::<Self>()] as *mut u32)
                            )
                        ) as f64
                    } else {
                        Self::default()
                    }
                } else {
                    // We used a Little Endian to encode so let's reverse it
                    f64::from_bits(
                        u64::from_le(
                            *(ptr as *mut [u8; std::mem::size_of::<Self>()] as *mut u64)
                        )
                    )
                }
            }
        }
    }
}

impl Into<Value> for u128 {
    #[inline]
    fn into(self) -> Value {
        Value::Str(&self as *const _ as *const u8,
                   std::mem::size_of::<u128>())
    }
}

impl From<Value> for u128 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl Into<Value> for i128 {
    #[inline]
    fn into(self) -> Value {
        Value::Str(&self as *const _ as *const u8,
                   std::mem::size_of::<i128>())
    }
}

impl From<Value> for i128 {
    fn from(v: Value) -> Self {
        match &v {
            &Value::Int(i) => i as Self,
            &Value::Str(ptr, len) => unsafe {
                if len != std::mem::size_of::<Self>() {
                    Self::default()
                } else {
                    Self::from_le(*(ptr as *mut [u8; size_of::<Self>()] as *mut Self))
                }
            }
        }
    }
}

impl<'a> Into<Value> for &'a str {
    #[inline]
    fn into(self) -> Value {
        Value::Str(self.as_ptr(), self.len())
    }
}

impl<'a> Into<Value> for &'a [u8] {
    #[inline]
    fn into(self) -> Value {
        Value::Str(self.as_ptr(), self.len())
    }
}

impl<'a> Into<Value> for &'a String {
    #[inline]
    fn into(self) -> Value {
        Value::Str(self.as_ptr(), self.len())
    }
}

impl Into<Value> for String {
    #[inline]
    fn into(self) -> Value {
        Value::Str(self.as_ptr(), self.len())
    }
}

impl<'a> Into<Value> for &'a Vec<u8> {
    #[inline]
    fn into(self) -> Value {
        Value::Str(self.as_ptr(), self.len())
    }
}

impl Into<Value> for Vec<u8> {
    #[inline]
    fn into(self) -> Value {
        Value::Str(self.as_ptr(), self.len())
    }
}

impl Value {
    #[inline]
    pub fn as_bytes<'a>(&self) -> &'a [u8] {
        match *self {
            Value::Int(v) => {
                unsafe {
                    std::slice::from_raw_parts(
                        &v as *const _ as *const u8,
                        std::mem::size_of::<i64>(),
                    )
                }
            }
            Value::Str(ptr, len) => {
                if ptr.is_null() || len == 0 {
                    EMPTY
                } else {
                    unsafe {
                        std::slice::from_raw_parts(ptr, len as usize)
                    }
                }
            }
        }
    }

    #[inline]
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
                        std::slice::from_raw_parts(ptr, len as usize)
                    )
                }
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            &Value::Int(v) => {
                match other {
                    &Value::Int(v2) => v == v2,
                    &Value::Str(_, _) => false
                }
            }
            &Value::Str(ptr, len) => {
                match other {
                    &Value::Int(_) => false,
                    &Value::Str(ptr2, len2) => unsafe {
                        if len != len2 {
                            false
                        } else if ptr == ptr2 {
                            true
                        } else if ptr.is_null() || ptr2.is_null() {
                            false
                        } else {
                            libc::memcmp(
                                ptr as *mut libc::c_void,
                                ptr2 as *mut libc::c_void,
                                len,
                            ) == 0
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        Some(match self {
            &Value::Int(v) => {
                match other {
                    &Value::Int(v2) => {
                        v.cmp(&v2)
                    }
                    &Value::Str(_, _) => std::cmp::Ordering::Less
                }
            }
            &Value::Str(ptr, len) => {
                match other {
                    &Value::Int(_) => std::cmp::Ordering::Greater,
                    &Value::Str(ptr2, len2) => unsafe {
                        match len.cmp(&len2) {
                            std::cmp::Ordering::Less => {
                                if len == 0 {
                                    std::cmp::Ordering::Less
                                } else {
                                    let r = libc::memcmp(
                                        ptr as *mut libc::c_void,
                                        ptr2 as *mut libc::c_void,
                                        len,
                                    );

                                    if r <= 0 {
                                        std::cmp::Ordering::Less
                                    } else {
                                        std::cmp::Ordering::Greater
                                    }
                                }
                            }
                            std::cmp::Ordering::Equal => {
                                if len <= 0 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    let r = libc::memcmp(
                                        ptr as *mut libc::c_void,
                                        ptr2 as *mut libc::c_void,
                                        len,
                                    );

                                    if r == 0 {
                                        std::cmp::Ordering::Equal
                                    } else if r < 0 {
                                        std::cmp::Ordering::Less
                                    } else {
                                        std::cmp::Ordering::Greater
                                    }
                                }
                            }
                            std::cmp::Ordering::Greater => {
                                if len2 == 0 {
                                    std::cmp::Ordering::Greater
                                } else {
                                    let r = libc::memcmp(
                                        ptr as *mut libc::c_void,
                                        ptr2 as *mut libc::c_void,
                                        len2,
                                    );

                                    if r >= 0 {
                                        std::cmp::Ordering::Greater
                                    } else {
                                        std::cmp::Ordering::Less
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

pub trait Int: Clone + Default + std::fmt::Debug {
    fn to_int64(self) -> i64;
}

impl Int for isize {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for usize {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for f32 {
    #[inline]
    fn to_int64(self) -> i64 {
        self.to_bits() as i64
    }
}

impl Int for f64 {
    #[inline]
    fn to_int64(self) -> i64 {
        self.to_bits() as i64
    }
}

impl Int for i8 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for u8 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for i16 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for u16 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for i32 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for u32 {
    #[inline]
    fn to_int64(self) -> i64 {
        self as i64
    }
}

impl Int for i64 {
    #[inline]
    fn to_int64(self) -> i64 {
        self
    }
}

impl Int for u64 {
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
        // Encode as u32 Little Endian
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
            // We used a Little u32 to encode so let's reverse it
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
        // Encode as u64 Little Endian
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
            // We used a Little Endian u64 to encode so let's reverse it
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


/// A listpack is encoded into a single linear chunk of memory. It has a fixed
/// length header of six bytes (instead of ten bytes of ziplist, since we no
/// longer need a pointer to the start of the last element). The header is
/// followed by the listpack elements. In theory the data structure does not need
/// any terminator, however for certain concerns, a special entry marking the
/// end of the listpack is provided, in the form of a single byte with value
/// FF (255). The main advantages of the terminator are the ability to scan the
/// listpack without holding (and comparing at each iteration) the address of
/// the end of the listpack, and to recognize easily if a listpack is well
/// formed or truncated. These advantages are, in the idea of the writer, worth
/// the additional byte needed in the representation.
/// 
///     <tot-bytes> <num-elements> <element-1> ... <element-N> <listpack-end-byte>
/// 
/// The six byte header, composed of the tot-bytes and num-elements fields is
/// encoded in the following way:
/// 
/// * `tot-bytes`: 32 bit unsigned integer holding the total amount of bytes
/// representing the listpack. Including the header itself and the terminator.
/// This basically is the total size of the allocation needed to hold the listpack
/// and allows to jump at the end in order to scan the listpack in reverse order,
/// from the last to the first element, when needed.
/// * `num-elements`:  16 bit unsigned integer holding the total number of elements
/// the listpack holds. However if this field is set to 65535, which is the greatest
/// unsigned integer representable in 16 bit, it means that the number of listpack
/// elements is not known, so a LIST-LENGTH operation will require to fully scan
/// the listpack. This happens when, at some point, the listpack has a number of
/// elements equal or greater than 65535. The num-elements field will be set again
/// to a lower number the first time a LIST-LENGTH operation detects the elements
/// count returned in the representable range.
/// 
/// All integers in the listpack are stored in little endian format, if not
/// otherwise specified (certain special encodings are in big endian because
/// it is more natural to represent them in this way for the way the specification
/// maps to C code).
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
        unsafe { u32::from_le(lpLength(self.lp) as u32) }
    }

    /// Return the total number of bytes the listpack is composed of.
    pub fn size(&self) -> u32 {
        unsafe { u32::from_le(lpBytes(self.lp)) }
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

    #[inline]
    pub fn get_i8(&self, ele: Element) -> i8 {
        i8::from(self.get(ele))
    }

    #[inline]
    pub fn append_i8(&mut self, v: i8) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn replace_i8(&mut self, ele: Element, v: i8) -> Option<Element> {
        self.replace_int(ele, v as i64)
    }

    #[inline]
    pub fn append_i8_fixed(&mut self, v: i8) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_u8(&self, ele: Element) -> u8 {
        u8::from(self.get(ele))
    }

    #[inline]
    pub fn append_u8(&mut self, v: u8) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_u8_fixed(&mut self, v: u8) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_i16(&self, ele: Element) -> i16 {
        i16::from(self.get(ele))
    }

    #[inline]
    pub fn append_i16(&mut self, v: i16) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_i16_fixed(&mut self, v: i16) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_u16(&self, ele: Element) -> u16 {
        u16::from(self.get(ele))
    }

    #[inline]
    pub fn append_u16(&mut self, v: u16) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_u16_fixed(&mut self, v: u16) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_i32(&self, ele: Element) -> i32 {
        i32::from(self.get(ele))
    }

    #[inline]
    pub fn append_i32(&mut self, v: i32) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_i32_fixed(&mut self, v: i32) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_u32(&self, ele: Element) -> u32 {
        u32::from(self.get(ele))
    }

    #[inline]
    pub fn append_u32(&mut self, v: u32) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_u32_fixed(&mut self, v: u32) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_i64(&self, ele: Element) -> i64 {
        i64::from(self.get(ele))
    }

    #[inline]
    pub fn append_i64(&mut self, v: i64) -> bool {
        self.append_int(v)
    }

    #[inline]
    pub fn append_i64_fixed(&mut self, v: i64) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_u64(&self, ele: Element) -> u64 {
        u64::from(self.get(ele))
    }

    #[inline]
    pub fn append_u64(&mut self, v: u64) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_u64_fixed(&mut self, v: u64) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_isize(&self, ele: Element) -> isize {
        isize::from(self.get(ele))
    }

    #[inline]
    pub fn append_isize(&mut self, v: isize) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_isize_fixed(&mut self, v: isize) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_usize(&self, ele: Element) -> usize {
        usize::from(self.get(ele))
    }

    #[inline]
    pub fn append_usize(&mut self, v: usize) -> bool {
        self.append_int(v as i64)
    }

    #[inline]
    pub fn append_usize_fixed(&mut self, v: usize) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_f32(&self, ele: Element) -> f32 {
        f32::from(self.get(ele))
    }

    #[inline]
    pub fn append_f32(&mut self, v: f32) -> bool {
        self.append_int(v.to_bits() as i64)
    }

    #[inline]
    pub fn append_f32_fixed(&mut self, v: f32) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_f64(&self, ele: Element) -> f64 {
        f64::from(self.get(ele))
    }

    #[inline]
    pub fn append_f64(&mut self, v: f64) -> bool {
        self.append_int(v.to_bits() as i64)
    }

    #[inline]
    pub fn append_f64_fixed(&mut self, v: f64) -> bool {
        self.append_str(v)
    }

    #[inline]
    pub fn get_i128(&self, ele: Element) -> i128 {
        i128::from(self.get(ele))
    }

//    #[inline]
//    pub fn append_i128(&mut self, v: i128) {
//        // Is it within 64bit boundaries?
//        if v < i64::max_value() as i128 && v >= i64::min_value() as i128 {
//            self.append_int(v as i64)
//        } else {
//            self.append(v.into())
//        }
//    }
//
//    #[inline]
//    pub fn get_u128(&self, ele: Element) -> u128 {
//        u128::from(self.get(ele))
//    }
//
//    #[inline]
//    pub fn append_u128(&mut self, v: u128) {
//        // Is it within 64bit boundaries?
//        if v < i64::max_value() as u128 {
//            self.append_int(v as i64)
//        } else {
//            self.append(v.into())
//        }
//    }

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
                        std::slice::from_raw_parts(ptr, len as usize)
                    )
                }
            }
            Value::Int(_) => {
//                let s: &'a mut String = Box::leak(Box::new(format!("{}", v)));
//                s.as_str()
                default
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
                let new_lp = unsafe {
                    lpInsert(
                        self.lp,
                        ptr, len as u32,
                        p,
                        action as libc::c_int,
                        newp,
                    )
                };

                if !new_lp.is_null() {
                    self.lp = new_lp;
                    if (*newp).is_null() {
                        None
                    } else {
                        Some(*newp)
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Append the specified element 'ele' of length 'len' at the end of the
    /// listpack. It is implemented in terms of insert(), so the return value is
    /// the same as insert().
    pub fn append(&mut self, value: Value) -> bool {
        match value {
            Value::Int(v) => self.append_int(v),
            Value::Str(ptr, len) => {
                let new_lp = unsafe {
                    lpAppend(
                        self.lp,
                        ptr, len as u32,
                    )
                };

                if !new_lp.is_null() {
                    self.lp = new_lp;
                    true
                } else {
                    false
                }
            }
        }
    }

    ///
    pub fn replace(&mut self, pos: Element, value: Value) -> Option<Element> {
        match value {
            Value::Int(v) => self.replace_int(pos, v),
            Value::Str(ptr, len) => unsafe {
                let mut newp: &mut *mut u8 = &mut std::ptr::null_mut();
                let new_lp = lpInsert(
                    self.lp,
                    ptr, len as u32,
                    pos,
                    REPLACE,
                    newp,
                );

                if !new_lp.is_null() {
                    self.lp = new_lp;
                    if (*newp).is_null() {
                        None
                    } else {
                        Some(*newp)
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Insert, delete or replace the specified element 'ele' of length 'len' at
    /// the specified position 'p', with 'p' being a listpack element pointer
    /// obtained with first(), last(), index(), next(), prev() or seek().
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
    pub fn insert_int<V>(
        &mut self,
        value: V,
        p: Element,
        action: libc::c_int,
    ) -> Option<Element>
        where V: Int {
        unsafe {
            let i = value.to_int64();
            let newp: &mut *mut u8 = &mut std::ptr::null_mut();
            let new_lp = lpInsertInt64(
                self.lp,
                i,
                p,
                action as libc::c_int,
                newp,
            );

            if !new_lp.is_null() {
                self.lp = new_lp;
                if (*newp).is_null() {
                    None
                } else {
                    Some(*newp)
                }
            } else {
                None
            }
        }
    }

    /// Append the specified element 'ele' of length 'len' at the end of the
    /// listpack. It is implemented in terms of insert(), so the return value is
    /// the same as insert().
    ///
    /// Returns true if it succeeded or false when out-of-memory or when the
    /// listpack total length would exceed the max allowed size of 2^32-1
    pub fn append_int<V>(&mut self, value: V) -> bool where V: Int {
        unsafe {
            let i = value.to_int64();
            let result = lpAppendInt64(self.lp, i);
            if !result.is_null() {
                self.lp = result;
                true
            } else {
                false
            }
        }
    }

    pub fn replace_int<V>(&mut self, mut pos: Element, value: V) -> Option<Element> where V: Int {
        unsafe {
            let i = value.to_int64();
            let newp: &mut *mut u8 = &mut pos;
            let new_lp = lpReplaceInt64(self.lp, newp, i);
            if !new_lp.is_null() {
                self.lp = new_lp;
                if (*newp).is_null() {
                    None
                } else {
                    Some(*newp)
                }
            } else {
                None
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
    ///
    /// If None is returned then it failed because of out-of-memory or invalid
    /// element pointer.
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

            if result.is_null() {
                None
            } else {
                self.lp = result;
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
    pub fn append_str<V>(&mut self, value: V) -> bool where V: Str {
        unsafe {
            let v = value.encode();
            let (ele, size) = v.to_buf();
            let result = lpAppend(self.lp, ele, size as u32);
            if !result.is_null() {
                self.lp = result;
                true
            } else {
                false
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

    pub unsafe fn unsafe_set_bytes(&mut self, len: u32) {
        let p = self.lp as *mut u8;
        *p.offset(1) = (len & 0xff) as u8;
        *p.offset(2) = ((len >> 8) & 0xff) as u8;
        *p.offset(3) = ((len >> 16) & 0xff) as u8;
        *p.offset(4) = ((len >> 24) & 0xff) as u8;
    }

    pub unsafe fn unsafe_set_len(&mut self, len: u16) {
        let p = self.lp as *mut u8;
        *p.offset(5) = (len & 0xff) as u8;
        *p.offset(6) = ((len >> 8) & 0xff) as u8;
    }
}

// Map Drop -> "lpFree"
impl Drop for Listpack {
    fn drop(&mut self) {
        unsafe { lpFree(self.lp) }
    }
}

/// Opaque C type from listpack.c
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[repr(C)]
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
        newp: &mut *mut u8,
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
        pos: &mut *mut u8,
        value: libc::int64_t,
    ) -> *mut listpack;
}

/// A facade to represent a series of contiguous fields as a Record.
pub trait Adapter {}


#[cfg(test)]
mod tests {
    use *;

    fn print_cmp(lp: &mut Listpack, ele_1: Element, ele_2: Element) {
        match ele_1.cmp(&ele_2) {
            std::cmp::Ordering::Less => {
                println!("{} < {}", lp.get_str(ele_1), lp.get_str(ele_2));
                ;
            }
            std::cmp::Ordering::Equal => {
                println!("{} == {}", lp.get_str(ele_1), lp.get_str(ele_2));
            }
            std::cmp::Ordering::Greater => {
                println!("{} > {}", lp.get_str(ele_1), lp.get_str(ele_2));
            }
        }
    }

    #[test]
    fn test_cmp() {
        let mut lp = Listpack::new();
        lp.append_str("hello");
        lp.append_str("bye");

        let ele_1 = lp.first().unwrap();
        let ele_2 = lp.seek(1).unwrap();

        print_cmp(&mut lp, ele_1, ele_2);
    }

    #[test]
    fn test_replace() {
        let mut lp = Listpack::new();
        println!("lp size before append_i32: {}", lp.size());
        lp.append_i32(1);

        println!("lp size before append_i32_fixed: {}", lp.size());
        lp.append_i32_fixed(1);
        println!("lp size after append_i32_fixed:  {}", lp.size());

        let v1 = lp.get_i32(lp.first().unwrap());
        let v2 = lp.get_i32(lp.next(lp.first().unwrap()).unwrap());

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_append_helpers() {
        let mut lp = Listpack::new();
        println!("lp size before append_i32: {}", lp.size());
        lp.append_i32(1);

        println!("lp size before append_i32_fixed: {}", lp.size());
        lp.append_i32_fixed(1);
        println!("lp size after append_i32_fixed:  {}", lp.size());

        let v1 = lp.get_i32(lp.first().unwrap());
        let v2 = lp.get_i32(lp.next(lp.first().unwrap()).unwrap());

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_multiple() {
        let mut lp = Listpack::new();

        for i in 0..24 {
            lp.append_int(i as u8);
        }

        lp.append(Value::Int(25));
        lp.append("hi".into());
        lp.append_str("hello");
        lp.append_str("bye");

        println!("Iterate forward...");
        let mut ele = lp.start();
        while let Some(v) = lp.first_or_next(ele) {
            ele = v;
            let val = lp.get(ele);
            match val {
                Value::Int(v) => {
                    println!("Int     -> {}", v);
                }
                Value::Str(_v, _len) => {
                    println!("String  -> {}", val.as_str());
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
                    println!("Int     -> {}", v);
                }
                Value::Str(_v, _len) => {
                    println!("String  -> {}", val.as_str());
                }
            }
        }

        println!();
        println!("Seeking to index 10...");
        match lp.seek(10) {
            Some(el) => {
                println!("Found...");
                let val = lp.get(el);
                match val {
                    Value::Int(v) => {
                        println!("Int     -> {}", v);
                    }
                    Value::Str(_v, _len) => {
                        println!("String  -> {}", val.as_str());
                    }
                }
            }
            None => {
                println!("Not Found!");
            }
        }

        println!();
        println!("Seeking to index 100...");
        match lp.seek(100) {
            Some(el) => {
                println!("Found...");
                let val = lp.get(el);
                match val {
                    Value::Int(v) => {
                        println!("Int     -> {}", v);
                    }
                    Value::Str(_v, _len) => {
                        println!("String  -> {}", val.as_str());
                    }
                }
            }
            None => {
                println!("Not Found!");
            }
        }

        println!();
        println!("Bytes:            {}", lp.size());
        println!("Length:           {}", lp.len());
        println!("Bytes / Element:  {}", (lp.size() - 6) as f32 / lp.len() as f32);
//        assert_eq!(lp.len(), 3);
    }
}