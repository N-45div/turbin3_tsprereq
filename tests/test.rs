import bs58 from 'bs58'
import * as prompt from 'prompt-sync

#[test]
fn base58_to_wallet() {
    println!("Enter your wallet:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    let wallet: Vec<u8> =
    vec![47,29,59,246,94,113,202,21,193,147,45,164,166,38,185,86,52,64,155,170,34,191,102,115,178,181,218,56,212,14,226,113,197,7,190,216,254,246,146,236,27,155,163,131,51,43,95,158,169,226,230,240,41,78,222,150,207,130,166,167,70,189,98,96];
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}