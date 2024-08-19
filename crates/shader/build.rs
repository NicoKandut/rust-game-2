use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let root = env::current_dir().unwrap().to_str().unwrap().to_owned();

    let shader_dir = root.clone() + "\\src\\shaders";
    println!("Shaders are in: {shader_dir}");

    let spv_dir = root.clone() + "\\src\\spv";
    println!("Outputs are in: {}", spv_dir.clone());

    let vulkan_dir = env::var("VULKAN_SDK").unwrap();
    let glsl_compile = vulkan_dir + "\\Bin\\glslc.exe";

    println!("Recompiling shaders...");

    for entry in fs::read_dir(shader_dir.clone()).unwrap() {
        let input_file = entry.unwrap().file_name();
        let input_path = shader_dir.clone() + "\\" + input_file.to_str().unwrap();
        println!("cargo::rerun-if-changed=src/{}", input_path);
        let output_path = spv_dir.clone() + "\\" + input_file.to_str().unwrap() + ".spv";
        let stderr = Command::new(&glsl_compile)
            .arg(input_path)
            .arg("-o")
            .arg(output_path)
            .output()
            .expect("Failed to compile shader").stderr;
        if !stderr.is_empty() {
            panic!("Error in shader\n{}", String::from_utf8(stderr).unwrap());
        }
    }

    println!("Done");
}