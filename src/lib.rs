use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Website {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebsiteList {
    list: Vec<Website>,
}

impl Website {
    pub fn new(url: String) -> Self {
        Self {
            url,
        }
    }

    /*
    pub fn is_valid(website: Website) -> bool {

    }
    */
}

impl WebsiteList {
    pub fn new(list: Vec<Website>) -> Self {
        Self {
            list: list,
        }
    }

    pub fn add(&mut self, website: Website) {
        for i in self.list.iter() {
            if i.url == website.url {
                return;
            }
        }
        self.list.push(website);
    }

    // deal with errors in case the list is empty
    pub fn get(&mut self) -> String {
        let website = self.list[0].url.clone();
        self.list.remove(0);
        website
    }
}
