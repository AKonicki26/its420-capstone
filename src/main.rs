use std::collections::HashSet;
use std::fs;

fn main() {

    let mut new_set = HashSet::new();

    new_set.insert(5);
    new_set.insert(6);
    new_set.insert(7);
    new_set.insert(8);

    println!("{:?}", new_set);

    match output_to_file() {
        Ok(()) => { println!("done"); }
        Err(e) => { println!("failed: {}", e); }
    }
}

type Include = String;

fn output_to_file() -> std::io::Result<()> {
    let mut includes_set: HashSet<Include> = HashSet::new();

    includes_set.insert("<iostream>".to_string());
    includes_set.insert("<fstream>".to_string());
    includes_set.insert("<vector>".to_string());
    includes_set.insert("<chrono>".to_string());

    let includes = includes_set.iter()
        .map(|x| format!("#include {}", x.to_string()))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", includes);

    let output = includes + "\n\nint main() {\n\tstd::cout << \"Hello World\" << std::endl;\n\treturn 0;\n}\n";

    fs::write("output.cpp", output)?;


    Ok(())
}
