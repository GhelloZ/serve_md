use std::fs;
use std::path::Path;
use tiny_http::{Server, Response, Header};
use markdown::to_html;
user clap::Parser;

fn main() {
    let server = Server::http("0.0.0.0:3000").unwrap();
    let file_path = Path::new("./sample.md");

    println!("Serving markdown page at http://127.0.0.1:3000");

    for request in server.incoming_requests(){
        let markdown = fs::read_to_string(file_path).unwrap_or_else(|_| "# File not found".into());
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
