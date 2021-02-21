use shaderc;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn visit_files(dir: &Path, f: &mut dyn FnMut(&Path) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_files(&path, f)?;
            } else {
                f(&path)?
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut compiler = shaderc::Compiler::new().expect("failed to initialize glsl compiler");
    let mut compile_options =
        shaderc::CompileOptions::new().expect("failed to initialize compiler options");

    if env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        compile_options.add_macro_definition("TARGET_WASM", None);
    }

    let out_dir: PathBuf = [&env::var("OUT_DIR").unwrap(), "shaders"].iter().collect();

    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }

    visit_files(Path::new("src/shaders/"), &mut |path| {
        let binary = match compiler.compile_into_spirv(
            &fs::read_to_string(path)?,
            match path.extension().and_then(|s| s.to_str()).unwrap() {
                "vert" => shaderc::ShaderKind::Vertex,
                "frag" => shaderc::ShaderKind::Fragment,
                "comp" => shaderc::ShaderKind::Compute,
                _ => return Ok(()),
            },
            path.file_name().and_then(|s| s.to_str()).unwrap(),
            "main",
            Some(&compile_options),
        ) {
            Ok(v) => v,
            Err(shaderc::Error::CompilationError(_, msg)) => {
                println!("{}", msg);
                panic!("shader compilation error")
            }
            e => e.unwrap(),
        };

        let mut new_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        new_path.push(path.strip_prefix("src/").unwrap());

        fs::write(&new_path, binary.as_binary_u8())?;

        Ok(())
    })?;

    // Set debug cfg
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }

    Ok(())
}
