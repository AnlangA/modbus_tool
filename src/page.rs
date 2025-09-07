use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Slave,
    Master,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

#[derive(Debug)]
pub struct PageManager {
    current: Page,
    previous: Page,
}

impl Default for PageManager {
    fn default() -> Self {
        Self {
            current: Page::default(),
            previous: Page::default(),
        }
    }
}

impl PageManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_page(&mut self, page: Page) -> Option<Page> {
        if self.current != page {
            let old = self.current;
            self.previous = old;
            self.current = page;
            Some(old)
        } else {
            None
        }
    }

    pub fn current_page(&self) -> Page {
        self.current
    }

    pub fn previous_page(&self) -> Page {
        self.previous
    }

    pub fn has_changed(&self) -> bool {
        self.current != self.previous
    }
}
