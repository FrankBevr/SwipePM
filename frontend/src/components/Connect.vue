<script setup lang="ts">
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp";
import { onMounted } from "vue";
import { store } from "../store/store";

onMounted(async () => {
  //@ts-ignore
  const allInjected = await web3Enable("SwipePM");
  const allAccounts = await web3Accounts();
  store.accounts = allAccounts;
  store.selectedAccount = allAccounts[0];
});
const logMe = () => {
  console.log("Log Me");
  console.log(`Thats my selected Account: ${store.selectedAccount?.address}`);
};
</script>
<template>
  <div class="flex flex-justify-center mb-10 mt-10">
    <button
      @click="logMe"
      class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3 hover:bg-#ffffff"
    >
      Connect
    </button>
    <pre></pre>
  </div>
</template>
