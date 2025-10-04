mod tokenizer;

use std::collections::HashSet;
use std::fs;
use regex::Regex;
use crate::tokenizer::Tokenizer;

fn main() {

    Tokenizer::tokenize("let x = 5");

    /*
    match generate_malware() {
        Ok(()) => { println!("done"); }
        Err(e) => { println!("failed: {}", e); }
    }
    */
}

type Include = String;

fn generate_malware() -> std::io::Result<()> {

    let data =  read_malware_program("./.mal-scripts/example.mal").unwrap();

    println!("data: {:?}", data);

    let includes: HashSet<Include> = HashSet::from(["<windows.h>", "<iostream>", "<chrono>", "<thread>"].map(|s| s.to_string()));

    let print_re = Regex::new(r#"print\([ \t\n]*"(.+)"[ \t\n]*\)"#).unwrap();
    let windows_popup_re = Regex::new(r#"create_windows_popup\([ \t\n]*"(.+)",[ \t\n]*"(.+)"[ \t\n]*\)"#).unwrap();
    let sleep_re = Regex::new(r#"sleep\([ \t\n]*([0-9]+)[ \t\n]*\)"#).unwrap();

    let cpp_script: String = data.replace("start", "main");

    println!("print statement found: {}", print_re.is_match(&cpp_script));
    println!("windows popup statement found: {}", windows_popup_re.is_match(&cpp_script));
    println!("sleep statement found: {}", sleep_re.is_match(&cpp_script));

    let after_print_replace = print_re.replace_all(&cpp_script, "std::cout << \"$1\" << std::endl");
    let after_win_popup_replace = windows_popup_re.replace_all(&after_print_replace,
       "MessageBox(nullptr,\"$1\",\"$2\", MB_OK)");
    let after_sleep_replace = sleep_re.replace_all(&after_win_popup_replace, "Sleep($1 * 1000)").to_string();

    output_to_file(includes, after_sleep_replace)
}

fn read_malware_program(file_name: &str) -> Result<String, String> {
    match fs::read_to_string(file_name) {
        Ok(content) => { Ok(content) }
        Err(e) => { Err(format!("Error reading file: {}", e)) }
    }
}

fn output_to_file(includes: HashSet<Include>, main_content: String) -> std::io::Result<()> {

    let formatted_includes = includes.iter()
        .map(|x| format!("#include {}", x.to_string()))
        .collect::<Vec<String>>()
        .join("\n");

    // println!("{}", formatted_includes);

    let output = formatted_includes + "\n\n"+  main_content.as_str();

    fs::write("output.cpp", output)
}
