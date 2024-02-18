use std::{
    io::{self, Read},
    thread,
};

fn main() -> io::Result<()> {
    let mut i = thunder::Interface::new()?;
    let mut l1 = i.bind(8000)?;

    let jh1 = thread::spawn(move || {
        while let Ok(mut stream) = l1.accept() {
            eprintln!("got connection!");
            stream.shutdown(std::net::Shutdown::Write).unwrap();
            loop {
                let mut buf = [0u8; 512];
                let n = stream.read(&mut buf).unwrap();
                eprintln!("read {}bytes of data", n);
                if n == 0 {
                    eprintln!("no more data");
                    break;
                } else {
                    println!("{}", std::str::from_utf8(&buf[..n]).unwrap());
                }
            }
        }
    });

    jh1.join().unwrap();

    Ok(())
}
