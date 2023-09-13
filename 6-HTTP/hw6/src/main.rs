use axum::{
    extract::Extension,
    routing::get,
    // response::NamedFile,
    Router,
};
use rand::seq::SliceRandom;
use std::io;
use std::sync::Arc;
use tokio::fs::File;

#[derive(Debug)]
struct Image {
    path: String,
}

impl Image {
    fn new(path: String) -> Self {
        Image { path }
    }

    fn random_from_directory(directory: &str) -> Result<Self, io::Error> {
        let files: Vec<_> = std::fs::read_dir(directory)?
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        Some(path.to_string_lossy().to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        if let Some(random_path) = files.choose(&mut rand::thread_rng()) {
            Ok(Self::new(random_path.to_string()))
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No images found"))
        }
    }
}

async fn random_image(Extension(images_directory): Extension<Arc<String>>) -> Result<File, io::Error> {
    match Image::random_from_directory(&images_directory.as_str()) {
        Ok(image) => {
            let file_path = format!("{}/{}", &images_directory, image.path);
            let named_file = File::open(file_path).await?;
            Ok(named_file)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let images_directory = Arc::new("images".to_string());

    let app = Router::new()
        .route("/random-image", get(random_image))
        .layer(Extension(images_directory.clone()));

    println!("{:?}", images_directory);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
