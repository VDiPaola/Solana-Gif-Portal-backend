const anchor = require('@project-serum/anchor');

// Need the system program
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  // Create and set the provider.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();


  const program = anchor.workspace.GifPortalBackend;
  
  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // increment
  await program.rpc.addGif("http://someoihtdsf.com",{
    accounts:{
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey
    }
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList)

  //upvote gif
  await program.rpc.incrementGifUpvotes("0",{
    accounts:{
      baseAccount: baseAccount.publicKey
    }
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF List', account.gifList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
