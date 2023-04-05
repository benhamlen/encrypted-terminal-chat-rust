use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};
use std::net::{TcpListener, TcpStream};

fn main(partner_public: PublicKey) {
    let local_secret = EphemeralSecret::new(OsRng);
    let local_public = PublicKey::from(&local_secret);

    let local_shared_secret = local_secret.diffie_helman(&partner_public);

    // compare the client_shared_secret with the partner's shared secret, if they assert_eq
    // with both as_bytes() then the shared secret can be used for encryption
}

fn client() {

}

fn server(stream: TcpStream) {

}