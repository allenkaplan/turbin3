import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("FXj4pzuG8Cakf8HJLBshLFPosYFK653DhNeun4wFuswz");

// Recipient address
const to = new PublicKey("83wPgXhEscwj6aMZy3cDZB98HE9HS6E9ax4PcitMiSA2");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAccount = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        console.log(`Your fromTokenAccount is: ${fromTokenAccount.address.toBase58()}`);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);
        console.log(`Your toTokenAccount is: ${toTokenAccount.address.toBase58()}`);

        // Transfer the new token to the "toTokenAccount" we just created
        const transferTx = await transfer(connection, keypair, fromTokenAccount.address, toTokenAccount.address, keypair.publicKey, 100);
        console.log(`Your transfer txid: ${transferTx}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();


// Your fromTokenAccount is: FXoQt722iXNVHQfsqzUeRa7ysz6aU5y2WRTGwaAwQ6na
// Your toTokenAccount is: 9GYveUok2rtauUfXeM6W8FrGxxgsJjHneQtQSMY9AcLY
// Your transfer txid: 3UgaJ9qBLkqcURaLJ76fMKXV3HjHBjN1ywbAGrwwETzPxLUB7WzMfFfWaberJbDoMndBktjkJPYrTRAUJzDi9F9r