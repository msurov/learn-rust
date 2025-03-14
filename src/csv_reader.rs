#![allow(unused)]
use std::{fs::File, io::{BufRead, BufReader}};

pub struct CSVReader {
  buf_reader : BufReader<File>,
  temp_buf : String,
  cur_line : Vec<f64>,
  cur_line_num : u32,
  status : String,
  header : String
}

impl CSVReader {

  pub fn new(filepath : &str) -> CSVReader {
    let f = File::open(filepath).expect("Can't read the file");
    let buf_reader : BufReader<File> = std::io::BufReader::new(f);
    let temp_buf = String::with_capacity(1024);
    let cur_line = Vec::<f64>::with_capacity(100);
    CSVReader {
      buf_reader,
      temp_buf,
      cur_line,
      cur_line_num : 0,
      status : "".to_owned(),
      header : "".to_owned()
    }
  }

  pub fn take_header(&mut self) -> Option<&str> {
    self.temp_buf.clear();
    match self.buf_reader.read_line(&mut self.temp_buf) {
      Err(e) => {
        self.status = format!("Can't read next line, because of error {}", e);
        None
      }
      Ok(0) => {
        self.status = "Nothing to read, EOF reached".to_owned();
        None
      }
      _ => {
        self.temp_buf.clone_into(&mut self.header);
        self.cur_line_num += 1;
        Some(&self.header)
      }
    }
  }

  pub fn take(&mut self) -> Option<&Vec<f64>> {
    self.temp_buf.clear();
    match self.buf_reader.read_line(&mut self.temp_buf) {
      Err(e) => {
        self.status = format!("Can't read next line, because of error {}", e);
        None
      }
      Ok(0) => {
        self.status = "Nothing to read, EOF reached".to_owned();
        None
      }
      _ => {
        self.parse_line();
        self.cur_line_num += 1;
        Some(&self.cur_line)
      }
    }
  }

  fn parse_line(&mut self) -> bool {
    self.cur_line.clear();

    for (pos, entry) in self.temp_buf.split(",").enumerate() {
      if let Ok(value) = entry.trim().parse::<f64>() {
        self.cur_line.push(value);
      } else {
        self.status = format!("can't process line {} at position {}", self.cur_line_num, pos);
        return false;
      }
    };
    true
  }

}
