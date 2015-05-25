use std::iter;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Eq)]
pub struct CellBuffer {
    cols: usize,
    rows: usize,
    cells: Vec<Vec<Cell>>,
}

impl CellBuffer {
    pub fn new(cols: usize, rows: usize) -> CellBuffer {
        CellBuffer::with_cell(cols, rows, Cell::default())
    }

    pub fn with_char(cols: usize, rows: usize, ch: char) -> CellBuffer {
        CellBuffer::with_cell(cols, rows, Cell::with_char(ch))
    }

    pub fn with_styles(cols: usize, rows: usize, fg: Style, bg: Style) -> CellBuffer {
        CellBuffer::with_cell(cols, rows, Cell::with_styles(fg, bg))
    }

    pub fn with_cell(cols: usize, rows: usize, cell: Cell) -> CellBuffer {
        CellBuffer {
            cols: cols,
            rows: rows,
            cells: vec![vec![cell; rows]; cols],
        }
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn clear(&mut self) {
        self.clear_with_cell(Cell::default());
    }

    pub fn clear_with_char(&mut self, ch: char) {
        self.clear_with_cell(Cell::with_char(ch));
    }

    pub fn clear_with_styles(&mut self, fg: Style, bg: Style) {
        self.clear_with_cell(Cell::with_styles(fg, bg));
    }

    pub fn clear_with_cell(&mut self, blank: Cell) {
        for col in &mut self.cells {
            for cell in col.iter_mut() {
                cell.ch = blank.ch;
                cell.fg = blank.fg;
                cell.bg = blank.bg;
            }
        }
    }

    pub fn resize(&mut self, newcols: usize, newrows: usize, blank: Cell) {
        self.cells.resize(newcols, vec![blank; newrows]);
        for col in &mut self.cells {
            col.resize(newrows, blank);
        }
        self.cols = newcols;
        self.rows = newrows;
    }
}

impl Default for CellBuffer {
    fn default() -> CellBuffer {
        CellBuffer::with_cell(0, 0, Cell::default())
    }
}

impl Index<usize> for CellBuffer {
    type Output = Vec<Cell>;

    fn index(&self, index: usize) -> &Vec<Cell> {
        &self.cells[index]
    }
}

impl IndexMut<usize> for CellBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Cell> {
        &mut self.cells[index]
    }
}

/// A cell of a terminal display.
///
/// A `Cell` contains a character and a set of foreground and background `Style`s; it represents a
/// single point on a terminal display. A terminal is an array of cells, each having its own
/// character and styles.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    ch: char,
    fg: Style,
    bg: Style,
}

impl Cell {
    /// Creates a new `Cell` from the given `char` and styles.
    pub fn new(ch: char, fg: Style, bg: Style) -> Cell {
        Cell {
            ch: ch,
            fg: fg,
            bg: bg,
        }
    }

    /// Creates a new `Cell` from the given `char` and default styles.
    pub fn with_char(ch: char) -> Cell {
        Cell::new(ch, Style::default(), Style::default())
    }

    /// Creates a new `Cell` from the given styles and a blank `char`.
    pub fn with_styles(fg: Style, bg: Style) -> Cell {
        Cell::new(' ', fg, bg)
    }

    /// Returns the `Cell`'s character.
    pub fn ch(&self) -> char {
        self.ch
    }

    /// Sets the `Cell`'s character to the given `char`
    pub fn set_ch(&mut self, newch: char) -> &mut Cell {
        self.ch = newch;
        self
    }

    /// Returns the `Cell`'s foreground `Style`.
    pub fn fg(&self) -> Style {
        self.fg
    }

    pub fn fg_mut(&mut self) -> &mut Style {
        &mut self.fg
    }

    pub fn set_fg(&mut self, newfg: Style) -> &mut Cell {
        self.fg = newfg;
        self
    }

    pub fn bg(&self) -> Style {
        self.bg
    }

    pub fn bg_mut(&mut self) -> &mut Style {
        &mut self.bg
    }

    pub fn set_bg(&mut self, newbg: Style) -> &mut Cell {
        self.bg = newbg;
        self
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::new(' ', Style::default(), Style::default())
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Style(Color, Attr);

impl Style {
    pub fn new(color: Color, attr: Attr) -> Style {
        Style(color, attr)
    }

    pub fn with_color(c: Color) -> Style {
        Style::new(c, Attr::Default)
    }

    pub fn with_attr(a: Attr) -> Style {
        Style::new(Color::Default, a)
    }

    pub fn color(&self) -> Color {
        self.0
    }

    pub fn set_color(&mut self, newcolor: Color) -> &mut Style {
        self.0 = newcolor;
        self
    }

    pub fn attr(&self) -> Attr {
        self.1
    }

    pub fn set_attr(&mut self, newattr: Attr) -> &mut Style {
        self.1 = newattr;
        self
    }
}

impl Default for Style {
    fn default() -> Style {
        Style::new(Color::Default, Attr::Default)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u8),
    Default,
}

impl Color {
    pub fn as_byte(&self) -> u8 {
        match *self {
            Color::Black => { 0x00 },
            Color::Red => { 0x01 },
            Color::Green => { 0x02 },
            Color::Yellow => { 0x03 },
            Color::Blue => { 0x04 },
            Color::Magenta => { 0x05 },
            Color::Cyan => { 0x06 },
            Color::White => { 0x07 },
            Color::Byte(b) => { b },
            Color::Default => { panic!("Attempted to cast default color to u8") },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Attr {
    Default,
    Bold,
    Underline,
    Reverse,
}
