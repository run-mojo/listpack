use std;
use std::ptr;
use ::raw::*;

pub type Element = *mut u8;

trait Listpack {
    fn size(&self) -> u32;

    fn len(&self) -> u16;

    fn seek(&self, i: u16) -> Element;
}

trait MutListpack {
    fn insert(&mut self);
    fn replace(&mut self);
    fn delete(&mut self);
}

trait ListpackAppender: Listpack {
    fn append(&mut self, v: Value);
}

///
pub struct MemListpack {}

/// File of Append-Only listpacks.
pub struct ListpackFile {}


pub trait Appender {
    fn realloc(&mut self, size: u64);

    fn append(&mut self, p: *mut u8, size: usize) -> bool;
}

///
pub trait SegmentReader {
    fn next() -> Option<MemListpack>;

    fn prev() -> Option<MemListpack>;
}

/// Segment reader using a mmap of a file.
pub struct MMapSegmentReader {

}

/// Segment reader using standard blocking File IO.
pub struct FileSegmentReader {

}

/// Segments are the "on-disk" representation of a listpack.
/// Segment files are a series of listpacks appended one after
/// the other. Each listpack is a "page" in the segment file.
/// When reading segment files, it reads a page or listpack at
/// a time in memory.
///
/// Pages can optionally be compressed with LZ4 compression.
/// The part of the pages that contains elements is 1 for 1
/// binary compatible with the in-memory representation.
/// The only difference is the file version does not contain the
/// listpack header. Instead the reader reads entry by entry
/// until it reaches a EOF byte in which case in yields the
/// page and decompresses it if needed in the process.
pub struct SegmentWriter {
    current_len: u64,
    current_count: u16,

    max_page_size: u32,
    max_page_count: u16,

    offset: u64,
    alloc: u64,
    pages: u32,
    // The tail listpack is always a standard listpack.
    // This provides the ability for the SegmentWriter
    // to also allow reads from the tail.
    current_lp: listpack,
}

impl ::raw::Allocator for SegmentWriter {
    #[inline(always)]
    fn has_header(&self) -> bool {
        // Standard listpack header is omitted.
        false
    }

    fn alloc(&self, size: usize) -> *mut u8 {
        unimplemented!()
    }

    fn realloc(&self, lp: *mut u8, size: usize) -> *mut u8 {
        unimplemented!()
    }

    fn dealloc(&self, lp: *mut u8) {
        unimplemented!()
    }
}

impl SegmentWriter {
    pub fn new() -> SegmentWriter {
        SegmentWriter {
            current_lp: ptr::null_mut(),
            current_len: 0,
            current_count: 0,
            max_page_size: 0,
            max_page_count: 0,
            offset: 0,
            alloc: 0,
            pages: 0,
        }
    }

    pub fn append(&mut self, v: Value) {
//        append(self, self.current_lp, v)
    }
}

/// Memory-mapped appender.
pub struct MMapAppender {

}

/// File IO appender.
pub struct FileAppender {

}