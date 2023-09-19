use std::io::Read;
use std::collections::HashMap;

mod element;

pub enum HtmlEvent {
    HtmlElementOpened { opened_element: element::HtmlElement },
    HtmlElementClosed { closed_element: element::HtmlElement },
    TextContent(String),
    HtmlDocumentEnd,
}

pub struct HtmlParserContext {
    opened_elements: Vec<element::HtmlElementName>,
    inside_brackets: bool,
    is_closing_element: bool,
    current_attribute: String,
    defined_attributes: HashMap<String, String>
}

pub struct HtmlParser<R> {
    source: R,
    context: HtmlParserContext
}

impl<R:Read> HtmlParser<R> {
    pub fn new (source: R) -> HtmlParser<R> {
        let context = HtmlParserContext {
            opened_elements: vec![],
            inside_brackets: false,
            is_closing_element: false,
            current_attribute: String::new(),
            defined_attributes: HashMap::new()
        };

        return HtmlParser { source, context };
    }

    pub fn next (&mut self) -> Result<HtmlEvent, &'static str> {
        loop {
            let mut buffer = [0; 10];
            let read_result = self.source.read(&mut buffer).unwrap();
            if read_result == 0 {
                return Ok(HtmlEvent::HtmlDocumentEnd);
            }

            let read_bytes = String::from_utf8(Vec::from(buffer)).unwrap();
            println!("{}", read_bytes);
            // for sign in read_bytes.split("").into_iter() {
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::HtmlParser;

    #[test]
    fn it_works() {
        println!("test kurwa");
        let file = File::open("test/entry.html").unwrap();
        let mut parser = HtmlParser::new(file);
        let _event = parser.next().unwrap();
        assert!(true);
    }
}
