# football_match

`football_match` is a small smart contract.\
football_match contains the logic for [SwipePM Dapp](https://github.com/FrankBevr/SwipePM).\
SwipePM is a betting dapp.\
It allows to bet on Manchester or Chelsea.\

## Overview (Userflow & Logic)
1. **Alice** instantiates the contract.
2. **Alice** becomes the admin.
3. **Bob** uses the frontend and bets on Manchester.
4. **Charlie** uses the frontend and isn't allowed to bet on Manchester.
5. **Charlie** bets on Chelsea.
6. The amount of the bet is controlled by the frontend.
7. **Alice**, the admin, calls `set_winner` with the winner value.\
    If Manchester won the value has to be 1\
    If Chelsea won the value has to be 2\
    The Default value is 0\
8. **Alice** sends value of 1 via `set_winner`.
9. Manchester won.
10. **Bob**, the manchester better, recieves all funds.
11. **Alice** calls `restart_match`, if a new manchester vs chelsea game is happening.\
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
    ```text
    import { WsProvider, ApiPromise } from '@polkadot/api'

    async function main(){
        const wsProvider = new WsProvider()
        const api = await ApiPromise.create({provider: wsProvider})
        console.log(`The runtime version is the following ${api.genesisHash}`)
        process.exit(1)
    }
    main()
    ```
10. run `npx ts-node ./call-substrate-contracts-node.ts`  
   > The output should be something like this:  
   > The runtimeversion is the following 0xcdb32a84e7bda0c3068b073089c6ec636f75a664939b878157a8776005a60af8  
11. Lets make query
12. overpaste the following in
     ```text
    import { WsProvider, ApiPromise } from '@polkadot/api'
    async function main(){
        const wsProvider = new WsProvider()
        const api = await ApiPromise.create({provider: wsProvider})
        console.log(api.query)
        process.exit(1)
    }
    main()
    ```
13. Now you see all that stuff, which is queryable. There is also a field. contracts. we will call contract.
14. overpaste the following in
     ```text
    import { WsProvider, ApiPromise } from '@polkadot/api'
    async function main(){
        const wsProvider = new WsProvider()
        const api = await ApiPromise.create({provider: wsProvider})
        console.log(api.query.contracts)
        process.exit(1)
    }
    main()
    ```
15. run `npx ts-node ./call-substrate-contracts-node.ts`  
16. Now we see clearer that there is a function, which calls contractInfoOf. Lets use it.
17. overpaste the following in
     ```text
     import { WsProvider, ApiPromise } from '@polkadot/api'

     const contractAddress = "PASTE IN PREVIOUSLY COPIED CONTRACT ADDRESS"
     const codeHash = "PASTE IN PREVIOUSLY COPIED CODEHASH"

     async function main() {
       const wsProvider = new WsProvider()
       const api = await ApiPromise.create({ provider: wsProvider })
       const queryContract = await api.query.contracts.contractInfoOf(contractAddress)
       const queryContractHumanVersion: any = queryContract.toHuman()
       console.log(queryContractHumanVersion)
       const queryCodeHash = queryContractHumanVersion!["codeHash"] as any
       if (queryCodeHash === codeHash) {
         console.log("YES, thats actually our deployed contract")
       }
       process.exit(1)
     }
     ```
18. run `npx ts-node ./call-substrate-contracts-node.ts`  
    > The output should be something like this:  
       ```text
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
#### Call Contract 
1. w.i.p

#### Call Contract on Frontend
1. w.i.p
