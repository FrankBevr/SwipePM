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
  <div class="flex flex-justify-center mb-10 mt-10">
    <button @click="connect_account(3)"
      class="bg-#CCCCCC text-2xl p-5 b-rd-3 hover:bg-#ffffff outline outline-1 font-light border-none">
      Connect
    </button>
    <pre></pre>
  </div>
</template>
