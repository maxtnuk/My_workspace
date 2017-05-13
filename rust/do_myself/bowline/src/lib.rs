extern crate termion;
#[macro_use]
extern crate log;

use std::u8;
use std::cmp::max;
use std::{convert, fmt, iter};

use log::LogLevel;
use termion::color::{self, LightBlack, Reset, Fg};

pub mod error;
#[derive(Clone, Debug, Eq, PartialEq)]
/// Representation of a set of data `Point` values
struct Line<V> {
    got_data: bool,
    name: String,
    started: bool,
    values: Vec<V>,
}
impl<V> Line<V> {
    /// Creates a new `Line`
    fn new(name: &str) -> Self {
        Line {
            got_data: false,
            name: name.to_owned(),
            started: false,
            values: vec![],
        }
    }

    /// Adds a value to a line
    fn add_value(&mut self, value: V) {
        self.values.push(value);
        self.got_data = true;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Specifies if a column can be used or not
enum Column<V> {
    /// Column free for usage
    Free,

    /// Column already in use
    Used(Line<V>),
}

pub struct Graph<V> {
    lines_to_be_removed: Vec<String>,
    columns: Vec<Column<V>>,
    prefix_len: usize,
}
impl<V> Graph<V>
    where V: Clone + Default + Ord + PartialEq + fmt::Debug,
          f64: convert::From<V>
{
    pub fn new(length: usize) -> Self {
        Graph {
            lines_to_be_removed: vec![],
            columns: vec![],
            prefix_len: length + 3,
        }
    }
    pub fn draw(&mut self) {
        fn fillchar() -> String {
            format!("{}┈{}", Fg(LightBlack), Fg(Reset))
        }
        let start_ch = "┬";
        let line_chr = "│";
        let nodata_c = "╎";
        let end_char = "┴";
        let col_width = 2;
        let (width, _) = termion::terminal_size().expect("error fo terminal size");

        let mut cursor = self.prefix_len as u16;
        let end_cursor = if width % 2 == 0 { width - 2 } else { width - 1 };


    }
    pub fn add(&mut self) {}
    pub fn remnove(&mut self) {}
}
