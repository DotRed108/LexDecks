use pasetors::keys::{AsymmetricKeyPair, Generate};
use pasetors::version4::V4;
use lex_decks::utils::back_utils::PasetoPrivateKey;


fn main() {
    let kp = AsymmetricKeyPair::<V4>::generate().unwrap();

    println!("{:?}", kp.public.as_bytes());
    println!("{}", PasetoPrivateKey::from_key(kp.secret).to_string());
}