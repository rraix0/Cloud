use std::fs::File;
use std::io::Read;
use std::path::Path;
use image::ImageFormat;
use image::imageops::FilterType;
use rocket::http::Status;
use webp::Encoder;

pub fn get_thumbnail (file_name: String, file_path: Option<&Path>) -> Result<Vec<u8>, ()> {
    let img_vec:Option<Vec<u8>> = {
        let extension:String = Path::new(file_name.as_str())
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or("unknown".to_string());

        let format:ImageFormat = match extension.as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            "bmp" => ImageFormat::Bmp,
            _ => {
                println!("Błąd: Nieobsługiwane rozszerzenie '{}'. Obsługiwane: png, jpg, gif, webp, bmp.", extension);
                return Err(());

            },
        };

        println!("format: {:?}, extension: {:?}", format, extension);

        let returning_vec:Option<Vec<u8>> = match file_path {
            None => None,
            Some(filepath) => {
                let mut file_bytes = Vec::new();
                let mut file = File::open(filepath)
                    .map_err(|err| {
                        eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
                        ()
                    })?;

                file.read_to_end(&mut file_bytes)
                    .map_err(|err| {
                        eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
                        ()
                    })?;

                let img = match image::load_from_memory_with_format(&file_bytes, format) {
                    Ok(img) => img,
                    Err(e) => {
                        eprintln!("Błąd ładowania z formatem {}: {}", extension, e);
                        return Err(());
                    },
                };

                let resized = img.resize(50, 50, FilterType::Triangle);

                let rgba = resized.to_rgba8();

                let encoder = Encoder::from_rgba(rgba.as_raw(), 50, 50);
                let webp_memory = encoder.encode(80.0);

                let img_vec = webp_memory.to_vec();

                println!("{:?}", img_vec);
                Some(img_vec)
            }
        };
        returning_vec
    };
    if let Some(vec) = img_vec {
        return Ok(vec);
    } else{
        return Err(());
    }
}
/*
let img_vec:Option<Vec<u8>> = {
        let extension:String = Path::new(form.title.as_str())
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or("unknown".to_string());

        let format:ImageFormat = match extension.as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            "bmp" => ImageFormat::Bmp,
            _ => {
                println!("Błąd: Nieobsługiwane rozszerzenie '{}'. Obsługiwane: png, jpg, gif, webp, bmp.", extension);
                return Err((Status::BadRequest, "Missing file name2"));

            },
        };

        println!("format: {:?}, extension: {:?}", format, extension);

        let returning_vec:Option<Vec<u8>> = match form.file.path() {
            None => None,
            Some(filepath) => {
                let mut file_bytes = Vec::new();
                let mut file = File::open(filepath)
                    .map_err(|err| {
                        eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
                        (Status::InternalServerError, "Couldn't open file!")
                    })?;

                file.read_to_end(&mut file_bytes)
                    .map_err(|err| {
                        eprintln!("Occurred an error while trying to remove file after db error: {:?}", err);
                        (Status::InternalServerError, "Couldn't open file2!")
                    })?;

                let img = match image::load_from_memory_with_format(&file_bytes, format) {
                    Ok(img) => img,
                    Err(e) => {
                        eprintln!("Błąd ładowania z formatem {}: {}", extension, e);
                        return Err((Status::InternalServerError, "Couldn't open file2!"));
                    },
                };

                let resized = img.resize(50, 50, FilterType::Triangle);

                let rgba = resized.to_rgba8();

                let encoder = Encoder::from_rgba(rgba.as_raw(), 50, 50);
                let webp_memory = encoder.encode(80.0);

                let img_vec = webp_memory.to_vec();

                println!("{:?}", img_vec);
                Some(img_vec)
            }
        };
        returning_vec
    };
 */