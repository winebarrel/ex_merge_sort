//! Rust [external merge sort](https://en.wikipedia.org/wiki/External_sorting#External_merge_sort) library.
//!
//! [![GitHub](https://img.shields.io/badge/github-winebarrel/ex__merge__sort-safegreen?logo=github)](https://github.com/winebarrel/ex_merge_sort)
//! [![Build Status](https://github.com/winebarrel/ex_merge_sort/workflows/CI/badge.svg)](https://github.com/winebarrel/ex_merge_sort/actions)
//! [![crates.io](https://img.shields.io/crates/v/ex_merge_sort.svg)](https://crates.io/crates/ex_merge_sort)
//! [![docs.rs](https://docs.rs/ex_merge_sort/badge.svg)](https://docs.rs/ex_merge_sort)
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! ex_merge_sort = "0.4"
//! ```
//!
//! ```rust
//! use std::fs::File;
//! use std::io;
//!
//! fn main() {
//!     let f = File::open("README.md").unwrap();
//!     let capacity = 1024;
//!     ex_merge_sort::sort(f, io::stdout(), capacity).unwrap();
//! }
//! ```

#[cfg(test)]
mod tests;

mod chunk;
mod file_utils;

use chunk::Chunk;
use file_utils::RoughCount;
use io::prelude::BufRead;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Write;

pub fn sort<T>(fin: fs::File, fout: T, cap: u64) -> io::Result<()>
where
    T: io::Write,
{
    sort_by_key(fin, fout, cap, |s| {
        s.trim_end_matches(|c| c == '\r' || c == '\n').to_string()
    })
}

pub fn sort_by_key<T, F, K>(fin: fs::File, fout: T, cap: u64, conv: F) -> io::Result<()>
where
    T: io::Write,
    F: Fn(&String) -> K,
    K: Ord,
{
    let chunk = Chunk::new(fin, cap)?;
    let sorted = sort_chunk(chunk, &conv)?;
    file_utils::copy(&sorted.file, fout)
}

fn sort_chunk<F, K>(chunk: Chunk, conv: &F) -> io::Result<Chunk>
where
    F: Fn(&String) -> K,
    K: Ord,
{
    if chunk.rough_count == RoughCount::Zero || chunk.rough_count == RoughCount::One {
        return Ok(chunk);
    }

    if chunk.fit_in_buffer() {
        return chunk.sort(conv);
    }

    let (c1, c2) = chunk.split()?;

    if c2.rough_count == RoughCount::Zero {
        return c1.sort(conv);
    }

    Ok(merge(sort_chunk(c1, conv)?, sort_chunk(c2, conv)?, conv)?)
}

fn merge<F, K>(c1: Chunk, c2: Chunk, conv: F) -> io::Result<Chunk>
where
    F: Fn(&String) -> K,
    K: Ord,
{
    assert!(c1.capacity == c2.capacity);

    let mut reader1 = io::BufReader::new(&c1.file);
    let mut reader2 = io::BufReader::new(&c2.file);
    let mut writer = io::BufWriter::new(tempfile::tempfile()?);
    let mut r1_buf = String::new();
    let mut r2_buf = String::new();

    let mut r1_read = reader1.read_line(&mut r1_buf)?;
    let mut r1_key = conv(&r1_buf);
    let mut r2_read = reader2.read_line(&mut r2_buf)?;
    let mut r2_key = conv(&r2_buf);

    while r1_read > 0 && r2_read > 0 {
        if r1_key.partial_cmp(&r2_key).unwrap() == Ordering::Less {
            // r1_key < r2_key
            writer.write(&r1_buf.as_bytes())?;
            r1_buf.clear();
            r1_read = reader1.read_line(&mut r1_buf)?;
            r1_key = conv(&r1_buf);
        } else {
            // r1_buf >= r2_buf
            writer.write(&r2_buf.as_bytes())?;
            r2_buf.clear();
            r2_read = reader2.read_line(&mut r2_buf)?;
            r2_key = conv(&r2_buf);
        }
    }

    while r1_read > 0 {
        writer.write(&r1_buf.as_bytes())?;
        r1_buf.clear();
        r1_read = reader1.read_line(&mut r1_buf)?
    }

    while r2_read > 0 {
        writer.write(&r2_buf.as_bytes())?;
        r2_buf.clear();
        r2_read = reader2.read_line(&mut r2_buf)?
    }

    let cap = c1.capacity;
    Ok(Chunk::new(writer.into_inner()?, cap)?)
}
