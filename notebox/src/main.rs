extern crate comrak;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_html, markdown_to_html, parse_document, Arena, ComrakOptions};
use std::fs;

fn main() {
    let file_contents: String = fs::read_to_string("../README.md")
        .expect("LogRocket: Should have been able to read the file");
    let markdown_input: &str = &file_contents;
    // println!("Parsing the following markdown string:\n{}", markdown_input);

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let mut options = &mut ComrakOptions::default();
    // Enable frontmatter detection
    options.extension.front_matter_delimiter = Some("---".to_string());
    // The "root" node, which we parse our markdown into
    let root = parse_document(&arena, &file_contents, options);
    // Iterate through the nodes (and their children) recursively
    // We pass the node to the callback provided as the second function param
    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where
        F: Fn(&'a AstNode<'a>),
    {
        f(node);
        for c in node.children() {
            iter_nodes(c, f);
        }
    }
    // Call the iterate nodes function
    iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
        // Check the value of the node data
        // This is basically a giant "switch" statement (for my JS peeps)
        &mut NodeValue::Text(ref mut text) => {
            let orig = std::mem::replace(text, vec![]);
            *text = String::from_utf8(orig)
                .unwrap()
                .replace("my", "your")
                .as_bytes()
                .to_vec();
        }
        _ => (),
    });

    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();

    let result = String::from_utf8(html).unwrap();

    assert_eq!(
        result,
        "<p>This is your input.</p>\n\
		 <ol>\n\
		 <li>Also your input.</li>\n\
		 <li>Certainly your input.</li>\n\
		 </ol>\n"
    );

    let basic_result = markdown_to_html("Hello, **世界**!", &ComrakOptions::default());
    assert_eq!(basic_result, "<p>Hello, <strong>世界</strong>!</p>\n");

    println!("\nHTML output:\n");
    println!("{}", result);

    println!("\nBasic HTML output:\n{}", basic_result);

    let file_result = markdown_to_html(markdown_input, &ComrakOptions::default());
    println!("\nFile HTML output:\n{}", file_result);
}
