extern crate comrak;
use comrak::nodes::{AstNode, NodeLink, NodeValue};
use comrak::{
    format_commonmark, format_html, markdown_to_html, parse_document, Arena, ComrakOptions,
};
use std::fs;
use std::mem::ManuallyDrop;

fn main() {
    let file_contents: String =
        fs::read_to_string("../README.md").expect("Should have been able to read the README file");
    let markdown_input: &str = &file_contents;
    // println!("Parsing the following markdown string:\n{}", markdown_input);

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let mut options = &mut ComrakOptions::default();
    // Enable frontmatter detection
    options.extension.front_matter_delimiter = Some("---".to_string());
    // The "root" node, which we parse our markdown into
    let root = parse_document(&arena, &file_contents, options);
    // hold a value found during an iteration function

    let hold_file = ManuallyDrop::new(bool::default());
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
    // let mut frontmatter_string: String = Default::default();
    // Call the iterate nodes function
    iter_nodes(root, &|node| {
        // Check the value of the node data
        match &mut node.data.borrow_mut().value {
            NodeValue::SoftBreak => {
                // println!("Soft break")
            }
            NodeValue::LineBreak => {
                // println!("Line break")
            }
            NodeValue::Item(ref mut blocks) => {
                // std::mem::drop(blocks);
                // std::mem::replace(blocks, vec![]); // Zeroing this out requires changing the type.
                // dbg!(blocks);
            }
            NodeValue::FrontMatter(ref mut block) => {
                println!(
                    "Frontmatter: {}",
                    String::from_utf8(block.to_vec()).unwrap()
                );
                // let replace_result = std::mem::replace(block, block.to_vec());
                //*block = vec![];
                let _ = std::mem::replace(block, vec![]);
                dbg!(block);
                // let raw_frontmatter = String::from_utf8(block.to_vec())
                //    .expect("Couldn't parse frontmatter into string.");
            }
            &mut NodeValue::CodeBlock(ref mut block) => {
                // std::mem::drop(block);
                // let orig = std::mem::replace(&mut block.literal, vec![]);
                // println!("Code Block: {}", String::from_utf8(orig).unwrap());
            }
            &mut NodeValue::Link(ref mut link) => {
                // std::mem::take(&mut link);
                // let orig = std::mem::replace(&mut link.url, vec![]);
                // println!("Link: {}", String::from_utf8(orig).unwrap());
            }
            &mut NodeValue::Strong => {
                // println!("Bold text: ");
            }
            // Got text?
            &mut NodeValue::Text(ref mut text) => {
                //let orig = std::mem::replace(text, vec![]);
                // *text = vec![];
                // println!("{}", String::from_utf8(orig).unwrap());
                // *text = String::from_utf8(orig).unwrap().replace("my", "your").as_bytes().to_vec();
            }
            _ => (),
        }
    });

    let file_result = markdown_to_html(markdown_input, options);
    println!("\nFile HTML output:\n{}", file_result);
    // let file_output_result;
    // dbg!(root);
    let mut html = vec![];
    format_commonmark(&root, &options, &mut html);
    //println!("\nFile commanmark output:\n{}", html);
    dbg!(String::from_utf8(html).unwrap());

    // let yaml_test_result = String::from_utf8(html).unwrap();
    let yaml_test_result = frontmatter::parse(markdown_input);
    // dbg!(yaml_test_result);
    assert!(yaml_test_result.is_ok() && !yaml_test_result.is_err());
    let some_yaml = yaml_test_result.unwrap();
    // yaml_test_result.and_then(|i| dbg!(i));
    dbg!(some_yaml.unwrap());

    //println!("\nProcessed File HTML output:\n{}", yaml_test_result);
}
