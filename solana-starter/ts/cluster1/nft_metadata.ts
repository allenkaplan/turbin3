import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createGenericFileFromJson, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/DQi4HA6fMvzfLFUpppn4PUBy3z7MEUdEQiry572nExgx";
        const metadata = {
            name: "Salt Lake Rug Company",
            symbol: "SLCRUGS",
            description: "Salt Lake Rug Company is a collection of 100 unique rugs, each with its own story and history. The collection is a tribute to the rich culture and heritage of Salt Lake City, Utah.",
            image: image,
            attributes: [
                {trait_type: '?', value: '?'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "?"
                    },
                ]
            },
            creators: []
        };

        // Convert metadata to generic file
        const genericFile = createGenericFileFromJson(metadata, "metadata.json");

        // Upload metadata
        const [myUri] = await umi.uploader.upload([genericFile]);
        const correct_uri = myUri.replace("https://arweave.net/", "https://devnet.irys.xyz/");
        console.log("Your metadata URI: ", correct_uri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();


// Your metadata URI:  https://devnet.irys.xyz/4Y5NdWc9Qz8Co5muccx4LzudjgkfdhGDcgYqc8NeBkmh

