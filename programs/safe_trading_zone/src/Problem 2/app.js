const anchor = require("@project-serum/anchor"); // Import the anchor module
const {
  Connection,
  PublicKey,
  clusterApiUrl,
  Keypair,
} = require("@solana/web3.js"); // Import the web3 module to interact with the Solana blockchain network
const { SystemProgram } = anchor.web3; // Import the SystemProgram module from the anchor module to interact with the Solana system program

const programId = new PublicKey("Fg6PaFpoGXkYsidMpWxqSWY1hXkYsi1d1tQ1tQ1tQ1tQ"); // The program ID of the Safe Trading Zone program
const network = clusterApiUrl("devnet"); // The network to connect to
const connection = new Connection(network, "processed"); // Connect to the network using the connection object

const provider = anchor.AnchorProvider.local(); // Create a new AnchorProvider object using the local provider
anchor.setProvider(provider); // Set the provider for the anchor module to the local provider

const program = new anchor.Program(idl, programId, provider); // Create a new program object using the program ID and the provider

// Initialize the escrow account
async function initializeEscrow() {
  const seller = new PublicKey(document.getElementById("seller").value); // Get the seller's public key from the input field
  const safeZone = new PublicKey(document.getElementById("safeZone").value); // Get the safe zone's public key from the input field
  const notary = new PublicKey(document.getElementById("notary").value); // Get the notary's public key from the input field

  const escrowAccount = Keypair.generate(); // Generate a new keypair for the escrow account

  // Call the initialize method of the program to initialize the escrow account
  await program.rpc.initialize(seller, safeZone, notary, {
    accounts: {
      escrowAccount: escrowAccount.publicKey, // Pass the escrow account's public key as an account
      user: provider.wallet.publicKey, // Pass the user's public key as an account
      systemProgram: SystemProgram.programId, // Pass the system program ID as an account
    },
    signers: [escrowAccount], // Sign the transaction with the escrow account's keypair
  });

  console.log("Escrow initialized:", escrowAccount.publicKey.toString()); // Log the escrow account's public key to the console
}

// Deposit funds into the escrow account
async function depositFunds() {
  const amount = parseInt(document.getElementById("amount").value); // Get the amount to deposit from the input field
  const escrowAccount = new PublicKey("YOUR_ESCROW_ACCOUNT_PUBLIC_KEY"); // Get the escrow account's public key from the input field

  // Call the deposit method of the program to deposit funds into the escrow account
  await program.rpc.deposit(new anchor.BN(amount), {
    accounts: {
      escrowAccount: escrowAccount, // Pass the escrow account's public key as an account
      buyer: provider.wallet.publicKey, // Pass the buyer's public key as an account
      systemProgram: SystemProgram.programId, // Pass the system program ID as an account
    },
  });

  console.log("Funds deposited:", amount); // Log the amount deposited to the console
}

// Withdraw funds from the escrow account
async function confirmTransaction() {
  const escrowAccount = new PublicKey("YOUR_ESCROW_ACCOUNT_PUBLIC_KEY"); // Get the escrow account's public key from the input field
  const seller = new PublicKey("SELLER_PUBLIC_KEY"); // Get the seller's public key from the input field
  const safeZone = new PublicKey("SAFE_ZONE_PUBLIC_KEY"); // Get the safe zone's public key from the input field
  const notary = provider.wallet.publicKey; // Get the notary's public key from the provider's wallet

  // Call the confirm method of the program to confirm the transaction
  await program.rpc.confirm({
    accounts: {
      escrowAccount: escrowAccount, // Pass the escrow account's public key as an account
      buyer: provider.wallet.publicKey, // Pass the buyer's public key as an account
      notary: notary, // Pass the notary's public key as an account
      seller: seller, // Pass the seller's public key as an account
      safeZone: safeZone, // Pass the safe zone's public key as an account
      systemProgram: SystemProgram.programId, // Pass the system program ID as an account
    },
  });

  console.log("Transaction confirmed");
}
