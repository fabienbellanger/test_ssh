use ssh2::Session;
use std::{io::Read, net::TcpStream, path::Path};

const ADDR: &str = "51.15.160.20:22";
const USER: &str = "dev";
const PUBLIC_KEY_PATH: &str = "/Users/fabien/.ssh/id_ed25519.pub";
const PRIVATE_KEY_PATH: &str = "/Users/fabien/.ssh/id_ed25519";

fn main() {
    // Connect to the local SSH server
    let tcp = TcpStream::connect(ADDR).expect("Unable to connect to the SSH server");
    let mut sess = Session::new().expect("Unable to create SSH session");
    sess.set_tcp_stream(tcp);
    sess.handshake().expect("Unable to perform SSH handshake");

    // Ask for the passphrase
    let passphrase =
        rpassword::prompt_password("Your passphrase: ").expect("Unable to read the password");
    let passphrase = passphrase.trim();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file(
        USER,
        Some(&Path::new(PUBLIC_KEY_PATH)),
        &Path::new(PRIVATE_KEY_PATH),
        Some(passphrase),
    )
    .expect("Failed to authenticate with public key");

    // Make sure we succeeded
    assert!(sess.authenticated());

    // Try to run the uname command
    let mut channel = sess.channel_session().expect("Unable to create a session channel");
    channel.exec("uname -a").expect("Unable to execute uname command");
    let mut s = String::new();
    channel.read_to_string(&mut s).expect("Unable to read uname command output");
    channel.wait_close().expect("Unable to close the channel");
    println!("\nuname command:\n--> {}", s.trim());
    println!("<-- Exit status: {}", channel.exit_status().expect("Unable to get exit status"));

    // Try pwd command
    let mut channel = sess.channel_session().expect("Unable to create a session channel");
    channel.exec("pwd").expect("Unable to execute pwd command");
    let mut s = String::new();
    channel.read_to_string(&mut s).expect("Unable to read pwd command output");
    channel.wait_close().expect("Unable to close the channel");
    println!("\npwd command:\n--> {}", s.trim());
    println!("<-- Exit status: {}", channel.exit_status().expect("Unable to get exit status"));
}
