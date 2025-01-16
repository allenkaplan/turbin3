import wallet from "../dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("FXj4pzuG8Cakf8HJLBshLFPosYFK653DhNeun4wFuswz")
// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

// Your ata is: FXoQt722iXNVHQfsqzUeRa7ysz6aU5y2WRTGwaAwQ6na
// Your mint txid: 4pVorhwnkvLfCrp9NvE9jtAozrKQHkBESjF3Am7taaJgn1qAtyWdNRSxy2SWaF2VerNT9z5e7RXoxqXm6P3txpnz  

(async () => {
    try {
        // Start here
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint: mint,
            mintAuthority: signer,
        }

        let data: DataV2Args = {
            name: "turbin3",
            symbol: "TRB",
            uri: "https://arweave.net/",
            sellerFeeBasisPoints: 500,
            creators: null,
            collection: null,
            uses: null,
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data:data,
            isMutable: false,
            collectionDetails: null,
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// result
// 4XFNVVBBaLstwdnQZSn7ZsnKm7uEsRELBMrpYmFrwpsypy3szLuUhBPSoa8cNwp8si2jEC6gtfiHUPUa8P1QmZdK