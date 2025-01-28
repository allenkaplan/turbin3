import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
        mint: mint,
        name: "Salt Lake Rug Company",
        symbol: "SLCRUGS",
        uri: "https://devnet.irys.xyz/4Y5NdWc9Qz8Co5muccx4LzudjgkfdhGDcgYqc8NeBkmh",
        sellerFeeBasisPoints: percentAmount(5),
    });

    const cost = await tx.getRentCreatedOnChain(umi);
    const maxCost = 0.1 * 1e9; // 0.1 SOL in lamports
    
    console.log("Cost (lamports): ", cost.basisPoints);
    if (cost.basisPoints > maxCost) {
        throw new Error(`Cost to mint NFT (${cost.basisPoints.toString()} lamports) exceeds maximum allowed (${maxCost} lamports)`);
    }

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();

// 
// https://explorer.solana.com/tx/4huTtFRPRpD5SvNLixXtVEKvoT1Aa5xFufFSjzb6EUNpv5ZyNg8eH2jhtvq2VDT2wujYb6FVZfVjKKXdfqhV4bcX?cluster=devnet
// Mint Address:  AX7ddspkpgptarLGLDyVq4VBycNfHTYSyBciDNRpDBRP