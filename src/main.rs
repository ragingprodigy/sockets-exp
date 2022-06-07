use std::time::SystemTime;

fn main() {
    println!("Hello, world!");

    let ctx = zmq::Context::new();
    let addr = "tcp://*:8888";

    let socket = ctx.socket(zmq::STREAM).unwrap();
    socket.bind(addr).unwrap();

    let start_time = SystemTime::now();
    let mut i = 0;

    loop {
        i += 1;
        // let data = socket.recv_msg(0).unwrap();
        let data = socket.recv_multipart(0).unwrap();
        println!(
            "#{:5} Elapsed: {:?}, Identity : {:?}, Message : {:?}",
            i,
            start_time.elapsed().unwrap(),
            // data,
            data[0],
            &data[1]
        );
    }
}
