use std::fs::File;

use htmlparser::{HtmlParser, HtmlEvent};
use htmlparser::element::HtmlElementName;

#[test]
fn it_works() {
    let file = File::open("tests/htmls/complex.html").unwrap();
    let mut parser = HtmlParser::new(file);
    let mut collected_as = vec![
        "/",
        "javascript: void(1);",
        "https://blog.epicdrama.pl/",
        "/watch-now/",
        "javascript: void(1);",
        "https://www.facebook.com/EpicDramaPL/",
        "javascript: void(0);",
        "#carousel-header",
        "javascript: void(1)",
        "/watch-now/",
        "/watch-now/Dalgliesh/2/1",
        "/",
        "/bundles/app/files/new/polityka-prywatnosci.pdf?1695806565",
        "javascript: void(1)",
        "/watch-now/",
        "http://viasatexplore.pl/#tvschedule",
        "http://viasatnature.pl/#tvschedule",
        "http://viasathistory.pl/#tvschedule",
    ];

    loop {
        let event = parser.next().unwrap();
        // println!("{:#?}", event);

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
                println!("{:#?}", collected_as);
                assert!(collected_as.is_empty());

                break;
            }

            _ => {
                continue;
            }
        }
    }
}

