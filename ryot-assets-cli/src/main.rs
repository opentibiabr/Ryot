use clap::{Parser, Subcommand};
use config::Config;
use glam::UVec2;
use log::*;
use ryot::{decompress_sprite_sheets, ContentConfigs};
use simple_logger::SimpleLogger;
use std::path::{Path, PathBuf};
use std::{fs, result};

static DEFAULT_CONTENT_CONFIG_PATH: &str = "config/assets-cli.toml";

/// CLI to manage assets from Cipbia
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extracts assets into sprite sheets
    Extract,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let cli = Cli::parse();
    let config_path = cli
        .config
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONTENT_CONFIG_PATH));

    match &cli.command {
        Some(Commands::Extract) => {
            info!("Running extract command");
            ContentBuild::from_path(config_path)
                .run()
                .expect("Failed to build assets");
        }
        None => {
            println!("No command provided. Use --help to see available commands");
        }
    }
}

#[derive(Debug)]
struct ContentBuild {
    path: PathBuf,
}

impl ContentBuild {
    fn from_path(path: PathBuf) -> Self {
        Self { path }
    }

    fn run(self) -> color_eyre::Result<()> {
        info!("Running content build {:?}", self);

        let content_config_path = self.path.clone();
        let content_config = read_content_configs(content_config_path.clone());
        let ContentConfigs { directories, .. } = content_config.clone();

        directories.source_path.try_exists().unwrap_or_else(|_| {
            panic!(
                "Source path {} does not exist",
                directories.source_path.display()
            )
        });

        if copy_catalog(&directories.source_path, &directories.destination_path).is_err() {
            error!(
                "Catalog file not found in {}",
                directories.source_path.display()
            );
            return Ok(());
        }

        copy_appearances(&directories.source_path, &directories.destination_path)?;
        let sheet_size = content_config.sprite_sheet.sheet_size;
        decompress_sprites(content_config, &sheet_size)?;

        Ok(())
    }
}

fn copy_catalog(
    source_path: &Path,
    destination_path: &Path,
) -> result::Result<u64, std::io::Error> {
    let file_name = "catalog-content.json";

    fs::copy(
        source_path.join(file_name),
        destination_path.join(file_name),
    )
}

fn copy_appearances(
    source_path: &Path,
    destination_path: &Path,
) -> result::Result<(), std::io::Error> {
    let entries = fs::read_dir(source_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("appearances-")
                    && file_name.ends_with(&format!(".{}", "dat"))
                {
                    let new_path = destination_path.join("appearances.dat");

                    fs::copy(&path, new_path)?;

                    break;
                }
            }
        }
    }

    Ok(())
}

fn decompress_sprites(
    content_configs: ContentConfigs,
    sheet_size: &UVec2,
) -> result::Result<(), std::io::Error> {
    let ContentConfigs { directories, .. } = content_configs.clone();

    let files = fs::read_dir(directories.source_path)?
        .filter_map(|e| {
            if let Ok(entry) = e {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("sprites-")
                            && file_name.ends_with(&format!(".{}", "bmp.lzma"))
                        {
                            return Some(file_name.to_string());
                        }
                    }
                }
            }

            None
        })
        .collect::<Vec<String>>();

    decompress_sprite_sheets(content_configs, sheet_size, &files);

    Ok(())
}

fn read_content_configs(config_path: PathBuf) -> ContentConfigs {
    let settings = Config::builder()
        .add_source(config::File::from(config_path))
        .build()
        .expect("Failed to build config")
        .try_deserialize::<ContentConfigs>()
        .expect("Failed to deserialize config");

    let dir_settings = &settings.directories;

    match is_path_within_root(&dir_settings.destination_path, Path::new("assets")) {
        Ok(true) => settings,
        Ok(false) | Err(_) => panic!(
            "Target path {} is not within assets folder",
            dir_settings
                .destination_path
                .to_str()
                .expect("Failed to convert target path to str")
        ),
    }
}

fn is_path_within_root(
    destination_path: &Path,
    root_path: &Path,
) -> result::Result<bool, std::io::Error> {
    Ok(fs::canonicalize(destination_path)?.starts_with(fs::canonicalize(root_path)?))
}
