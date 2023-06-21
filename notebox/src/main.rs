extern crate comrak;
use comrak::nodes::{AstNode, NodeLink, NodeValue};
use comrak::{
    format_commonmark, format_html, markdown_to_html, parse_document, Arena, ComrakOptions,
};
use std::any::Any;
use std::env;
use std::fs;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

fn iterate_or_process_note_path(note_path: String) {
    if fs::read_dir(note_path.clone()).is_err() {
        // Create the notes directory if none is found.
        println!("No notes directory found. Creating one now.");
        fs::create_dir(note_path).expect("Failed to create notes directory");
    } else {
        // Parse the notes directory into an iteratable object that we can walk through and take actions on.
        fs::read_dir(note_path)
            .and_then(|op| {
                for entry in op {
                    let entry = entry?;
                    let path = entry.path();
                    println!("Name: {}", path.display());
                    let metadata_object = std::fs::metadata(".").unwrap();
                    if metadata_object.is_dir() {
                        println!("Dir Name: {}", path.display());
                    }
                    if metadata_object.is_file() {
                        println!("File Name: {}", path.display());
                        process_md_file(path.into_os_string().into_string().unwrap());
                    }
                }
                Ok(())
            })
            .expect("Failed to read notes directory");
    }
}

fn public_file_transform(path_string: String, markdown_input: &str, processing_type: &str) {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    let mut options = &mut ComrakOptions::default();
    // Enable frontmatter detection
    options.extension.front_matter_delimiter = Some("---".to_string());
    // The "root" node, which we parse our markdown into
    let root = parse_document(&arena, &markdown_input, options);
    // hold a value found during an iteration function
    // https://stackoverflow.com/questions/30559073/cannot-borrow-captured-outer-variable-in-an-fn-closure-as-mutable
    let option_to_hold_file = Arc::new(Mutex::new(bool::default()));
    // Can I replace this ^ with Any?

    //let mut yaml_options = ManuallyDrop::new(Vec::new());
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
                //std::mem::replace(yaml_options, block);
                let _ = std::mem::replace(block, vec![]);
                option_to_hold_file.lock().unwrap().clone_from(&true);
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

    // dbg!(option_to_hold_file.lock().unwrap().clone());
    let file_result = markdown_to_html(markdown_input, options);
    // println!("\nFile HTML output:\n{}", file_result);
    // let file_output_result;
    // dbg!(root);
    let mut html = vec![];
    format_commonmark(&root, &options, &mut html);
    //println!("\nFile commanmark output:\n{}", html);
    dbg!(String::from_utf8(html).unwrap());
    println!("Public is true or partial");
    let write_result = fs::write(path_string, markdown_input);
    let written_file = match write_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

/**
 * This function will take a file path and transform the content
 * of the file as directed by YAML directives and then copy over
 * the file to the target directory.
 */
fn process_md_file(single_file_path: String) {
    println!("process_md_file");
    let file_contents: String = fs::read_to_string(single_file_path.clone())
        .expect("Should have been able to read the README file");
    let markdown_input: &str = &file_contents;
    // let yaml_test_result = String::from_utf8(html).unwrap();
    // alternatives - https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust
    let yaml_test_result = frontmatter::parse(markdown_input);
    // dbg!(yaml_test_result);
    assert!(yaml_test_result.is_ok() && !yaml_test_result.is_err());
    // Understand the object - https://docs.rs/yaml-rust/latest/yaml_rust/
    let some_yaml = yaml_test_result.unwrap();
    let yamlObj = some_yaml.unwrap();
    //let mut emitter = YamlEmitter::new(&mut out_str);
    // yaml_test_result.and_then(|i| dbg!(i));
    dbg!(&yamlObj);
    dbg!(&yamlObj["aliases"]);
    // Transition YAML object to hashmap so we can check it for a key.
    let yamlHashmap = &yamlObj.as_hash().unwrap();
    // YAML hashmap has keys and values set to Yaml formatted values using the `Yaml` object. We borrow the Yaml object here to transform a string into the expected type.
    let keycheck = yamlHashmap.contains_key(&Yaml::from_str("public"));
    // as_hash().unwrap().contains_key("public");
    dbg!(&yamlObj["public"]);

    // Check if variable is true
    if !keycheck {
        // Not a public ready file.
    } else {
        match yamlObj["public"].as_str().unwrap() {
            "true" => public_file_transform(single_file_path, markdown_input, "public"),
            "partial-public" => {
                public_file_transform(single_file_path, markdown_input, "partial-public")
            }
            "partial-private" => {
                public_file_transform(single_file_path, markdown_input, "partial-private")
            }
            "false" => println!("Public is false"),
            _ => println!("Public is not an expected value"),
        }
    }
    println!("process_md_file ends");
}

fn main() {
    let env_result = dotenvy::dotenv();
    if env_result.is_ok() {
        dbg!(env_result.unwrap());
    } else {
        println!("Env tool failed");
        dbg!(env_result.unwrap());
    }
    let note_path: String;
    if env::var("PATH_TO_NOTES").is_ok() {
        note_path = env::var("PATH_TO_NOTES").unwrap();
    } else {
        println!("No path to notes");
        note_path = "../Notes".to_string();
    }
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    println!("Needed value: {note_path}");

    if fs::read_dir(note_path.clone()).is_err() {
        // Create the notes directory if none is found.
        println!("No notes directory found. Creating one now.");
        fs::create_dir(note_path).expect("Failed to create notes directory");
    } else {
        // Parse the notes directory into an iteratable object that we can walk through and take actions on.
        fs::read_dir(note_path)
            .and_then(|op| {
                for entry in op {
                    let entry = entry?;
                    let path = entry.path();
                    println!("Name: {}", path.display());
                }
                Ok(())
            })
            .expect("Failed to read notes directory");
    }

    const PATH_TO_NOTES: &str = "../src/notes";
    let file_contents: String =
        fs::read_to_string("../README.md").expect("Should have been able to read the README file");
    let markdown_input: &str = &file_contents;

    process_md_file("../README.md".to_string());

    // let yaml_test_result = String::from_utf8(html).unwrap();
    // alternatives - https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust
    let yaml_test_result = frontmatter::parse(markdown_input);
    // dbg!(yaml_test_result);
    assert!(yaml_test_result.is_ok() && !yaml_test_result.is_err());
    // Understand the object - https://docs.rs/yaml-rust/latest/yaml_rust/
    let some_yaml = yaml_test_result.unwrap();
    let yamlObj = some_yaml.unwrap();
    //let mut emitter = YamlEmitter::new(&mut out_str);
    // yaml_test_result.and_then(|i| dbg!(i));
    dbg!(&yamlObj);
    dbg!(&yamlObj["aliases"]);
    dbg!(&yamlObj["public"]);
    // Check if variable is true

    //println!("\nProcessed File HTML output:\n{}", yaml_test_result);
}
