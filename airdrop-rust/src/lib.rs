mod programs;

#[cfg(test)]
mod tests {
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
        pubkey::Pubkey,
        message::Message,
        system_program,
    };
    use solana_client::rpc_client::RpcClient;   

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }


    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn airdrop() {

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find dev-wallet.json");

        // Create a client
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
        };
}
        
        

    use solana_program::system_instruction::transfer;
    use std::str::FromStr;

    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ").unwrap();
   
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        
        let transaction = Transaction::new_signed_with_payer(   
            &[transfer(
                &keypair.pubkey(), 
                &to_pubkey, 
                1000000000
            )], 
            Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        );

        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        println!("success check out tx here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

        // Get balance of dev wallet
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance)], 
            Some(&keypair.pubkey()), &recent_blockhash
        );

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees 
        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator"); 
        println!("Fee for the transaction: {}", fee);

       // Deduct fee from lamports amount and create a TX with correct balance let transaction =
        let transaction2 = Transaction::new_signed_with_payer(
            &[transfer( &keypair.pubkey(), &to_pubkey, balance - fee)], 
            Some(&keypair.pubkey()), 
            &vec![&keypair], 
            recent_blockhash
        );

        // Send the transaction
        let signature2 = rpc_client.send_and_confirm_transaction(&transaction2).expect("Failed to send transaction");
        println!("success check out tx here: https://explorer.solana.com/tx/{}?cluster=devnet", signature2);
    }

    use bs58;
    use std::io::{self, BufRead};

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:"); 
        let stdin = io::stdin(); 
        println!("Your private key is:");
        let wallet = stdin.lock().lines().next().unwrap().unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>(); 
        
        // Create keypair from the bytes
        let keypair = Keypair::from_bytes(&wallet).unwrap();
        
        println!("\nYour private key (base58) is:");
        let base58 = bs58::encode(&wallet).into_string(); 
        println!("{}", base58);
        
        println!("\nYour public key (wallet address) is:");
        println!("{}", keypair.pubkey().to_string());
    }

    use crate::programs::turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};


    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        // Define our instruction data 
        let args = CompleteArgs {
            github: b"allenkaplan".to_vec() };
    // Get recent blockhash
    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    // Now we can invoke the "complete" function 
    let transaction = Turbin3PrereqProgram::complete(
        &[&signer.pubkey(), 
        &prereq,
         &system_program::id()
         ], &args, Some(&signer.pubkey()),
        &[&signer],
        blockhash );
    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
    println!("Success! Check out your TX here:");
    println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);

    }
}
