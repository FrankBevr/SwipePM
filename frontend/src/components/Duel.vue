<script setup lang="ts">
import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { BN, BN_ONE } from "@polkadot/util";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { ContractPromise } from "@polkadot/api-contract";
import * as metadata from "./football_match.json";
import { store } from "../store/store";
import { ref, watch } from "vue";

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

const set_participant_chelsea = async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const bob = keyring.addFromUri("//Alice", { name: "Alice" });

  const address = store.contractAddress;
  const contract = new ContractPromise(api, metadata, address);

  const storageDepositLimit = null;

  const { gasRequired } = await contract.query[
    "footballMatch::setParticpantChelsea"
  ](bob.address, {
    gasLimit: api?.registry.createType("WeightV2", {
      refTime: MAX_CALL_WEIGHT,
      proofSize: PROOFSIZE,
    }) as WeightV2,
    storageDepositLimit,
  });

  const gasLimit = api?.registry.createType(
    "WeightV2",
    gasRequired,
  ) as WeightV2;

  await contract.tx["footballMatch::setParticpantChelsea"]({
    gasLimit,
    storageDepositLimit,
  }).signAndSend(bob, async (res) => {
    if (res.isInBlock && bob.meta.name) {
      store.participantChelsea = bob.meta.name;
    }
  });
};

const set_participant_manchester = async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const charlie = keyring.addFromUri("//Bob", { name: "Bob" });
  const address = store.contractAddress;

  const contract = new ContractPromise(api, metadata, address);

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
    if (res.isInBlock && charlie.meta.name) {
      store.participantManchester = charlie.meta.name;
    }
  });
};
/* Styling */
const manchesterStyle = ref(
  "flex flex-col outline outline-1 w-72 b-rd-3 bg-#FF8080 hover:bg-#FFa0a0 hover:px-9 duration-500 p-7 gap-5",
);
const chelseaStyle = ref(
  "flex flex-col outline outline-1 w-72 b-rd-3 bg-#80D4FF hover:bg-#a0F4FF hover:px-9 duration-500 p-7 gap-5",
);
watch(store, () => {
  if (store.participantChelsea !== "") {
    chelseaStyle.value =
      "flex flex-col outline outline-1 w-72 b-rd-3 bg-#dddddd p-7 gap-5 pointer-events-none duration-500";
  }
  if (store.participantManchester !== "") {
    manchesterStyle.value =
      "flex flex-col outline outline-1 w-72 b-rd-3 bg-#dddddd p-7 gap-5 pointer-events-none duration-500";
  }
});
</script>
<template>
  <main>
    <div
      class="bg-#ffffff/[.40] bg-r-5 p-25 b-rd-3 outline outline-1 backdrop-blur mt-20 max-w-screen-lg mx-auto"
    >
      <div class="flex flex-row flex-justify-center mt-6">
        <div :class="manchesterStyle">
          <img src="/logo-manchester.png" alt="manchester" class="" />
          <button
            type="button"
            @click="set_participant_manchester"
            class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1 mt-auto m-5"
          >
            PICK
          </button>
        </div>
        <img src="/vs.svg" alt="vs" class="p-10" />
        <div :class="chelseaStyle">
          <img src="/logo-chelsea.png" alt="chelsea" class="item-center" />
          <button
            type="button"
            @click="set_participant_chelsea"
            class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1 mt-auto m-5"
          >
            PICK
          </button>
        </div>
      </div>
    </div>
    <div
      class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10"
      v-if="store.participantManchester !== ''"
    >
      <p class="leading-relaxed">
        <strong>{{ store.participantManchester }}</strong> voted for Manchester.
        <br />
        🎉 {{ store.participantManchester }} is an amazing voter 🎉
      </p>
    </div>
    <div
      class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-10"
      v-if="store.participantChelsea !== ''"
    >
      <p class="leading-relaxed">
        <strong>{{ store.participantChelsea }}</strong> voted for Chelsea.
        <br />
        🎉 {{ store.participantChelsea }} is an amazing voter 🎉
      </p>
    </div>
  </main>
</template>
