use clap::{Parser};
use std::fs;
use std::path::{Path, PathBuf};


#[derive(Parser)]
#[command(name = "Medical_file_organize")]
#[command(about = "Organize and manage medical reports by patient and type.")]
struct Cli {
    #[arg(short, long, value_name = "DIRS", num_args = 1.., value_delimiter = ',')]
    directories: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let output_dir = Path::new("output");
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    for dir in &cli.directories {
        if !dir.exists() || !dir.is_dir() {
            eprintln!("Error: {:?} is not a valid directory.", dir);
            continue;
        }
        println!("Processing directory: {:?}", dir);
        organize_files(dir, output_dir);
    }

    println!("All directories created successfully!");
}


fn organize_files(input_dir: &Path, output_dir: &Path) {
    let entries = fs::read_dir(input_dir).expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Failed to read");
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let patient_name = file_name.split('_').next().unwrap_or("Unknown_Patient");

            let file_type = match path.extension().and_then(|ext| ext.to_str()) {   //sorting through file type
                Some("dcm") => "Imaging_Files",
                Some("pdf") | Some("docx") | Some("txt") => "Patient_Reports",
                Some("csv") | Some("xlsx") | Some("xls") => "Lab_Reports",
                _ => "Other_Files",
            };

            let target_dir = output_dir.join(patient_name).join(file_type);
            fs::create_dir_all(&target_dir).expect("Failed to create directories");

            let new_file_path = resolve_duplicate(&target_dir, &file_name); // handle duplicate file
            fs::rename(&path, &new_file_path).expect("Failed to move file");
        }
    }
}


fn resolve_duplicate(target_dir: &Path, file_name: &str) -> PathBuf {
    let mut new_path = target_dir.join(file_name);
    let mut counter = 1;

    while new_path.exists() {       // Check duplicate file with same name 
        let file_stem = new_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let extension = new_path
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default();
        let new_file_name = format!("{}_{}{}", file_stem, counter, extension);
        new_path = target_dir.join(new_file_name);
        counter += 1;
    }

    new_path
}
