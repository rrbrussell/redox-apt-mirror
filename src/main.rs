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
#![allow(dead_code)]
#![allow(unused_imports)]
//! The redox-apt-mirror is a replacement for apt-mirror.

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use voca_rs::manipulate::trim;
use voca_rs::split::split;
//use voca_rs::query;
use voca_rs::Voca;

const FILENAME: &str = "Debian-InRelease";

/// main does stuff
///
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(FILENAME)?;
    let file_reader = BufReader::new(file);
    let mut _data: ReleaseData = ReleaseData::new();
    //let _date: DateTime<Utc>;
    let mut lines = file_reader.lines().map(|l| l.unwrap());
    loop {
        let line = lines.next();
        if line.is_some() {
            let trimmed_line = trim(&line.unwrap(), " ");
            if trimmed_line.contains("----BEGIN PGP SIGNED MESSAGE-----") {
                // skip the Hash: line from an inline Ascii Armored signature
                // and the blank line following it.
                lines.next();
                lines.next();
                continue;
            }
            if trimmed_line.contains("-----BEGIN PGP SIGNATURE-----") {
                // The ASCII Armored signature block is at the end of the file.
                break;
            }
            // Process a Date tag.
            if trimmed_line.starts_with("Date:") {
                _data.date = parse_a_date(&trimmed_line);
            }
            // Process a suite tag
            if trimmed_line.starts_with("Suite:") {
                _data.suite = parse_a_suite(&trimmed_line);
            }
            // Process a codename
            if trimmed_line.starts_with("Codename:") {
                _data.codename = parse_a_codename(&trimmed_line);
            }
            if trimmed_line.starts_with("Components:") {
                _data.components = parse_the_components(&trimmed_line);
            }
            if trimmed_line.starts_with("Architectures:") {
                _data.architectures = parse_the_architectures(&trimmed_line);
            }
        } else {
            break;
        }
    }
    println!("{:#?}", _data);
    return Ok(());
}

fn parse_the_architectures(input_line: &String) -> Vec<String> {
    let index = input_line
        .find(':')
        .expect("A : went missing in parse_the_architectures");
    let last_part = input_line[(index + 1)..].trim();
    let vecstr = last_part._split(" ");
    let mut v = Vec::<String>::with_capacity(vecstr.len());
    for i in vecstr.iter() {
        v.push(String::from(*i));
    }
    return v;
}

/// parses the components
fn parse_the_components(input_line: &String) -> Vec<String> {
    let index = input_line
        .find(':')
        .expect("A : went missing in parse_the_components");
    let last_part = input_line[(index + 1)..].trim();
    let vecstr = last_part._split(" ");
    let mut v = Vec::<String>::with_capacity(vecstr.len());
    for i in vecstr.iter() {
        v.push(String::from(*i));
    }
    return v;
}

/// parses a codename
fn parse_a_codename(input_line: &String) -> String {
    let index = input_line
        .find(':')
        .expect("A : went missing in parse_a_codename");
    let last_part = input_line[(index + 1)..].trim();
    return last_part.to_string();
}

/// parse a suite tag
fn parse_a_suite(input_line: &String) -> String {
    let index = input_line
        .find(':')
        .expect("A : went missing in parse_a_suite.");
    let last_part = input_line[(index + 1)..].trim();
    return last_part.to_string();
}

/// parses a date tag
fn parse_a_date(input_line: &String) -> DateTime<Utc> {
    let index = input_line.find(':').expect("could not find a :");
    let last_part = input_line[(index + 1)..].trim();
    let _date = DateTime::<Utc>::from_utc(
        NaiveDateTime::parse_from_str(last_part, "%a, %d %b %Y %T %Z").unwrap(),
        Utc,
    );
    return _date;
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

/// The default is uncompressed.
impl Default for Compression {
    fn default() -> Compression {
        return Compression::NONE;
    }
}

/// ReleaseData represents the control file that is used to define a repository
#[derive(Debug)]
struct ReleaseData {
    suite: String,
    codename: String,
    components: Vec<String>,
    architectures: Vec<String>,
    date: DateTime<Utc>,
    files: Option<Vec<HashedFile>>,
    acquire_by_hash: Option<bool>,
}

/// the stuff I manually implement for ReleaseData.
impl ReleaseData {
    /// Create a ReleaseData
    fn new() -> ReleaseData {
        return ReleaseData {
            suite: String::new(),
            codename: String::new(),
            components: vec![String::new()],
            architectures: vec![String::from("All"), String::from("Amd64")],
            date: Utc::now(),
            files: None,
            acquire_by_hash: None,
        };
    }
}

/// HashedFile represents the individual files linked to from the InRelease file.
/// This struct does not seperate between Packages, Sources, Contents, or Indices.
#[derive(Debug)]
struct HashedFile {
    compression: Compression,
    md5: Option<String>,
    name: String,
    sha1: Option<String>,
    sha256: Option<String>,
    sha512: Option<String>,
    size: usize,
}

/// Implemented stuff for HashedFile.
impl HashedFile {
    fn new(name: String, compression: Compression, size: usize) -> HashedFile {
        return HashedFile {
            compression: compression,
            md5: None,
            name: name,
            sha1: None,
            sha256: None,
            sha512: None,
            size: size,
        };
    }
}
