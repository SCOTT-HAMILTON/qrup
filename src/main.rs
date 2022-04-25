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
use local_ip_address::local_ip;
use qrcode::{
    QrCode,
    render::unicode,
};
mod files;

const PORT: i32 = 27717;

struct Api {
    sender: Sender<bool>
}

#[OpenApi]
impl Api {
    /// Upload file
    #[oai(path = "/", method = "post")]
    async fn upload(&self, mut multipart: Multipart) -> Result<Html<String>> {
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
        Ok(Html(files::HTML_SUCCESS.to_string(),))
    }

    #[oai(path = "/", method = "get")]
    async fn index(&self) -> Result<Html<String>> {
        Ok(Html(files::get_html_form(),))
    }
}

fn print_qrcode() {
    let my_local_ip = local_ip().unwrap();
    // println!("This is my local IP address: {:?}", my_local_ip);
    let code = QrCode::new(format!("http://{}:{}/", my_local_ip, PORT)).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    print_qrcode(); 

    let (tx, mut rx) = channel::<bool>(1);
    let api_service = OpenApiService::new(
        Api {
            sender: tx
        },
        "QrUp File Uploader",
        "1.0",
    )
    .server(format!("http://0.0.0.0:{}/", PORT));

    let app = Route::new().nest("/", api_service);

    Server::new(TcpListener::bind(format!("0.0.0.0:{}", PORT)))
        .run_with_graceful_shutdown(
            app,
            async move {
                rx.recv().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await
}
