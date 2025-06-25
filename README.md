# Basic Structured Text Extractor

This crate uses the [mupdf-rs bindings](https://github.com/messense/mupdf-rs)  to do a very simple structured text extraction. 

## License
Because of the usage of `mupdf` itself, this extractor is AGPL-Licensed.


## Structure
The structure looks like this for each page --NOTE that font_name is a field, but there doesn't appear to be a way to get the font in the bindings (or, at least I have not properly identified it.)

```
lines {
    Line {
        text_fragments {
            TextFragment {
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

```

A `Line` is a series of `TextFragments` which share the same Y-Value. The fragments within the line are sorted by their X-value to be in proper PDF left-to-right order.