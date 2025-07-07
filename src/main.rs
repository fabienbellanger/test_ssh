use ssh2::Session;
use std::{error::Error, io::Read, net::TcpStream, path::Path};

const USER: &str = "dev";
const PUBLIC_KEY_PATH: &str = "/Users/fabien/.ssh/id_ed25519.pub";
const PRIVATE_KEY_PATH: &str = "/Users/fabien/.ssh/id_ed25519";

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the local SSH server
    let tcp = TcpStream::connect("51.15.160.20:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    let passphrase = rpassword::prompt_password("Your passphrase: ")?;
    let passphrase = passphrase.trim();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_pubkey_file(
        USER,
        Some(&Path::new(PUBLIC_KEY_PATH)),
        &Path::new(PRIVATE_KEY_PATH),
        Some(passphrase),
    )?;

    // Make sure we succeeded
    assert!(sess.authenticated());
    // Try to run the ls command
    let mut channel = sess.channel_session()?;
    channel.exec("ls -h")?;
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    println!("{}", s.trim());
    channel.wait_close()?;
    println!("--------\nExit status: {}", channel.exit_status()?);

    Ok(())
}
