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
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Ref, ref } from "vue";

const allInjected: Ref<null | InjectedExtension[]> = ref(null);
const allAccounts: Ref<null | InjectedAccountWithMeta[]> = ref(null);
async function accounts() {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });
  allInjected.value = await web3Enable("my cool dapp");
  allAccounts.value = await web3Accounts();
  const SENDER = "5DTestUPts3kjeXSTMyerHihn1uwMfLj8vU8sqF7qYrFabHE";
  const injector = await web3FromAddress(SENDER);
  api.tx.balances
    .transfer("5C5555yEXUcmEJ5kkcCMvdZjUo7NGJiQJMS7vZXEeoMhj3VQ", 123456)
    .signAndSend(SENDER, { signer: injector.signer }, (status) => {
      console.log(status.isInBlock);
    });
}
accounts();
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
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3"
  >
    <h1>Accounts</h1>
    <pre class="font-light text-sm text-left w-full">
      {{ allAccounts }}
    </pre>
    <h1>Selected Account</h1>
    <pre class="font-light text-sm text-left w-full" v-if="allAccounts">
     {{ allAccounts[0] }}
    </pre>
    <h1>Metadata</h1>
    <pre class="font-light text-sm text-left w-full" v-if="allAccounts">
      {{ allAccounts[0].meta }}
    </pre>
  </div>
</template>
