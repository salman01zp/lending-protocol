use std::{
    collections::BTreeMap,
    env,
    ffi::OsString,
    fmt::Write as FmtWrite,
    fs,
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

use miden_assembly::Report;
use miden_lib::transaction::TransactionKernel;
use miden_objects::{
    assembly::{
        diagnostics::{IntoDiagnostic, Result},
        Assembler, DefaultSourceManager, Library, LibraryPath, Module, ModuleKind,
    },
    utils::Serializable,
};
use regex::Regex;
use walkdir::WalkDir;

// CONSTANTS
// ================================================================================================

const CAN_WRITE_TO_SRC: bool = option_env!("DOCS_RS").is_none();

const ASSETS_DIR: &str = "assets";
const ASM_DIR: &str = "asm";
const ASM_CONTRACTS_DIR: &str = "contracts";
const ASM_NOTE_SCRIPTS_DIR: &str = "note_scripts";
const LENDING_ERRORS_FILE: &str = "src/errors/lending_errors.rs";

// MAIN
// ================================================================================================

/// Compiles MASM code during build time.
/// - Compiles contracts in asm/contracts directory into MASL library files
/// - Compiles note scripts in asm/note_scripts directory into MASB files
/// - Generates error constants from MASM code
fn main() -> Result<()> {
    // Re-build when the MASM code changes
    println!("cargo:rerun-if-changed={ASM_DIR}");
    println!("cargo::rerun-if-env-changed=BUILD_GENERATED_FILES_IN_SRC");

    // Set up directories
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_dir = env::var("OUT_DIR").unwrap();
    let src = Path::new(&crate_dir).join("src").join(ASM_DIR);
    let dst = Path::new(&build_dir).to_path_buf();

    // Copy MASM source to build directory
    if src.exists() {
        copy_directory(&src, &dst);

        // Set source and target directories
        let source_dir = dst.join(ASM_DIR);
        let contracts_dir = source_dir.join(ASM_CONTRACTS_DIR);
        let note_scripts_dir = source_dir.join(ASM_NOTE_SCRIPTS_DIR);

        let target_dir = Path::new(&build_dir).join(ASSETS_DIR);
        let target_contracts_dir = target_dir.join(ASM_CONTRACTS_DIR);
        let target_note_scripts_dir = target_dir.join(ASM_NOTE_SCRIPTS_DIR);

        // Compile contracts if they exist
        if contracts_dir.exists() {
            let assembler = compile_contracts(&contracts_dir, &target_contracts_dir)?;

            // Compile note scripts if they exist
            if note_scripts_dir.exists() {
                compile_note_scripts(&note_scripts_dir, &target_note_scripts_dir, assembler)?;
            }
        }

        // Generate error constants from MASM if contracts exist
        if contracts_dir.exists() && CAN_WRITE_TO_SRC {
            generate_error_constants(&contracts_dir, LENDING_ERRORS_FILE)?;
        }
    } else {
        println!("cargo:warning=No src/asm directory found, skipping MASM compilation");
    }

    Ok(())
}

// COMPILATION FUNCTIONS
// ================================================================================================

fn create_assembler() -> Result<Assembler> {
    Ok(TransactionKernel::assembler().with_debug_mode(true))
}

/// Compiles contract MASM files into MASL libraries
fn compile_contracts(
    source_dir: &Path,
    target_dir: &Path,
) -> Result<Assembler, Report> {
    fs::create_dir_all(target_dir).into_diagnostic()?;

    let mut assembler = create_assembler()?;

    for masm_file_path in get_masm_files(source_dir).into_diagnostic()? {
        let name = masm_file_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let code = fs::read_to_string(&masm_file_path).into_diagnostic()?;

        let library = create_library(
            assembler.clone(),
            &format!("lending::{}", name),
            &code,
        )?;

        assembler = assembler.clone().with_dynamic_library(library.clone())?;

        let library_path = target_dir.join(&name).with_extension(Library::LIBRARY_EXTENSION);
        library.write_to_file(library_path).into_diagnostic()?;

        println!("cargo:warning=Compiled lending contract: {}", name);
    }

    Ok(assembler)
}

/// Compiles note script MASM files into MASB program files
fn compile_note_scripts(
    source_dir: &Path,
    target_dir: &Path,
    assembler: Assembler,
) -> Result<()> {
    fs::create_dir_all(target_dir).into_diagnostic()?;

    for masm_file_path in get_masm_files(source_dir).into_diagnostic()? {
        let program = assembler.clone().assemble_program(masm_file_path.clone())?;
        let bytes = program.to_bytes();

        let masb_file_name = masm_file_path.file_stem().unwrap();
        let masb_file_path = target_dir.join(masb_file_name).with_extension("masb");

        fs::write(&masb_file_path, bytes).into_diagnostic()?;

        println!("cargo:warning=Compiled note script: {}", masb_file_name.to_str().unwrap());
    }

    Ok(())
}

/// Creates a Miden library from source code
pub fn create_library(
    assembler: Assembler,
    library_path: &str,
    source_code: &str,
) -> Result<Library, Report> {
    let source_manager = Arc::new(DefaultSourceManager::default());
    let module = Module::parser(ModuleKind::Library).parse_str(
        LibraryPath::new(library_path).into_diagnostic()?,
        source_code,
        &source_manager,
    )?;
    let library = assembler.clone().assemble_library([module])?;
    Ok(library)
}

// ERROR GENERATION
// ================================================================================================

fn generate_error_constants(source_dir: &Path, output_file: &str) -> Result<()> {
    let mut errors = BTreeMap::new();

    // Walk all MASM files and extract error constants
    for entry in WalkDir::new(source_dir) {
        let entry = entry.into_diagnostic()?;
        if !is_masm_file(entry.path()).into_diagnostic()? {
            continue;
        }
        let file_contents = fs::read_to_string(entry.path()).into_diagnostic()?;
        extract_errors(&mut errors, &file_contents)?;
    }

    // Generate the error file
    let error_file_content = generate_error_file(errors)?;

    // Ensure errors directory exists
    let error_dir = Path::new(output_file).parent().unwrap();
    fs::create_dir_all(error_dir).into_diagnostic()?;

    fs::write(output_file, error_file_content).into_diagnostic()?;

    println!("cargo:warning=Generated error constants in {}", output_file);

    Ok(())
}

fn extract_errors(
    errors: &mut BTreeMap<String, String>,
    file_contents: &str,
) -> Result<()> {
    let regex = Regex::new(r#"const\.ERR_(?<name>.*)="(?<message>.*)""#).unwrap();

    for capture in regex.captures_iter(file_contents) {
        let error_name = capture
            .name("name")
            .expect("error name should be captured")
            .as_str()
            .trim()
            .to_owned();
        let error_message = capture
            .name("message")
            .expect("error message should be captured")
            .as_str()
            .trim()
            .to_owned();

        if let Some(existing_message) = errors.get(&error_name) {
            if existing_message != &error_message {
                return Err(Report::msg(format!(
                    "Error constant ERR_{} defined with different messages",
                    error_name
                )));
            }
        }

        errors.insert(error_name, error_message);
    }

    Ok(())
}

fn generate_error_file(errors: BTreeMap<String, String>) -> Result<String> {
    let mut output = String::new();

    writeln!(output, "use miden_lib::errors::MasmError;\n").unwrap();
    writeln!(
        output,
        "// This file is generated by build.rs, do not modify manually."
    )
    .unwrap();
    writeln!(
        output,
        "// It extracts error constants from MASM files in the contracts directory.\n"
    )
    .unwrap();

    for (name, message) in errors.iter() {
        writeln!(output, "/// Error Message: \"{}\"", message).into_diagnostic()?;
        writeln!(
            output,
            r#"pub const ERR_{}: MasmError = MasmError::from_static_str("{}");"#,
            name, message
        )
        .into_diagnostic()?;
    }

    Ok(output)
}

// HELPER FUNCTIONS
// ================================================================================================

/// Recursively copies src into dst
fn copy_directory<T: AsRef<Path>, R: AsRef<Path>>(src: T, dst: R) {
    let mut prefix = src.as_ref().canonicalize().unwrap();
    prefix.pop();

    let target_dir = dst.as_ref().join(ASM_DIR);
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).unwrap();
    }

    let dst = dst.as_ref();
    let mut todo = vec![src.as_ref().to_path_buf()];

    while let Some(goal) = todo.pop() {
        for entry in fs::read_dir(goal).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                let src_dir = path.canonicalize().unwrap();
                let dst_dir = dst.join(src_dir.strip_prefix(&prefix).unwrap());
                if !dst_dir.exists() {
                    fs::create_dir_all(&dst_dir).unwrap();
                }
                todo.push(src_dir);
            } else {
                let dst_file = dst.join(path.strip_prefix(&prefix).unwrap());
                fs::copy(&path, dst_file).unwrap();
            }
        }
    }
}

/// Returns a vector with paths to all MASM files in the specified directory
fn get_masm_files<P: AsRef<Path>>(dir_path: P) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let path = dir_path.as_ref();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let file = entry?;
            let file_path = file.path();
            if is_masm_file(&file_path)? {
                files.push(file_path);
            }
        }
    }

    Ok(files)
}

/// Returns true if the path is a MASM file
fn is_masm_file(path: &Path) -> io::Result<bool> {
    if let Some(extension) = path.extension() {
        let extension = extension
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid UTF-8 filename"))?
            .to_lowercase();
        Ok(extension == "masm")
    } else {
        Ok(false)
    }
}
