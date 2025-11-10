use std::fs;
use std::path::PathBuf;
use tiny_http::{Server, Response, Header};
use markdown::to_html;
use clap::Parser;

/// A minimal markdown web server
#[derive(Parser, Debug)]
#[command(version, about = "Serve a markdown file as HTML")]
struct Args {
    /// IP Address to bind to
    #[arg(short='a', long="address", default_value="0.0.0.0")]
    address: String,

    /// Port to listen on
    #[arg(short='p', long="port", default_value_t = 3000)]
    port: u16,

    /// Path to the markdown file to host
    #[arg(default_value = "./index.md")]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let bind_addr = format!("{}:{}", args.address, args.port);

    println!("Serving {} at http://{} ...", args.file.display(), bind_addr);

    let server = Server::http(&bind_addr).unwrap();

    for request in server.incoming_requests(){
        let markdown = fs::read_to_string(&args.file)
            .unwrap_or_else(|_| "# File not found".into());
        let html_content = to_html(&markdown);

        let html_page = format!(
            "<!DOCTYPE html>
            <html>
                <head>
                    <meta charset=\"utf-8\">
                    <title>Apps</title>
                    <style>
                        body {{ font-family: sans-serif; 
                            max-width: 800px; 
                            margin: auto;
                            padding: 2rem;
                        }}
                        a {{
                            color: #007acc;
                            text-decoration: none;
                        }}
                        a:hover {{ text-decoration: underline; }}
                    </style>
                </head>
                <body>{}</body>
            </html>", html_content
        );

        let response = Response::from_string(html_page)
            .with_header(Header::from_bytes(
                    b"Content-Type",
                    b"text/html; charset=UTF-8",
            ).unwrap());

        let _ = request.respond(response);
    }
}
