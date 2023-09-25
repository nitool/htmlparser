use std::io::Read;
use std::collections::HashMap;
use crate::element::{HtmlElement, HtmlElementName};

mod element;

#[derive(Debug)]
pub enum HtmlEvent {
    HtmlElementOpened { opened_element: element::HtmlElement },
    HtmlElementClosed { closed_element: element::HtmlElement },
    TextContent(String),
    HtmlDocumentEnd,
}

pub struct HtmlParserContext {
    current_element: Option<HtmlElementName>,
    inside_brackets: bool,
    is_closing_element: bool,
    current_attribute: Option<String>,
    defined_attributes: HashMap<String, String>,
    text_content: String,
    skip_content_fillup: bool
}

pub struct HtmlParser<R> {
    source: R,
    context: HtmlParserContext
}

impl<R:Read> HtmlParser<R> {
    pub fn new (source: R) -> HtmlParser<R> {
        let context = HtmlParserContext {
            current_element: None,
            inside_brackets: false,
            is_closing_element: false,
            current_attribute: None,
            defined_attributes: HashMap::new(),
            text_content: String::new(),
            skip_content_fillup: false
        };

        return HtmlParser { source, context };
    }

    fn fill_element_from_text_content(&mut self) -> Option<HtmlElementName> {
        if self.context.inside_brackets && self.context.current_element.is_none() {
            let element_name = HtmlElementName::from_str(self.context.text_content.as_str());
            
            if element_name.is_ok() {
                self.context.current_element = Some(element_name.unwrap());
                self.context.text_content = String::new();
                let name = self.context.current_element.as_ref().unwrap();
                let current_name = HtmlElementName::from_str(name.to_str()).unwrap();

                return Some(current_name);
            }
            
            return None;
        }

        if self.context.current_element.is_none() {
            return None;
        }

        let name = self.context.current_element.as_ref().unwrap();
        let current_name = HtmlElementName::from_str(name.to_str()).unwrap();

        return Some(current_name);
    }

    fn handle_whitespace(&mut self) -> Option<HtmlEvent> {
        if self.context.text_content.is_empty() {
            return None;
        }

        let element_filled = self.fill_element_from_text_content();
        if element_filled.is_none() {
            return None;
        }

        if self.context.inside_brackets && self.context.current_element.is_some() {

        }
        
        return None;
    }

    fn handle_closing_bracket(&mut self) -> Option<HtmlEvent> {
        let element_filled = self.fill_element_from_text_content();
        if element_filled.is_none() {
            self.context.text_content = String::new();

            return None;
        }

        let name = self.context.current_element.as_ref().unwrap();
        let current_name = HtmlElementName::from_str(name.to_str()).unwrap();
        let element = HtmlElement {
            name: current_name,
            attributes: self.context.defined_attributes.clone()
        };

        let event: HtmlEvent;
        if self.context.is_closing_element {
            event = HtmlEvent::HtmlElementClosed { closed_element: element }
        } else {
            event = HtmlEvent::HtmlElementOpened { opened_element: element }
        }

        self.context.inside_brackets = false;
        self.context.is_closing_element = false;
        self.context.current_element = None;
        self.context.current_attribute = None;
        self.context.defined_attributes = HashMap::new();
        self.context.text_content = String::new();
        
        return Some(event);
    }

    fn handle_opening_bracket(&mut self) -> Option<HtmlEvent> {
        if self.context.inside_brackets {
            return None;
        }

        let mut event: Option<HtmlEvent> = None;
        let content = self.context.text_content.trim().to_string();
        if !content.is_empty() {
            event = Some(HtmlEvent::TextContent(content));
        }

        self.context.skip_content_fillup = true;
        self.context.text_content = String::new();
        self.context.inside_brackets = true;

        return event;
    }

    pub fn next(&mut self) -> Result<HtmlEvent, &'static str> {
        loop {
            let mut buffer = [0; 1];
            let read_result = self.source.read(&mut buffer).unwrap();
            if read_result == 0 {
                return Ok(HtmlEvent::HtmlDocumentEnd);
            }

            let read_bytes = String::from_utf8(Vec::from(buffer)).unwrap();
            let mut event: Option<HtmlEvent>;
            for sign in read_bytes.split("").into_iter() {
                event = None;
                self.context.skip_content_fillup = false;

                if sign == "/" && self.context.inside_brackets {
                    self.context.is_closing_element = true;
                    continue;
                }

                if sign == "<" {
                    event = self.handle_opening_bracket();
                }

                if sign == ">" && self.context.inside_brackets {
                    event = self.handle_closing_bracket();
                }

                if sign == " " {
                    event = self.handle_whitespace();
                }

                if event.is_some() {
                    let value = event.unwrap();

                    return Ok(value);
                }

                if !self.context.skip_content_fillup {
                    self.context.text_content.push_str(sign);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{HtmlParser, HtmlEvent};
    use crate::element::HtmlElementName;

    #[test]
    fn it_works() {
        let file = File::open("test/entry.html").unwrap();
        let mut parser = HtmlParser::new(file);
        let mut counter = 0;
        loop {
            let event = parser.next().unwrap();
            counter += 1;
            println!("{:#?}", event);

            match event {
                HtmlEvent::HtmlElementOpened { opened_element } => {
                    if counter == 1 {
                        assert!(HtmlElementName::Div.to_str() == opened_element.name.to_str());
                    } else if counter == 2 {
                        assert!(HtmlElementName::P.to_str() == opened_element.name.to_str());
                    } else if counter == 3 {
                        assert!(HtmlElementName::Span.to_str() == opened_element.name.to_str());
                    }
                }

                HtmlEvent::HtmlElementClosed { closed_element } => {
                    if counter == 7 {
                        assert!(HtmlElementName::Div.to_str() == closed_element.name.to_str());
                    } else if counter == 6 {
                        assert!(HtmlElementName::P.to_str() == closed_element.name.to_str());
                    } else if counter == 5 {
                        assert!(HtmlElementName::Span.to_str() == closed_element.name.to_str());
                    }
                }

                HtmlEvent::TextContent(content) => {
                    assert!(counter == 4);
                    assert!("text content" == content);
                }

                HtmlEvent::HtmlDocumentEnd => {
                    assert!(counter == 8);

                    break;
                }
            }
        }
    }
}
