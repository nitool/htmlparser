use std::fs::File;

use htmlparser::{HtmlParser, HtmlEvent};
use htmlparser::element::HtmlElementName;

#[test]
fn it_works() {
    let file = File::open("tests/htmls/complex.html").unwrap();
    let mut parser = HtmlParser::new(file);
    let mut collected_a = false;

    loop {
        let event = parser.next().unwrap();
        println!("{:#?}", event);

        match event {
            HtmlEvent::HtmlElementOpened { opened_element } => {
                if opened_element.name.is_element(HtmlElementName::A) {
                    println!("{:#?}", opened_element);
                    collected_a = true;
                }

                continue;
            }

            HtmlEvent::HtmlDocumentEnd => {
                assert!(collected_a);

                break;
            }

            _ => {
                continue;
            }
        }
    }
}

