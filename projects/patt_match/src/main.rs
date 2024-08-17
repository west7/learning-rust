#![allow(unused)]

enum Http {
    Status(u16),
    Ok,
    NotFound,
    ServerError,
}

fn main() {
    let response = Http::Status(404);

    let status = match response {
        Http::Status(code) => code,
        Http::Ok => 200,
        Http::NotFound => 404,
        Http::ServerError => 500,
    };

    if let 200 = status {
        println!("Status code: {}\n", 200);
    }

    let mut responses = vec![
        Http::NotFound,
        Http::NotFound,
        Http::ServerError,
        Http::Ok,
        Http::Ok,
        Http::Status(500),
        Http::Status(503),
        Http::Status(100),
        Http::Status(201),
    ];

    while let Some(Http::Status(code)) = responses.pop() {
        println!("Status code: {}, {}", code, responses.len());
    } 
    // O problema deste loop é que ele consome o valor Http::Ok, o segundo loop não deveria funcionar, porém como o valor é consumido ele funciona.

    println!("");

    while let Some(Http:: NotFound | Http::ServerError) = responses.pop() {
        println!("Unexpected status code {}", responses.len());
    }   
}
