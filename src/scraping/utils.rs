pub fn _parse_document(text: String) -> scraper::Html {
    scraper::Html::parse_document(&text).to_owned()
}

pub trait Select {
    fn select_first(&self, selection_string: &str) -> scraper::ElementRef;
    fn select_all(&self, selection_string: &str) -> Vec<scraper::ElementRef>;
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
        return vec;
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
        return vec;
    }
}

pub fn is_logged_in(document: &scraper::Html) -> bool {
    document.select_all("body.template-name-login").len() == 0
}
