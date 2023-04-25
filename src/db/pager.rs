use std::fs::{File, OpenOptions};
use std::path::Path;

use super::page::Page;
use super::{ROWS_PER_PAGE, TABLE_MAX_PAGES};

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
