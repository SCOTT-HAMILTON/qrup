use poem::{
    listener::TcpListener, Result, Route, Server,
    web::{Multipart},
};
use poem_openapi::{
    payload::Html,
    OpenApi, OpenApiService,
};
use std::{
    io::Write,
    fs::OpenOptions,
    time::Duration,
};
use tokio::sync::mpsc::{
    channel, Sender
};

struct Api {
    sender: Sender<bool>
}

#[OpenApi]
impl Api {
    /// Upload file
    #[oai(path = "/", method = "post")]
    async fn upload(&self, mut multipart: Multipart) -> Result<Html<&'static str>> {
        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().map(ToString::to_string);
            let file_name = match field.file_name().map(ToString::to_string) {
                None => "temp.data".to_string(),
                Some(n) => n
            };
            if let Ok(bytes) = field.bytes().await {
                match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_name) {
                    Ok(mut file) =>
                        match file.write_all(&bytes) {
                            Ok(_) => {
                                let _ = self.sender.send(true).await;
                                println!(
                                    "name={:?} filename={:?} length={}",
                                    name,
                                    file_name,
                                    bytes.len()
                                );
                            },
                            Err(e) =>
                                println!(
                                    "[error] couldn't write data to file {}, : {}",
                                    file_name, e
                                )
                        },
                    Err(e) => println!("[error] can't open file {} : {}",
                                       file_name, e)
                }
            }
        }
        Ok(Html(
            r###"
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>QrUp File Upload</title>
            </head>
            <body>
                File uploaded successfully!
            </body>
            </html>
            "###,
        ))
    }

    #[oai(path = "/", method = "get")]
    async fn index(&self) -> Result<Html<&'static str>> {
        Ok(Html(
            r###"
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>QrUp File Upload</title>
            </head>
            <body>
                <form action="/" enctype="multipart/form-data" method="post">
                    <input type="file" name="upload" id="file">
                    <button type="submit">Submit</button>
                </form>
            </body>
            </html>
            "###,
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let (tx, mut rx) = channel::<bool>(1);
    let api_service = OpenApiService::new(
        Api {
            sender: tx
        },
        "QrUp File Uploader",
        "1.0",
    )
    .server("http://0.0.0.0:27717/");

    let app = Route::new().nest("/", api_service);

    Server::new(TcpListener::bind("0.0.0.0:27717"))
        .run_with_graceful_shutdown(
            app,
            async move {
                rx.recv().await;
                println!("received lol!");
                // let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await
}
