use std::fs::File;

use htmlparser::{HtmlParser, HtmlEvent};
use htmlparser::element::HtmlElementName;

#[test]
fn it_works() {
    let file = File::open("tests/htmls/complex.html").unwrap();
    let mut parser = HtmlParser::new(file);
    let mut collected_as = vec![
        "/",
        "/watch-now/",
        "https://www.facebook.com/EpicDramaPL/",
    ];

    loop {
        let event = parser.next().unwrap();
        println!("{:#?}", event);

        match event {
            HtmlEvent::HtmlElementOpened { opened_element } => {
                if !opened_element.name.is_element(HtmlElementName::A) {
                    continue;
                }

                if !opened_element.attributes.contains_key("href") {
                    continue;
                }

                let href = opened_element.attributes.get("href").unwrap();
                if collected_as.contains(&href.as_str()) {
                    let index = collected_as.iter().position(|x| *x == href).unwrap();
                    collected_as.remove(index);
                }

                continue;
            }

            HtmlEvent::HtmlDocumentEnd => {
                assert!(collected_as.is_empty());

                break;
            }

            _ => {
                continue;
            }
        }
    }
}

