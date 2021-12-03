const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("Starting test...");

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Myepicproject;

  // create account keypair
  const baseAccount = anchor.web3.Keypair.generate();

  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("your tx sig", tx);

  // fetch actual data from account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif Count: ", account.totalGifs.toString());

  await program.rpc.addGif(
    "https://media.giphy.com/media/TV8FByMBqsLsYOWflq/giphy.gif",
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    }
  );

  // retrieve gifCount again to report new count
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Updated Gif Count: ", account.totalGifs.toString());

  console.log("Gif List", account.gifList);

  // increment likes for gif
  // retrieve account again
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  // get gif_link for first gif
  let gif_link = account.gifList[0].gifLink;

  // increment likes for gif
  await program.rpc.incrementLikes(gif_link, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // get list of gifs again
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif List (likes: 1):", account.gifList);

  // decrement likes for gif
  // retrieve account again
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  // decrement likes for gif
  // use same gif_link as above
  await program.rpc.decrementLikes(gif_link, {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  // get list of gifs again
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif List (likes: 0):", account.gifList);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
};

runMain();
