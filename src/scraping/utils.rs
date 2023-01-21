pub fn _parse_document(text: String) -> scraper::Html {
    scraper::Html::parse_document(&text)
}

pub trait Select {
    fn select_first(&self, selection_string: &str) -> scraper::ElementRef;
    fn select_all(&self, selection_string: &str) -> Vec<scraper::ElementRef>;
    fn all_text(&self) -> Vec<String>;
}

impl Select for scraper::Html {
    fn select_first(&self, selection_string: &str) -> scraper::ElementRef {
        let selector = scraper::Selector::parse(selection_string).unwrap();
        self.select(&selector).next().unwrap()
    }

    fn select_all(&self, selection_string: &str) -> Vec<scraper::ElementRef> {
        let selector = scraper::Selector::parse(selection_string).unwrap();
        let mut vec: Vec<scraper::ElementRef> = vec![];
        for element in self.select(&selector) {
            vec.push(element);
        }
        vec
    }

    fn all_text(&self) -> Vec<String> {
        let mut vec = vec![];
        for text in self.root_element().text() {
            vec.push(text.to_string());
        }
        vec
    }
}

impl Select for scraper::ElementRef<'_> {
    fn select_first(&self, selection_string: &str) -> scraper::ElementRef {
        let selector = scraper::Selector::parse(selection_string).unwrap();
        self.select(&selector).next().unwrap()
    }

    fn select_all(&self, selection_string: &str) -> Vec<scraper::ElementRef> {
        let selector = scraper::Selector::parse(selection_string).unwrap();
        let mut vec: Vec<scraper::ElementRef> = vec![];
        for element in self.select(&selector) {
            vec.push(element);
        }
        vec
    }

    fn all_text(&self) -> Vec<String> {
        let mut vec = vec![];
        for text in self.text() {
            vec.push(text.to_string());
        }
        vec
    }
}

pub fn is_logged_in(document: &scraper::Html) -> bool {
    document.select_all("body.template-name-login").is_empty()
}
