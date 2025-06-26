use mupdf::{TextPageOptions};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Fragment {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub font_name: Option<String>,
    pub font_size: f64,
    pub bbox_width: f64,
    pub bbox_height: f64

}
#[derive(Default, Debug, Clone)]
pub struct Line {
    pub text_fragments: Vec<Fragment>

}
#[derive(Default, Debug, Clone)]
pub struct Page {
    pub lines: Vec<Line>
}
#[derive(Default, Debug, Clone)]
pub struct Doc {
    pub pages: Vec<Page>
}

pub fn get_structured_document_from_filepath(path: String)-> Result<Doc, Box<dyn std::error::Error>> {
    let filename: String = path;
    let document = mupdf::Document::open(&filename)?;
    let mut new_document: Doc = Doc::default();

    for page in document.pages()? {
        let pageref = page.as_ref();
        let pagebounds = &pageref.unwrap().bounds()?;
        let text_page = &page?.to_text_page(TextPageOptions::empty())?;
        let text_blocks = text_page.blocks();

        let mut raw_page_fragments = Vec::<Fragment>::default();

        for block in text_blocks {
            
            let lines = block.lines();
            for line in lines {
                let chars = line.chars();
                let mut current_working_fragment= Fragment::default();

                let mut working_frag_reset = false;
                let mut first_char = true;
                for char in chars {

                    if working_frag_reset {
                        first_char = true;
                        working_frag_reset = false;
                    }

                    if first_char {
                        current_working_fragment.x = char.quad().ll.x as f64;
                        current_working_fragment.y = pagebounds.height() as f64 - char.quad().ll.y as f64 ;

                        current_working_fragment.font_size = char.size() as f64;
                        current_working_fragment.bbox_width = (char.quad().lr.x - char.quad().ll.x) as f64;
                        current_working_fragment.bbox_height = (char.quad().ur.y - char.quad().lr.y) as f64;
                        
                        first_char = false;
                    }
                    if let Some(ch) = char.char() {
                        if ch.is_whitespace() {
                            raw_page_fragments.push(current_working_fragment);
                            current_working_fragment = Fragment::default();
                            working_frag_reset = true;
                        }
                        current_working_fragment.text.push(ch);
                    }
                    
                }
                if current_working_fragment != Fragment::default() {
                    raw_page_fragments.push(current_working_fragment);
                }
            }
        }

        let mut new_page = Page::default();

        let mut current_working_newline = Line::default();
        let mut working_newline_reset = false;
        let mut last_y:f64 = 0.0;
        if let Some(first_fr) = raw_page_fragments.first() {
            last_y = first_fr.y.clone();
        }

        for fr in &raw_page_fragments {
            if working_newline_reset {
                working_newline_reset = false;
            }
            if fr.y != last_y {
                new_page.lines.push(current_working_newline.clone());
                current_working_newline = Line::default();
                working_newline_reset = true;
            }
            current_working_newline.text_fragments.push(fr.clone());
            last_y = fr.y;
        }
        if working_newline_reset {
            new_page.lines.push(current_working_newline);
        }

        for line in &mut new_page.lines {
            line.text_fragments.sort_by(|a, b| a.x.total_cmp(&b.x));
        }

        new_document.pages.push(new_page);
    }

    Ok(new_document)
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn it_works() {
        let structured_doc_extracted =
        get_structured_document_from_filepath("test_pdfs/DraftTest_02.pdf".into());
        
        if let Ok(doc) = structured_doc_extracted {
            for page in doc.pages {
                println!("PAGE -----");
                for line in page.lines {
                    print!("     LINE: ----- ");
                    if let Some(first_frag) = line.text_fragments.first() {
                        print!("Y: {}, FONTSIZE: {}", first_frag.y, first_frag.font_size);
                        println!("\n");
                    }
                    print!("          ");
                    for frag in line.text_fragments {
                        print!("{} |", frag.text);
                    }
                    println!("");
                }
            }
        }

    }
}
