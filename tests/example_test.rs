use std::fs::File;

use htmlparser::{HtmlParser, HtmlEvent};
use htmlparser::element::HtmlElementName;

#[test]
fn it_works() {
    let file = File::open("tests/htmls/entry.html").unwrap();
    let mut parser = HtmlParser::new(file);
    let mut counter = 0;
    loop {
        let event = parser.next().unwrap();
        counter += 1;
        println!("{:#?}", event);

        match event {
            HtmlEvent::HtmlElementOpened { opened_element } => {
                if counter == 1 {
                    assert_eq!(HtmlElementName::Div.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("class"), Some(&"test test_eq".to_string()));
                    assert_eq!(opened_element.attributes.get("data-value"), Some(&"test".to_string()));
                } else if counter == 2 {
                    assert_eq!(HtmlElementName::P.to_str(), opened_element.name.to_str());
                } else if counter == 3 {
                    assert_eq!(HtmlElementName::Span.to_str(), opened_element.name.to_str());
                }
            }

            HtmlEvent::HtmlElementClosed { closed_element } => {
                if counter == 7 {
                    assert_eq!(HtmlElementName::Div.to_str(), closed_element.name.to_str());
                } else if counter == 6 {
                    assert_eq!(HtmlElementName::P.to_str(), closed_element.name.to_str());
                } else if counter == 5 {
                    assert_eq!(HtmlElementName::Span.to_str(), closed_element.name.to_str());
                }
            }

            HtmlEvent::TextContent(content) => {
                assert_eq!(counter, 4);
                assert_eq!("text content", content);
            }

            HtmlEvent::HtmlDocumentEnd => {
                assert_eq!(counter, 8);

                break;
            }
        }
    }
}

