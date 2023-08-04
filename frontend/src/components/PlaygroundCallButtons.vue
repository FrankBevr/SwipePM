<script setup lang="ts">
import GetGameButton from "./GetGameButton.vue";
import SetWinnerButton from "./SetWinnerButton.vue";
import SetParticipantChelseaButton from "./SetParticipantChelseaButton.vue";
import SetParticpantManchesterButton from "./SetParticipantManchesterButton.vue";

import {
  web3Accounts,
  web3Enable,
  web3FromAddress,
} from "@polkadot/extension-dapp";
import type {
  InjectedExtension,
  InjectedAccountWithMeta,
} from "@polkadot/extension-inject/types";
import { Ref, ref } from "vue";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { store } from "../store/store";

const allInjected: Ref<null | InjectedExtension[]> = ref(null);
const allAccounts: Ref<null | InjectedAccountWithMeta[]> = ref(null);
const selectedAccount: Ref<null | InjectedAccountWithMeta> = ref(null);
const injector: Ref<null | InjectedExtension> = ref(null);
const successTransaction: Ref<boolean> = ref(false);

async function get_accounts() {
  allInjected.value = await web3Enable("my cool dapp");
  allAccounts.value = await web3Accounts();
}
async function select_account(index: number) {
  allInjected.value = await web3Enable("my cool dapp");
  allAccounts.value = await web3Accounts();
  selectedAccount.value = allAccounts.value[index];
}
async function send_transaction() {
  allInjected.value = await web3Enable("my cool dapp");
  allAccounts.value = await web3Accounts();
  selectedAccount.value = allAccounts.value[3];

  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const SENDER = selectedAccount.value.address;
  injector.value = await web3FromAddress(SENDER);

  api.tx.balances
    .transfer("5C5555yEXUcmEJ5kkcCMvdZjUo7NGJiQJMS7vZXEeoMhj3VQ", 123456)
    .signAndSend(SENDER, { signer: injector.value.signer }, (status) => {
      successTransaction.value = status.isInBlock;
    });
}
</script>

<template>
  <h1 class="flex flex-justify-center p-10 underline">
    Playground Call Buttons
  </h1>
  <GetGameButton></GetGameButton>
  <SetWinnerButton></SetWinnerButton>
  <SetParticipantChelseaButton></SetParticipantChelseaButton>
  <SetParticpantManchesterButton></SetParticpantManchesterButton>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline outline-1 items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none">
    <p class="leading-relaxed">
      <strong>get_accounts</strong> gets all the accounts which are injected
    </p>
    <button
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="get_accounts">
      get_accounts
    </button>

    <p class="leading-relaxed">
      <strong>select_account</strong> gets all the accounts and sets the account
      based on the chosen index.
    </p>
    <button
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="select_account(3)">
      select_account(3)
    </button>
    <p class="leading-relaxed">
      <strong>send_transaction()</strong> sends transaction to address <br />
      gets all the accounts which are injected. <br />
      selects third account which is injected. <br />
      sets up api <br />
      send Transaction to 5C5555yEXUcmEJ5kkcCMvdZjUo7NGJiQJMS7vZXEeoMhj3VQ
    </p>
    <button
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="send_transaction">
      send_transaction
    </button>
    <p v-if="successTransaction">Its was successfull</p>
  </div>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline outline-1 items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none">
    <p class="leading-relaxed">show all Accounts</p>
    <pre class="font-light text-10px text-left w-full">
      {{ allAccounts }}
    </pre>
    <p class="leading-relaxed">show selected Accounts</p>
    <pre class="font-light text-10px text-left w-full" v-if="allAccounts">
     {{ allAccounts[0] }}
    </pre>
    <p class="leading-relaxed">shows metadata of an selected Accounts</p>
    <pre class="font-light text-10px text-left w-full" v-if="allAccounts">
      {{ allAccounts[0].meta }}
    </pre>
  </div>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light items-stretch">
    <p class="leading-relaxed">
      <strong>store.contractAddress</strong> holds the contract address
      globally.
      <br />
      After redeploy it allows to reset the contract to the new value.
    </p>
    <form @submit.prevent>
      <div>
        <label for="address" class="block text-sm font-light">Contract Address</label>
        <input
          class="text-white w-full leading-tight focus:outline-none focus:shadow-outline opacity-20 bg-black leading-5 m-0 p-10px"
          type="text" name="address" v-model="store.contractAddress" />
      </div>
      SwipePM's contract address is {{ store.contractAddress }}
    </form>
  </div>
</template>
