use local_ipaddress;
use poem::{listener::TcpListener, web::Multipart, Result, Route, Server};
use poem_openapi::{payload::Html, OpenApi, OpenApiService};
use qrcode::{render::unicode, QrCode};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{
    io::AsyncReadExt,
    sync::mpsc::{channel, Sender},
};
mod files;

const PORT: i32 = 27717;

struct Api {
    sender: Sender<bool>,
    start_upload: Arc<Mutex<Instant>>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/start", method = "get")]
    async fn start(&self) -> Result<Html<String>> {
        let start = Arc::clone(&self.start_upload);
        let mut m_start = start.lock().unwrap();
        *m_start = Instant::now();
        // println!("[log] upload started ! {:?}", Instant::now());
        Ok(Html(files::get_html_form()))
    }

    /// Upload file
    #[oai(path = "/", method = "post")]
    async fn upload(&self, mut multipart: Multipart) -> Result<Html<String>> {
        while let Ok(Some(field)) = multipart.next_field().await {
            let name = field.name().map(ToString::to_string);
            let file_name = match field.file_name().map(ToString::to_string) {
                None => "temp.data".to_string(),
                Some(n) => n,
            };
            match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&file_name)
            {
                Ok(mut file) => {
                    let mut reader = field.into_async_read();
                    loop {
                        let mut buffer = [0; 1024];
                        match reader.read(&mut buffer[..]).await {
                            Ok(n) => {
                                if n == 0 {
                                    break;
                                } else {
                                    let _ = file.write_all(&buffer[..n]);
                                }
                            }
                            Err(e) => {
                                println!("[error] can't read uploaded file stream, {}", e);
                                break;
                            }
                        }
                    }

                    let _ = self.sender.send(true).await;
                    let start = Arc::clone(&self.start_upload);
                    let m_start = start.lock().unwrap();
                    let s = *m_start;
                    println!("[log] upload ended in {:?}", s.elapsed());
                    println!("[log] file saved to {:?} filename={:?}", name, file_name);
                }
                Err(e) => println!("[error] can't open file {} : {}", file_name, e),
            }
        }
        Ok(Html(files::HTML_SUCCESS.to_string()))
    }

    #[oai(path = "/", method = "get")]
    async fn index(&self) -> Result<Html<String>> {
        Ok(Html(files::get_html_form()))
    }
}

fn print_qrcode() {
    let my_local_ip = local_ipaddress::get().unwrap();
    let code = QrCode::new(format!("http://{}:{}/", my_local_ip, PORT)).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
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
            sender: tx,
            start_upload: Arc::new(Mutex::new(Instant::now())),
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
