use std::{io::Write, path::Path};

use super::{pager::Pager, row::Row, ROWS_PER_PAGE, TABLE_MAX_ROWS};

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
