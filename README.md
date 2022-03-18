# Solana Demos #



To compile the program, you need to install the [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) with the following command.

```
sh -c "$(curl -sSfL https://release.solana.com/v1.9.9/install)" 
```



To confirm that the Solana CLI is installed successfully, we can check its version with the following command.

```
solana --version 
```



Note that [Rust](https://rustup.rs/) and [NodeJS](https://nodejs.org/en/) are aslo required.

To compile the program, we can use the following command. 

```
cargo build-bpf --manifest-path=./src/program-rust/Cargo.toml --bpf-out-dir=dist/program 
```

The compiled program will be generated under the directory specified in `--bpf-out-dir`.



### Select a cluster ###

To connect with the Devnet, use the following command. 

```
solana config set --url https://api.devnet.solana.com 
```



### Deploy the program ###

```
solana program deploy dist/program/helloworld.so 
```



### Send Transaction ### 

To run the client, you'll have to install the required dependcies. 

```
npm install  
```



Run the scipt 

```
npm run start  
```



For more details, please refer to our medium article. 

```
https://medium.com/@blocksecteam/secure-the-solana-ecosystem-1-hello-solana-bb7ecc1e6b21
```

