const anchor = require('@project-serum/anchor');

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const idl = JSON.parse(require('fs').readFileSync('./target/idl/solivium.json'));
  const program = new anchor.Program(idl, "YourProgramIdHere");

  console.log("Deploying program...");
  // Add your deployment logic
}

main();
