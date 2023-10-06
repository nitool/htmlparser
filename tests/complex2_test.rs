use std::fs::File;

use htmlparser::{HtmlParser, HtmlEvent};
use htmlparser::element::HtmlElementName;

#[test]
fn it_works() {
    let file = File::open("tests/htmls/complex2.html").unwrap();
    let mut parser = HtmlParser::new(file);
    let mut image_count = 0;

    loop {
        let event = parser.next().unwrap();
        println!("{:#?}", event);
        match event {
            HtmlEvent::HtmlElementOpened { opened_element } => {
                if opened_element.name.is_element(HtmlElementName::Img) {
                    image_count += 1;
                    continue;
                }
            }

            HtmlEvent::HtmlDocumentEnd => {
                assert_eq!(image_count, 123);

                break;
            }

            _ => {
                continue;
            }
        }
    }
}

