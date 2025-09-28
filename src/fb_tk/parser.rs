pub type Item = (String, String);

pub const EMPTY_ITEM: Item = (String::new(), String::new());

pub trait Parser {
    fn open(&self, filename: &str);
    fn close(&self) -> bool;
    fn eof(&self) -> bool;
    fn is_loaded(&self) -> bool;
    fn row(&self) -> isize;
    fn line(&self) -> String;
    fn next_item(&self) -> Item;
}
