use  std::net::{TcpListener, TcpStream};
use  std::thread;

use std::io::Write;
use std::io::Read;

fn handle_request(mut stream: TcpStream){

    let mut buf;
    loop {
        buf = [0; 1024];
        let _ = match stream.read(&mut buf){
            Err(e) => panic!("Errore: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                println!("Letti: {} caratteri", m);
                m
            },
        };
        println!("Mando dati!");
        stream.write(b"Ciao anche a te!\r\n").unwrap();

    }
}

fn main() {

  
    let porta = ::std::env::args().nth(1).unwrap();
    let ip = "127.0.0.1";
    let mio_bind = ip.to_string() + ":" + &porta;
    let mio_bind_finale: &str = &mio_bind;
    
    let listener = TcpListener::bind(mio_bind_finale).unwrap();
    for stream in listener.incoming(){
        match stream{
            Err(e)=> {
                println!("(Fallito: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_request(stream)
                });
            }
        }
    }


}


















