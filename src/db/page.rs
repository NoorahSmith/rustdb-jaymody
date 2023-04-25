use super::{row::Row, ROWS_PER_PAGE};

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
