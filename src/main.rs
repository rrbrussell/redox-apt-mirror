//Copyright (C) 2021 Robert R. Russell
//This program is free software; you can redistribute it and/or modify it under
//the terms of the GNU General Public License as published by the Free Software
//Foundation; version 2.
//
//This program is distributed in the hope that it will be useful, but WITHOUT
//ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
//FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with
//this program; if not, write to the Free Software Foundation, Inc., 51 Franklin
//Street, Fifth Floor, Boston, MA 02110-1301, USA.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! The redox-apt-mirror is a replacement for apt-mirror.

use chrono::DateTime;
use chrono::Utc;
use std::error::Error;
use std::fs::File;
//use std::io;
//use std::io::prelude::*;
use std::io::{BufRead, BufReader};
//use voca_rs::manipulate::trim;
//use voca_rs::query;
use voca_rs::Voca;

const FILENAME: &str = "Debian-InRelease";

/// main does stuff
///
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(FILENAME)?;
    let file_reader = BufReader::new(file);
    let mut release: ReleaseData;
    for line in file_reader.lines().map(|l| l.unwrap()) {
        if line.trim().starts_with("Date:") {
            release.date = DateTime::parse_from_rfc2822(line).unwrap();
            println!("{}", line);
        }
    }
    return Ok(());
}

/// Which compression algorithm is used for an index.
#[derive(Debug)]
enum Compression {
    NONE,
    XZ,
    GZIP,
    BZIP2,
    LZMA,
}

/// ReleaseData represents the control file that is used to define a repository
#[derive(Debug)]
struct ReleaseData {
    description: Option<String>,
    origin: Option<String>,
    label: Option<String>,
    version: Option<String>,
    suite: String,
    codename: String,
    components: Vec<String>,
    architectures: Vec<String>,
    date: DateTime<Utc>,
    valid_until: Option<DateTime<Utc>>,
    files: Option<Vec<HashedFile>>,
    not_automatic: Option<bool>,
    but_automatic_upgrades: Option<bool>,
    acquire_by_hash: Option<bool>,
    signed_by: Option<Vec<String>>,
}

/// HashedFile represents the individual files linked to from the InRelease file.
/// This struct does not seperate between Packages, Sources, Contents, or Indices.
#[derive(Debug)]
struct HashedFile {
    name: String,
    compression: Compression,
    md5: Option<String>,
    sha1: Option<String>,
    sha256: Option<String>,
    sha512: Option<String>,
}
