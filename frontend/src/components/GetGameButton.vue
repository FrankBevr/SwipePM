<script setup lang="ts">
import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { BN, BN_ONE } from "@polkadot/util";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { ContractPromise } from "@polkadot/api-contract";
import * as metadata from "./football_match.json";
import { Ref, onMounted, ref } from "vue";
import { store } from "../store/store";

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

const getGame: Ref<null | number> = ref(null);
onMounted(() => {
  call_getGame();
});

const call_getGame = async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const address = store.contractAddress;
  const contract = new ContractPromise(api, metadata, address);

  const storageDepositLimit = null;

  const { result, output } = await contract.query["footballMatch::getGame"](
    alice.address,
    {
      gasLimit: api?.registry.createType("WeightV2", {
        refTime: MAX_CALL_WEIGHT,
        proofSize: PROOFSIZE,
      }) as WeightV2,
      storageDepositLimit,
    },
  );

  if (result.isOk) {
    const outy = output?.toJSON() as { ok: { ok: number } };
    if (outy !== undefined) {
      getGame.value = outy["ok"]["ok"];
    }
  } else {
    console.error("Error", result.asErr);
  }
};
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light">
    <p class="leading-relaxed">
      <strong>footballMatch::get_game</strong> returns 0.
      <br />
      If both participants are set it spits out 1 or 2.
      <br />
      Value of 1 is manchester won.
      <br />
      Value of 2 is united won.
    </p>
    <button type="button"
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1"
      @click="call_getGame">
      getGame
    </button>
    <p v-if="getGame === null">0</p>
    <p v-if="getGame !== null">{{ getGame }}</p>
  </div>
</template>
