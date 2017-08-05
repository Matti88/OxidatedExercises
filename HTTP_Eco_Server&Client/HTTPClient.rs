use  std::net::TcpStream;
use  std::time::Duration;

use std::io::Write;
use std::io::Read;

fn main() {
    let un_secondo = Duration::new(1, 0);
    let porta = ::std::env::args().nth(1).unwrap();
    let ip = "127.0.0.1";
    let mio_bind = ip.to_string() + ":" + &porta;
    let mio_bind_finale: &str = &mio_bind;
    

    let mut buf: [u8; 20] = [0; 20];

    let stream = TcpStream::connect(mio_bind_finale);
    std::thread::sleep(un_secondo);
    println!("info di stream: {:?}", stream );

    let mut mio_stream = stream.unwrap();
    mio_stream.write(b"Ciao Mondo\r\n").unwrap();

    let _ =match mio_stream.read(&mut buf){Err(e) => panic!("Errore: {}", e),
                             Ok(m)=>{if m == 0 {println!("Lettura fallita!");
                             }
                             println!("Letti: {} caratteri", m); m
                             }, };
                             println!("{:?}", &buf);

}