<script setup lang="ts">
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp";
import { onMounted } from "vue";
let allAccounts: any;
let allInjected: any;

onMounted(async () => {
  allInjected = await web3Enable("SwipePM");
  allAccounts = await web3Accounts();
});
const logMe = () => {
  console.log("Log Me");
  console.log(allAccounts);
};
</script>
<template>
  <div class="flex flex-justify-center mb-10 mt-10">
    <button @click="logMe" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3 hover:bg-#ffffff">
      Connect
    </button>
    <pre></pre>
  </div>
</template>
