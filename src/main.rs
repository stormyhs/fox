mod log;

fn main() {
    let msg = String::from("message");

    debug!("This is a debug {msg}");
    info!("This is an info {msg}");
    warn!("This is a warning {msg}");
    error!("This is an error {msg}");
    critical!("This is a critical {msg}");

    println!();

    sdebug!("This is a shortened debug {msg}");
    sinfo!("This is a shortened info {msg}");
    swarn!("This is a shortened warning {msg}");
    serror!("This is a shortened error {msg}");
    scritical!("This is a shortened critical {msg}");
}
