<script setup lang="ts">
import GetGameButton from "./GetGameButton.vue";
import SetWinnerButton from "./SetWinnerButton.vue";
import SetParticipantChelseaButton from "./SetParticipantChelseaButton.vue";

import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import type { WeightV2 } from "@polkadot/types/interfaces";
import * as abi from "./football_match.json";
import { BN, BN_ONE } from "@polkadot/util";
import { Ref, ref } from "vue";

const participantManchester: Ref<null | string> = ref(null);

async function set_participant_manchester() {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const charlie = keyring.addFromUri("//Charlie", { name: "Charlie" });
  const address = "5EkfPTWeH5UmTEAHQGDaGvNXGU3EGzQsTSXKiuSk6r9ADXZf";

  const contract = new ContractPromise(api, abi, address);

  const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
  const PROOFSIZE = new BN(1_000_000);
  const storageDepositLimit = null;
  const { gasRequired } = await contract.query[
    "footballMatch::setParticpantManchester"
  ](charlie.address, {
    gasLimit: api.registry.createType("WeightV2", {
      refTime: MAX_CALL_WEIGHT,
      proofSize: PROOFSIZE,
    }) as WeightV2,
    storageDepositLimit,
  });

  const gasLimit = api.registry.createType("WeightV2", gasRequired) as WeightV2;
  await contract.tx["footballMatch::setParticpantManchester"]({
    gasLimit,
    storageDepositLimit,
  }).signAndSend(charlie, async (res) => {
    if (res.isError) {
      console.log("ERROR");
    }
    if (res.isInBlock) {
      participantManchester.value = "Charlie";
    }
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
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3">
    <p class="leading-relaxed">
      <strong>footballMatch::set_participant_manchester</strong> sets the
      participant who bets on manchester.
      <br />
      The caller of it bets on manchester
      <br />
      The address of the caller will be safed.
    </p>
    <button @click="set_participant_manchester" type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
      setParticpantManchester
    </button>
    <p v-if="participantManchester === null">None bid on Manchester</p>
    <p v-if="participantManchester !== null">
      {{ participantManchester }} bid on Manchester
    </p>
  </div>
  <h1 class="flex flex-justify-center">Accounts</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      <!-- {{ store.accounts }} -->
    </pre>
  </div>
  <h1 class="flex flex-justify-center">selectedAccount</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      <!-- {{ store.selectedAccount }} -->
    </pre>
  </div>
  <h1 class="flex flex-justify-center">Metadata</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      <!-- {{ store.metadata }} -->
    </pre>
  </div>
</template>
