use std::io::Write;
use std::net::TcpListener;

fn main() {
    // リスナーソケットを用意
    let hostname = "127.0.0.1:23";
    
    let listener = TcpListener::bind(hostname);
    if let Err(e) = listener {
        println!("Listen failed");
        println!("{}", e);
        return;
    }

    let l = listener.unwrap();

    // ここ後でループにする
    loop {
        let stream = l.accept();
        if let Err(e) = stream {
            println!("Accept Error!");
            println!("{}", e);
            return;
        }

        let mut tcp_stream = stream.unwrap().0;
        let buffer = "\nWelcome to kasei.ski\nGo https://kasei.ski/\n\n  Thank you.\n\n".as_bytes();
        match tcp_stream.write(&buffer) {
            Result::Ok(_) => {},
            Result::Err(e) => println!("{}", e)
        }
    }

    // へえええスコープを抜けたtcp_stream は自動的にクローズされる

}
