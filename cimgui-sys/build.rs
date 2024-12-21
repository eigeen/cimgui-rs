use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("headers/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("src/bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    // process bindings.rs
    // remove duplicate enum prefixes
    // pub const ImGuiKey_ImGuiKey_GamepadLStickLeft: ImGuiKey = 647; -> pub const ImGuiKey_GamepadLStickLeft: ImGuiKey = 647;
    let bindings_file = std::fs::read_to_string(&out_path).expect("Unable to read bindings.rs");

    let mut new_lines = vec![];
    for line in bindings_file.lines() {
        if !line.starts_with("pub const ") {
            new_lines.push(line.to_string());
            continue;
        }

        let no_prefix = line.strip_prefix("pub const ").unwrap();
        let Some(end_pos) = no_prefix.find(":") else {
            new_lines.push(line.to_string());
            continue;
        };
        let const_name = &no_prefix[..end_pos];

        let parts = const_name.split('_').collect::<Vec<_>>();
        if parts.len() <= 2 {
            new_lines.push(line.to_string());
            continue;
        }

        let mut parts_iter = parts.iter();

        let mut skip_count = 1;
        let no1 = parts_iter.next().unwrap();
        let mut no2 = parts_iter.next().unwrap();
        while no2.is_empty() {
            // maybe double-underscore
            no2 = parts_iter.next().unwrap();
            skip_count += 1;
        }

        if no1 == no2 {
            // remove prefix and merge to new line
            let new_const_name = parts[skip_count..].join("_");
            let new_line = format!("pub const {new_const_name}{}", &no_prefix[end_pos..]);
            new_lines.push(new_line);
            continue;
        }

        new_lines.push(line.to_string());
    }

    std::fs::write(out_path, new_lines.join("\n")).expect("Unable to write bindings.rs");
}
