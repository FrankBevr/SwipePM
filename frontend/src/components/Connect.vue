<script setup lang="ts">
import { store } from "../store/store";
import {
  web3Accounts,
  web3Enable,
  web3FromAddress,
} from "@polkadot/extension-dapp";
async function connect_account(index: number) {
  store.allInjected = await web3Enable("my cool dapp");
  store.allAccounts = await web3Accounts();
  store.selectedAccount = store.allAccounts[index];
  store.injector = await web3FromAddress(store.selectedAccount.address);
  store.isConntected = true;
  console.log(store.injector);
}
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10">
    <p class="leading-relaxed">
      <strong>Connect</strong> and become the admin of the game.
      <br />
      You, as the admin, has the important role to set the winner of the game.
      <br />
    <div>For Demo Sake:
      <ul class="text-left">
        <li>Alice will pick Manchester </li>
        <li>Bob will pick Chelsea.</li>
      </ul>
    </div>
    It would be too annoying to switch accounts.
    <br />
    You will see what it means in a second.
    <br />
    <div><strong>Now connect</strong>, mister Admin.</div>
    </p>
    <button @click="connect_account(3)"
      class="bg-#CCCCCC text-2xl p-5 b-rd-3 hover:bg-#ffffff outline outline-1 font-light border-none">
      Connect
    </button>
  </div>
</template>
