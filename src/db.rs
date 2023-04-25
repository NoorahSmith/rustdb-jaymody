use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    usize,
};

use crate::fstring::FString;

pub const COLUMN_USERNAME_SIZE: usize = 32;
pub const COLUMN_EMAIL_SIZE: usize = 255;

const ROW_SIZE: usize = std::mem::size_of::<Row>();
const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_PAGES: usize = 100;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Row {
    pub id: u32,
    pub username: FString<COLUMN_USERNAME_SIZE>,
    pub email: FString<COLUMN_EMAIL_SIZE>,
}

impl std::str::FromStr for Row {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = s.split_ascii_whitespace();

        Ok(Row {
            id: a
                .next()
                .ok_or("id not found")?
                .parse::<u32>()
                .or(Err("could not parse id to uint"))?,
            username: FString::from_str(a.next().ok_or("username not found")?)?,
            email: FString::from_str(a.next().ok_or("email not found")?)?,
        })
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}

pub struct Page {
    pub rows: [Option<Row>; ROWS_PER_PAGE],
}

impl Page {
    pub fn new() -> Page {
        const ROW_INIT: Option<Row> = None;
        Page {
            rows: [ROW_INIT; ROWS_PER_PAGE],
        }
    }
}

pub struct Pager {
    pub file: File,
    pub pages: [Option<Page>; TABLE_MAX_PAGES],
}

impl Pager {
    pub fn open(path: &Path) -> Pager {
        const PAGE_INIT: Option<Page> = None;
        Pager {
            file: OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
            pages: [PAGE_INIT; TABLE_MAX_PAGES],
        }
    }

    pub fn get_page_by_page_num(&mut self, page_num: usize) -> Result<&mut Page, String> {
        if page_num > TABLE_MAX_PAGES {
            return Err(format!(
                "Tried to fetch page number out of bounds: {} > {}",
                page_num, TABLE_MAX_PAGES
            ));
        }

        if (self.pages[page_num]).is_none() {
            let page = Page::new();
            self.pages[page_num] = Some(page);
        }

        Ok(self.pages[page_num].as_mut().unwrap())
    }

    pub fn get_page_by_row_num(&mut self, row_num: usize) -> Result<&mut Page, String> {
        let page_num = row_num / ROWS_PER_PAGE;
        self.get_page_by_page_num(page_num)
    }
}

pub struct Table {
    pub nrows: usize,
    pub pager: Pager,
}

impl Table {
    pub fn open(path: &Path) -> Table {
        Table {
            nrows: 0,
            pager: Pager::open(path),
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        let s = self.select();
        self.pager
            .file
            .write_all(s.as_bytes())
            .or(Err("Could not write to file.".to_owned()))
    }

    pub fn insert_row(&mut self, row: Row) -> Result<(), String> {
        if self.nrows >= TABLE_MAX_ROWS {
            return Err("Table full.".to_owned());
        }

        let page = self.pager.get_page_by_row_num(self.nrows)?;
        page.rows[self.nrows % ROWS_PER_PAGE] = Some(row);
        self.nrows += 1;
        Ok(())
    }

    pub fn select(&mut self) -> String {
        let mut output = String::new();

        for i in 0..self.nrows {
            let page = self.pager.get_page_by_row_num(i).unwrap();
            let row = page.rows[i % ROWS_PER_PAGE].as_ref().unwrap();
            output.push_str(row.to_string().as_str());
            output.push_str("\n")
        }

        output
    }
}
