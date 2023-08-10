> ❗ see state @ submission deadline use git checkout 5327dbe

> run `cargo doc --open` to see the same as the README.md with extras.

<div align="center">
<img src="./../../Logo.jpg" alt="logo" width="80" height="80" />
</div>

<h3 align="center">SwipePM - Ink Contract</h3>
  <p align="center">
    A social competition Sports Prediction App with Zeitgeist
    <br />
    <a href="https://www.figma.com/file/V1W0wKDqXxGYrEH5TFfzJL/SwipePM?type=design&node-id=0%3A1&mode=design&t=cq5XPPpWLxi543Po-1" name="Figma">Figma</a>
    ·
    <a href="https://youtu.be/pBcYVFuj9M0">Youtube</a>
    ·
    <a href="https://github.com/FrankBevr/SwipePM">Code</a>
  </p>
</div>

# football_match

`football_match` is a small smart contract.  
football_match contains the logic for [SwipePM Dapp](https://github.com/FrankBevr/SwipePM).  
SwipePM is a betting dapp.  
It allows to bet on Manchester or Chelsea.

## Overview (Userflow & Logic)

1. **Alice** instantiates the contract.
2. **Alice** becomes the admin.
3. **Bob** uses the frontend and bets on Manchester.
4. **Charlie** uses the frontend and isn't allowed to bet on Manchester.
5. **Charlie** bets on Chelsea.
6. The amount of the bet is controlled by the frontend.
7. **Alice**, the admin, calls `set_winner` with the winner value.  
   If Manchester won the value has to be 1  
   If Chelsea won the value has to be 2  
   The Default value is 0
8. **Alice** sends value of 1 via `set_winner`.
9. Manchester won.
10. **Bob**, the manchester better, recieves all funds.
11. **Alice** calls `restart_match`, if a new manchester vs chelsea game is happening.  
     `restart_match` resets all values, expect the current admin.
12. **Alice** will be busy in the upcomming match.
13. **Alice** calls `change_admin` and sets **Django** as the new admin.
14. **Djange** is excited.

## Project Structure

```
.
|-- docs
|-- impls
|-- traits
|-- libs
|-- lib.rs
|-- Cargo.toml
|-- target
```

The folder `docs` holds small chunks of documentation.  
The folder `impls` holds the contract.  
The folder `traits` holds the interface which the contract implements.  
The folder `libs` holds the small utilities.  
The file `lib.rs` is the entry point.  
The file `Cargo.toml` declares all external depencies.  
The folder `target` is the folder which holds the buildt contract and the documentation.

## The football_match Contract

The contract holds the following data, named `GameData`

- A winning team represented by `winning_team`
- An admin represented by `admin`
- A chelsea better represented by `particpant_chelsea`
- A manchester better represented by `particpant_manchester`
- A chelsea better checker represented by `particpant_chelsea_is_set`
- A manchester better checker represented by `particpant_manchester_is_set`

The contracts implentent the following function for its Data

- A get_game function which sends eventbased the actual state
- A set_winner function which sets the winner
- A set_particpant_chelsea which set the chelsea better
- A set_particpant_manchester which set the manchester better
- A change_admin which changes the admin
- A restart_match which restarts the match.

## Usage

- **[Compile](#compile)**
- **[Deploy](#deploy)**
- **[Frontend](#frontend)**
  - **[Call Chain](#call-chain)**
  - **[Call Contract](#call-contract)**
  - **[Call Contract on frontend](#call-contract-on-frontend)**

<details>
<summary>Expand Me</summary>
   
   ### Compile
   
   1. Open up a terminal.
   2. cd in the football_match folder.
   3. make checks
      - `substrate-contracts-node --version`: v0.3 should be good
      - `cargo contract --version`: v3.0 should be good
      - `rustup show`: nightly-2022-12-23 works great
   4. run `cargo contract build`
   5. in `./target/ink/` you find all your needs
      - football_match.json holds the metadata
      - football_match.wasm holds the code
      - football_match.contract which holds metadata and code
   
   ### Deploy
   
   1. In Terminal A run `cargo substrate-contracts-node --dev`
   2. In Terminal B run `cargo contract instantiate --suri //Alice -x`
   3. Copy the deployed contract Addreess & codehash
   
   ### Frontend
   
   #### Call Chain
   
   1. In Terminal C run `cd ~` to go to your home Folder.
   2. run `mkdir frontend` to create a frontendfolder
   3. run `cd frontend` to change directory.
   4. make checks
      - `node --version`: version 18 should be good
      - `npm --version`: 9.6 should be good.
      - `code --version`: 1.8 should be good
   5. run `npm init -y`
   6. run `npm install @polkadot/api @polkadot/api-contract -D ts-node typescript`
   7. run `touch  call-substrate-contracts-node.ts`
   8. code call-substrate-contracts-node.ts
   9. paste the following in
      <details>
        <summary>show code</summary>
   
      ```ts
      import { WsProvider, ApiPromise } from "@polkadot/api";
   
      async function main() {
        const wsProvider = new WsProvider();
        const api = await ApiPromise.create({ provider: wsProvider });
        console.log(`The runtime version is the following ${api.genesisHash}`);
        process.exit(1);
      }
      main();
      ```
   
      </details>
   
   10. run `npx ts-node ./call-substrate-contracts-node.ts`
       <details>
         <summary>show output</summary>
   
       ```
       The runtimeversion is the following 0xcdb32a84e7bda0c3068b073089c6ec636f75a664939b878157a8776005a60af8
       ```
   
      </details>
   
   11. Lets make query
   12. overpaste the following in
         <details>
           <summary>show code</summary>
   
           ```ts
   
       import { WsProvider, ApiPromise } from '@polkadot/api'
       async function main(){
       const wsProvider = new WsProvider()
       const api = await ApiPromise.create({provider: wsProvider})
       console.log(api.query)
       process.exit(1)
       }
       main()
   
       ```
       </details>
   
       ```
   
   13. Now you see all that stuff, which is queryable. There is also a field. contracts. we will call contract.
   14. overpaste the following in
        <details>
          <summary>show code</summary>
   
       ```ts
       import { WsProvider, ApiPromise } from "@polkadot/api";
       async function main() {
         const wsProvider = new WsProvider();
         const api = await ApiPromise.create({ provider: wsProvider });
         console.log(api.query.contracts);
         process.exit(1);
       }
       main();
       ```
   
      </details>
   
   15. run `npx ts-node ./call-substrate-contracts-node.ts`
   16. Now we see clearer that there is a function, which calls contractInfoOf. Lets use it.
   17. overpaste the following in
        <details>
          <summary>show code</summary>
   
       ```ts
       import { WsProvider, ApiPromise } from "@polkadot/api";
   
       const contractAddress = "PASTE IN PREVIOUSLY COPIED CONTRACT ADDRESS";
       const codeHash = "PASTE IN PREVIOUSLY COPIED CODEHASH";
   
       async function main() {
         const wsProvider = new WsProvider();
         const api = await ApiPromise.create({ provider: wsProvider });
         const queryContract = await api.query.contracts.contractInfoOf(
           contractAddress,
         );
         const queryContractHumanVersion: any = queryContract.toHuman();
         console.log(queryContractHumanVersion);
         const queryCodeHash = queryContractHumanVersion!["codeHash"] as any;
         if (queryCodeHash === codeHash) {
           console.log("YES, thats actually our deployed contract");
         }
         process.exit(1);
       }
       ```
   
      </details>
   
   18. run `npx ts-node ./call-substrate-contracts-node.ts`
       <details>
       <summary>show output</summary>
   
       ```
       {
         trieId: '0xb81b26d0e34675dc667529bfccc57090f09c6e659f1a0a3418fb4a0d278b3665',
         depositAccount: '5GGpNGyDAzPpoxiFxmcmNQ5zJfvU54sPdSpQhVao7bz9xeMk',
         codeHash: '0x47538cdcab4785f60496e3c44dd239c868486fea6e94f121b81cf9779447ee50',
         storageBytes: '99',
         storageItems: '1',
         storageByteDeposit: '495,000,000',
         storageItemDeposit: '100,000,000,000',
         storageBaseDeposit: '101,765,000,000'
       }
       YES, thats actually our deployed contract
       ```
   
       </details>
   
   #### Call Contract
   
   1. run `touch  call-contract.ts`
   2. copy the metadata in the fronent folder. The metadata is called `football_match.json`. You can find it under `./target/ink` in the football_match folder, where the rust code sits.
   3. run `npx tsc --init`
   4. run `code tsconfig.json`
   5. uncomment line which says resolveJsonModule
   6. close the editor
   7. run `code call-contract.ts`
   8. paste the following in
        <details>
          <summary>show code</summary>
   
      ```ts
      import { WsProvider, ApiPromise } from "@polkadot/api";
      import { ContractPromise } from "@polkadot/api-contract";
      import metadata from "./football_match.json";
   
      async function main() {
        const wsProvider = new WsProvider();
        const api = await ApiPromise.create({ provider: wsProvider });
   
        const address = "PASTE YOUR COPIED CONTRACT ADDRESS";
        const contract = new ContractPromise(api, metadata, address);
        console.log(contract);
   
        process.exit(1);
      }
      main();
      ```
   
      </details>
   
   9. run `npx ts-node call-contract.ts`
   10. There you can see a bunch of stuff. And somehwere you see stuff like `footballMatch::getGame`. Hehe thats great. Hehe thats great. Next one.
   11. Paste the following in
        <details>
          <summary>show code</summary>
   
       ```ts
       import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
       import { ContractPromise } from "@polkadot/api-contract";
       import metadata from "./football_match.json";
       import { BN, BN_ONE } from "@polkadot/util";
   
       async function main() {
         const wsProvider = new WsProvider();
         const api = await ApiPromise.create({ provider: wsProvider });
   
         const address = "PASTE YOUR COPIED CONTRACT ADDRESS";
         const contract = new ContractPromise(api, metadata, address);
   
         const keyring = new Keyring({ type: "sr25519" });
         const bob = keyring.addFromUri("//Bob", { name: "Bob" });
   
         const storageDepositLimit = null;
         const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
         const PROOFSIZE = new BN(1_000_000);
   
         const { gasRequired } = await contract.query[
           "footballMatch::setParticpantChelsea"
         ](bob.address, {
           gasLimit: api?.registry.createType("WeightV2", {
             refTime: MAX_CALL_WEIGHT,
             proofSize: PROOFSIZE,
           }) as any,
           storageDepositLimit,
         });
   
         const gasLimit = api?.registry.createType("WeightV2", gasRequired) as any;
   
         await contract.tx["footballMatch::setParticpantChelsea"]({
           gasLimit,
           storageDepositLimit,
         }).signAndSend(bob, async (res) => {
           if (res.isInBlock) {
             console.log("Bob is has betted on chelsea");
             process.exit(1);
           }
         });
       }
       main();
       ```
   
      </details>
   
   11. run `npx ts-node call-contract.ts`
       > The output should be something like this:  
       > Bob is has betted on chelsea
   12. Hurei that how you interact with it. Change "footballMatch::setParticpantChelsea" to whatever function you like. Run the previous code example to see what is available and copy the naming correctly. Important is the correct gas Calculation and setting the weights. Kind of a pitfall. Now you know and can google correctly.
   
   #### Call Contract on Frontend
   
   1. run `touch index.html`
   2. npm install -D vite
   3. code `index.html`
   4. Paste the following in
        <details>
          <summary>show code</summary>
   
      ```html
      <html>
        <head></head>
        <body>
          <p>Hello world</p>
          <script type="module" src="./call-contract.ts"></script>
        </body>
      </html>
      ```
   
      </details>
   
   5. run `npx vite `
   6. modify callContract.ts, overpase the following.
        <details>
          <summary>show code</summary>
   
      ```ts
      import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
      import { ContractPromise } from "@polkadot/api-contract";
      import metadata from "./football_match.json";
      import { BN, BN_ONE } from "@polkadot/util";
   
      async function main() {
        console.log("hhelo");
        const wsProvider = new WsProvider();
        const api = await ApiPromise.create({ provider: wsProvider });
   
        const address = "5D6DbcxLogP2ThyaKvaHP1j2BdM9x76xX7QVT1uTcXoCRiEG";
        const contract = new ContractPromise(api, metadata, address);
   
        const keyring = new Keyring({ type: "sr25519" });
        const bob = keyring.addFromUri("//Bob", { name: "Bob" });
   
        const storageDepositLimit = null;
        const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
        const PROOFSIZE = new BN(1_000_000);
   
        const { gasRequired } = await contract.query[
          "footballMatch::setParticpantChelsea"
        ](bob.address, {
          gasLimit: api?.registry.createType("WeightV2", {
            refTime: MAX_CALL_WEIGHT,
            proofSize: PROOFSIZE,
          }) as any,
          storageDepositLimit,
        });
   
        const gasLimit = api?.registry.createType("WeightV2", gasRequired) as any;
   
        await contract.tx["footballMatch::setParticpantChelsea"]({
          gasLimit,
          storageDepositLimit,
        }).signAndSend(bob, async (res) => {
          if (res.isInBlock) {
            console.log("Bob is has betted on chelsea");
          }
        });
      }
      main();
      ```
   
      </details>
   
   7. open up localhost:5173 and "tada". Thats it.
   8. if you want to make on Button click, then just overpaste the index.html and add in ./call-contract.ts an export statement.
        <details>
          <summary>show code</summary>
   
      ```html
      <html>
        <head></head>
        <body>
          <p>Hello world</p>
          <button id="callMainButton">Call call-contract</button>
          <script type="module">
            import { main } from "./call-contract.ts";
            const callMainButton = document.getElementById("callMainButton");
            callMainButton.addEventListener("click", async () => {
              await main();
            });
          </script>
        </body>
      </html>
      ```
   
      </details>
   
   9. Now You can make button clicky.
   10. Have fun with the swipePM Contract.  
       I felt the need for a quick rundown of how to use it.
   
<details>
