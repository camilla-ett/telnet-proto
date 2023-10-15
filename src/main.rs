use std::io::Write;
use std::net::TcpListener;

static KASEISKI: &[u8] = "
__  ___      ___           _______. _______  __          _______. __  ___  __  
|  |/  /     /   \\         /       ||   ____||  |        /       ||  |/  / |  | 
|  '  /     /  ^  \\       |   (----`|  |__   |  |       |   (----`|  '  /  |  | 
|    <     /  /_\\  \\       \\   \\    |   __|  |  |        \\   \\    |    <   |  | 
|  .  \\   /  _____  \\  .----)   |   |  |____ |  |  __.----)   |   |  .  \\  |  | 
|__|\\__\\ /__/     \\__\\ |_______/    |_______||__| (__)_______/    |__|\\__\\ |__| 
                                                                                
".as_bytes();

fn main() {
    // リスナーソケットを用意
    let hostname = "127.0.0.1:2323";
    
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
        match tcp_stream.write(&KASEISKI) {
            Result::Ok(_) => {},
            Result::Err(e) => println!("{}", e)
        }
    }

    // へえええスコープを抜けたtcp_stream は自動的にクローズされる

}
