use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Page {
    pub total_item: i64,
    pub total_page: i64,
    pub page_size: i64,

    pub current: i64,
    #[serde(skip)]
    pub offset: i64,
}

#[derive(Debug)]
pub struct PageBuilder {
    total_item: i64,
    page_size: i64,
    current: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginateQuery {
    pub page: Option<i64>,
}

impl Page {
    pub const DEFAULT_PAGE_SIZE: i64 = 10;

    pub fn new(total_item: i64, page_size: i64, current: i64) -> Self {
        let total_page = total_item.div_ceil(page_size);
        let offset = (current - 1) * page_size;
        Self {
            total_item,
            total_page,
            page_size,
            current,
            offset,
        }
    }

    pub fn builder(total_item: i64, current: i64) -> PageBuilder {
        PageBuilder::new(total_item, current)
    }
}

impl PageBuilder {
    pub fn new(total_item: i64, current: i64) -> Self {
        Self {
            total_item,
            page_size: Page::DEFAULT_PAGE_SIZE,
            current,
        }
    }

    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = page_size;
        self
    }

    pub fn build(self) -> Page {
        Page::new(self.total_item, self.page_size, self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let page = Page::new(100, 10, 1);
        assert_eq!(page.total_page, 10);
        assert_eq!(page.offset, 0);
    }

    #[test]
    fn test_builder() {
        let page = Page::builder(81, 5).page_size(20).build();
        assert_eq!(page.total_page, 5);
        assert_eq!(page.offset, 80);
    }
}
