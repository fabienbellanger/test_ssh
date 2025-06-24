use ssh2::Session;
use std::{io::Read, net::TcpStream, path::Path};

fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("51.15.160.20:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    let passphrase =
        rpassword::prompt_password("Your passphrase: ").expect("Unable to read the password");
    let passphrase = passphrase.trim();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file(
        "dev",
        Some(&Path::new("/Users/fabien/.ssh/id_ed25519.pub")),
        &Path::new("/Users/fabien/.ssh/id_ed25519"),
        Some(passphrase),
    )
    .unwrap();

    // Make sure we succeeded
    assert!(sess.authenticated());

    println!("Connection success!");

    // Try to run the ls command
    println!("\nRun ls command...");
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls -lh").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("ls result:\n{}", s);
    channel.wait_close().unwrap();
    println!("--------\nExit status: {}", channel.exit_status().unwrap());

    // Try pwd command
    let mut channel = sess.channel_session().unwrap();
    channel.exec("pwd").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("pwd result:\n{}", s);
    channel.wait_close().unwrap();
    println!("--------\nExit status: {}", channel.exit_status().unwrap());
}
