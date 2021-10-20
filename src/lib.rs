extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::md_reader::CodeReader;

pub mod parser;
pub mod md_reader;
pub mod md_writer;

#[derive(Error, Debug)]
pub enum WritingError {
    #[error("io error: `{0}` ")]
    IOError(String),
    #[error("read file error: `{0}` ")]
    ReadFileError(String),
    #[error("unknown data store error")]
    Unknown,
}

pub struct Writing {}

impl Writing {
    pub fn process_file<P: AsRef<Path>>(path: P) -> Result<String, WritingError> {
        let path = path.as_ref().to_path_buf();

        if let Err(err) = Writing::pre_process_file(&path) {
            return Err(err);
        };

        let result = Writing::write_it(path).join("\n");

        Ok(result)
    }

    fn write_it(path: PathBuf) -> Vec<String> {
        let file = File::open(path).expect("cannot open file");
        let reader = BufReader::new(file);

        let mut results =  vec![] ;
        for res in reader.lines() {
            let line = res.expect("cannot parse line");
            if line.starts_with("// doc-") {
                let writing = parser::parse(line.replace("//", "").as_str());

                if writing.code_docs.len() > 0 {
                    results.append(&mut CodeReader::read_doc_code(&writing.code_docs[0]));
                } else if writing.code_sections.len() > 0 {
                    results.append(&mut CodeReader::read_doc_section(&writing.code_sections[0]));
                } else {
                    results.push(String::from(line));
                };
            } else {
                results.push(String::from(line));
            }
        }

        results
    }

    fn pre_process_file(path: &PathBuf) -> Result<(), WritingError> {
        if path.is_dir() {
            return Err(WritingError::IOError(format!("path: {:?} is a dir", path)));
        }

        if let Err(e) = fs::read(path) {
            return Err(WritingError::IOError(format!("read file error: {:?}", e)));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use pulldown_cmark::{Options, Parser};

    use super::*;

    // doc-start: section1
    #[test]
    fn should_parse_line() {
        let string = String::from_utf8_lossy(&fs::read("README.md").unwrap()).to_string();

        let parser = Parser::new_ext(&*string, Options::empty());

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"\nHTML output:\n").unwrap();
        md_writer::write_text(&mut handle, parser).unwrap();
    }
    // doc-end: section1
}
