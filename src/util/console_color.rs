use std::fmt::{self, Display};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum Color {
    /// No color has been set. Nothing is changed when applied.
    Unset,

    /// Black #0 (foreground code `30`, background code `40`).
    Black,

    /// Red: #1 (foreground code `31`, background code `41`).
    Red,

    /// Green: #2 (foreground code `32`, background code `42`).
    Green,

    /// Yellow: #3 (foreground code `33`, background code `43`).
    Yellow,

    /// Blue: #4 (foreground code `34`, background code `44`).
    Blue,

    /// Purple: #5 (foreground code `35`, background code `45`).
    Purple,

    /// Cyan: #6 (foreground code `36`, background code `46`).
    Cyan,

    /// White: #7 (foreground code `37`, background code `47`).
    White
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::Unset => Ok(()),
            Color::Black => write!(f, "0"),
            Color::Red => write!(f, "1"),
            Color::Green => write!(f, "2"),
            Color::Yellow => write!(f, "3"),
            Color::Blue => write!(f, "4"),
            Color::Purple => write!(f, "5"),
            Color::Cyan => write!(f, "6"),
            Color::White => write!(f, "7"),
        }
    }
}

impl Default for Color {
    #[inline(always)]
    fn default() -> Self { Color::Unset }
}


pub struct Print<T> {
    item: T,
    foreground: Color,
    background: Color
}

macro_rules! construct {
    ($name:ident, $color:ident) => (
        pub fn $name(item: T) -> Print<T> {
            Print {
                item: item,
                foreground: Color::$color,
                background: Color::default(),
            }
        }
    )
}

impl<T> Print<T>
    where T: Display
{
    pub fn new(item: T) -> Print<T> {
        Print {
            item: item,
            foreground: Color::default(),
            background: Color::default(),
        }
    }

    construct!(unset, Unset);
    construct!(black, Black);
    construct!(red, Red);
    construct!(green, Green);
    construct!(yellow, Yellow);
    construct!(blue, Blue);
    construct!(purple, Purple);
    construct!(cyan, Cyan);
    construct!(white, White);

    pub fn foreground(&mut self, color: Color) -> &Print<T> {
        self.foreground = color;
        self
    }

    pub fn background(mut self, color: Color) -> Print<T> {
        self.background = color;
        self
    }
}

impl<T: Display> fmt::Display for Print<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[")?;

        if self.background != Color::Unset {
            write!(f, "4")?;
            self.background.fmt(f)?;
            write!(f, ";")?;
        }

        if self.foreground != Color::Unset {
            write!(f, "3")?;
            self.foreground.fmt(f)?;
            write!(f, ";1")?;
        }

        write!(f, "m")?;

        self.item.fmt(f)?;

        write!(f, "\x1B[0m")
    }
}

impl<T: Display> fmt::Debug for Print<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[")?;

        if self.background != Color::Unset {
            write!(f, "4")?;
            self.background.fmt(f)?;
        }

        if self.foreground != Color::Unset {
            write!(f, "3")?;
            self.foreground.fmt(f)?;
        }

        write!(f, "m")?;

        self.item.fmt(f)?;

        write!(f, "\x1B[0m")
    }
}
