use std::str::Split;

type Items<'data> = Split<'data, char>;

pub struct Csv<'data> {
    items: Items<'data>,
}

impl<'data> Csv<'data> {
    fn new(items: Items<'data>) -> Csv<'data> {
        Csv { items }
    }
}

pub trait ToCsv<'data> {
    fn to_csv(&'data self) -> Csv<'data>;
}

impl<'data> ToCsv<'data> for &'data str {
    fn to_csv(&'data self) -> Csv<'data> {
        Csv::new(self.split(','))
    }
}

impl<'data> Iterator for Csv<'data> {
    type Item = &'data str;

    fn next(&mut self) -> Option<&'data str> {
        self.items.next().map(|item| item.trim())
    }
}
