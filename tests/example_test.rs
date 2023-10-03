use std::fs::File;

use htmlparser::{HtmlParser, HtmlEvent};
use htmlparser::element::HtmlElementName;

#[test]
fn it_works() {
    let file = File::open("tests/htmls/example.html").unwrap();
    let mut parser = HtmlParser::new(file);
    let mut counter = 0;

    loop {
        let event = parser.next().unwrap();
        counter += 1;
        println!("{:#?}", event);

        match event {
            HtmlEvent::HtmlElementOpened { opened_element } => {
                if counter == 1 {
                    assert_eq!(HtmlElementName::Doctype.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("html"), Some(&String::new()));
                } else if counter == 2 {
                    assert_eq!(HtmlElementName::Html.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("lang"), Some(&"en".to_string()));
                } else if counter == 3 {
                    assert_eq!(HtmlElementName::Head.to_str(), opened_element.name.to_str());
                } else if counter == 4 {
                    assert_eq!(HtmlElementName::Meta.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("charset"), Some(&"UTF-8".to_string()));
                } else if counter == 5 {
                    assert_eq!(HtmlElementName::Meta.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("name"), Some(&"viewport".to_string()));
                    assert_eq!(opened_element.attributes.get("content"), Some(&"width=device-width, initial-scale=1.0".to_string()));
                } else if counter == 6 {
                    assert_eq!(HtmlElementName::Meta.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("http-equiv"), Some(&"X-UA-Compatible".to_string()));
                    assert_eq!(opened_element.attributes.get("content"), Some(&"ie=edge".to_string()));
                } else if counter == 7 {
                    assert_eq!(HtmlElementName::Title.to_str(), opened_element.name.to_str());
                } else if counter == 10 {
                    assert_eq!(HtmlElementName::Link.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("rel"), Some(&"stylesheet".to_string()));
                    assert_eq!(opened_element.attributes.get("href"), Some(&"style.css".to_string()));
                } else if counter == 11 {
                    assert!(opened_element.name.is_element(HtmlElementName::Script));
                } else if counter == 15 {
                    assert_eq!(HtmlElementName::Body.to_str(), opened_element.name.to_str());
                } else if counter == 16 {
                    assert_eq!(HtmlElementName::Script.to_str(), opened_element.name.to_str());
                    assert_eq!(opened_element.attributes.get("src"), Some(&"index.js".to_string()));
                }
            }

            HtmlEvent::HtmlElementClosed { closed_element } => {
                if counter == 9 {
                    assert_eq!(HtmlElementName::Title.to_str(), closed_element.name.to_str());
                } else if counter == 13 {
                    assert_eq!(HtmlElementName::Script.to_str(), closed_element.name.to_str());
                } else if counter == 14 {
                    assert_eq!(HtmlElementName::Head.to_str(), closed_element.name.to_str());
                } else if counter == 17 {
                    assert_eq!(HtmlElementName::Script.to_str(), closed_element.name.to_str());
                } else if counter == 18 {
                    assert_eq!(HtmlElementName::Body.to_str(), closed_element.name.to_str());
                } else if counter == 19 {
                    assert_eq!(HtmlElementName::Html.to_str(), closed_element.name.to_str());
                }
            }

            HtmlEvent::TextContent(content) => {
                if counter == 8 {
                    assert_eq!("HTML 5 Boilerplate".to_string(), content);
                } else if counter == 12 {
                    assert_eq!("if (1 < 2) { console.log('test'); }".to_string(), content);
                }
            }

            HtmlEvent::HtmlDocumentEnd => {
                assert_eq!(counter, 20);

                break;
            }
        }
    }
}

