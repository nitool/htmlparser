use std::io::Read;
use std::collections::{HashMap, VecDeque};
use crate::element::{HtmlElement, HtmlElementName};

pub mod element;

#[derive(Debug)]
pub enum HtmlEvent {
    HtmlElementOpened { opened_element: element::HtmlElement },
    HtmlElementClosed { closed_element: element::HtmlElement },
    TextContent(String),
    HtmlDocumentEnd,
}

pub struct HtmlParserContext {
    current_element: Option<HtmlElementName>,
    elements: Vec<element::HtmlElement>,
    inside_brackets: bool,
    is_closing_element: bool,
    defined_attributes: HashMap<String, String>,
    text_content: String,
    skip_content_fillup: bool,
    buffer_vec: Vec<u8>,
    events: VecDeque<HtmlEvent>
}

pub struct HtmlParser<R> {
    source: R,
    context: HtmlParserContext
}

impl<R:Read> HtmlParser<R> {
    pub fn new (source: R) -> HtmlParser<R> {
        let context = HtmlParserContext {
            current_element: None,
            elements: vec![],
            inside_brackets: false,
            is_closing_element: false,
            defined_attributes: HashMap::new(),
            text_content: String::new(),
            skip_content_fillup: false,
            buffer_vec: vec![],
            events: VecDeque::new()
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

    fn fill_attritube(&mut self) -> () {
        if !self.context.inside_brackets 
            || self.context.current_element.is_none() 
            || self.context.text_content.is_empty() 
        {
            return;
        }

        let characters_count: Vec<&str> = self.context.text_content.matches('"').collect();
        if (self.context.text_content.contains('"') && characters_count.len() == 2) 
            || characters_count.len() == 0
        {
            let split: Vec<&str> = self.context.text_content.split('=').collect();
            let attr_name = split[0].trim().to_string();
            let mut value = split[1..].join("=");
            if value.starts_with('"') {
                value.remove(0);
            }

            if value.ends_with('"') {
                value.remove(value.len() - 1);
            }

            self.context.defined_attributes.insert(attr_name, value);
            self.context.text_content = String::new();
        }

        return;
    }

    fn handle_whitespace(&mut self) -> Option<HtmlEvent> {
        if self.context.text_content.is_empty() {
            return None;
        }

        let element_filled = self.fill_element_from_text_content();
        if element_filled.is_none() {
            return None;
        }

        self.fill_attritube();
        
        return None;
    }

    fn handle_closing_bracket(&mut self) -> Option<HtmlEvent> {
        let element_filled = self.fill_element_from_text_content();
        if element_filled.is_none() {
            self.context.text_content = String::new();
            self.context.skip_content_fillup = true;

            return None;
        }

        self.fill_attritube();

        let element = HtmlElement {
            name: self.context.current_element.clone().unwrap(),
            attributes: self.context.defined_attributes.clone()
        };

        let event: HtmlEvent;
        if self.context.is_closing_element {
            event = HtmlEvent::HtmlElementClosed { closed_element: element.clone() };
            self.context.elements.pop();
        } else {
            event = HtmlEvent::HtmlElementOpened { opened_element: element.clone() };
            self.context.elements.push(element.clone());
        }

        self.context.inside_brackets = false;
        self.context.is_closing_element = false;
        self.context.current_element = None;
        self.context.defined_attributes = HashMap::new();
        self.context.text_content = String::new();
        self.context.skip_content_fillup = true;
        
        return Some(event);
    }

    fn handle_closing_element(&mut self) -> Option<HtmlEvent> {
        let last_element = self.context.elements.last();
        let mut event: Option<HtmlEvent> = None;
        if last_element.is_some() 
            && (
                last_element.unwrap().name.is_element(HtmlElementName::Script)
                || last_element.unwrap().name.is_element(HtmlElementName::Style)
            ) && self.context.text_content.trim().ends_with("<")
        {
            let mut content = self.context.text_content.trim().to_string();
            content = content.as_str()[0..content.len() - 1].to_string();
            if !content.is_empty() {
                event = Some(HtmlEvent::TextContent(content));
            }

            self.context.skip_content_fillup = true;
            self.context.text_content = String::new();
            self.context.inside_brackets = true;
            self.context.is_closing_element = true;
        } else if self.context.inside_brackets 
            && self.context.text_content.clone().trim().is_empty() 
        {
            self.context.is_closing_element = true;
            self.context.skip_content_fillup = true;
        }

        return event;
    }

    fn handle_opening_bracket(&mut self) -> Option<HtmlEvent> {
        if self.context.inside_brackets {
            return None;
        }

        let last_element = self.context.elements.last();
        if last_element.is_some() 
            && (
                last_element.unwrap().name.is_element(HtmlElementName::Script)
                || last_element.unwrap().name.is_element(HtmlElementName::Style)
            )
        {
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
        if self.context.events.len() > 0 {
            let event = self.context.events.pop_front().unwrap();
            
            return Ok(event);
        }

        loop {
            let mut buffer = [0; 4];
            let read_result = self.source.read(&mut buffer).unwrap();
            if read_result == 0 {
                return Ok(HtmlEvent::HtmlDocumentEnd);
            }

            if self.context.buffer_vec.is_empty() {
                self.context.buffer_vec = Vec::from(buffer);
            } else {
                let mut temporary_vec = Vec::from(buffer);
                self.context.buffer_vec.append(&mut temporary_vec);
            }

            let read_bytes_result = String::from_utf8(self.context.buffer_vec.clone());
            let read_bytes: String;
            match read_bytes_result {
                Ok(element) => {
                    read_bytes = element.clone();
                    self.context.buffer_vec = vec![];
                }

                Err(_error) => {
                    continue;
                }
            }

            let mut event: Option<HtmlEvent>;
            for sign_im in read_bytes.split("").into_iter() {
                let mut sign = sign_im.clone();
                event = None;
                self.context.skip_content_fillup = false;
                let char = sign.chars().next();
                if sign == "/" {
                    event = self.handle_closing_element();
                } else if sign == "<" {
                    event = self.handle_opening_bracket();
                } else if sign == ">" && self.context.inside_brackets {
                    event = self.handle_closing_bracket();
                } else if char.is_some() && char.unwrap().is_whitespace() {
                    event = self.handle_whitespace();
                    sign = " ";
                }

                if event.is_some() {
                    self.context.events.push_back(event.unwrap());

                    continue;
                }

                if !self.context.skip_content_fillup {
                    self.context.text_content.push_str(sign);
                }
            }

            if self.context.events.len() > 0 {
                let event = self.context.events.pop_front().unwrap();
                
                return Ok(event);
            }
        }
    }
}

