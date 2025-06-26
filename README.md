# Basic Structured Text Extractor

This crate uses the [mupdf-rs bindings](https://github.com/messense/mupdf-rs)  to do a very simple structured text extraction. 

## License
Because of the usage of `mupdf` itself, this extractor is AGPL-Licensed.

## Limiatations / Scope

This module is not built for broad, generalized usage. But it may be a simple jumping-off point, an example of how to use the mupdf bindings.

This module assumes the following is true for the use case:
- Your document has left-to-right text
- You want "Lines" which span the full width of the page

This Basic Text Extractor is what it says on the tin: basic.

## Structure
The structure looks like this for each page --NOTE that font_name is a field, but there doesn't appear to be a way to get the font in the bindings (or, at least I have not properly identified it.)

```
Page {
    lines {
        Line {
            text_fragments {
                Fragment {
                    text: String,
                    x: f64,
                    y: f64,
                    font_name: Option<String>,
                    font_size: f64,
                    bbox_width: f64,
                    bbox_height: f64
                }
            }
        }
    }

}

```

A `Line` is a series of `TextFragments` which share the same Y-Value. The fragments within the line are sorted by their X-value to be in proper PDF left-to-right order.

## Usage

```rust
use mupdf_basic_text_extractor::get_structured_document_from_filepath;

let document: Result<Doc, Box<dyn std::error::Error>> = get_structured_document_from_filepath(path);

for page in document.pages {
    for line in page.lines {
        for fragment in line.text_fragments {
            todo()!
        }
    }
}

```

## Hotfix 2025-06-26

MuPDF uses a top-left coordinate system. Not only that, but it was not clear to me what counts as the "local origin" for a text element. The x,y positions now derive directly from the lower-left of the bounding box, and the y-height is calculated as the difference from the page height.

This now reflects the PDF coordinate system, with 0,0 being in the bottom left.