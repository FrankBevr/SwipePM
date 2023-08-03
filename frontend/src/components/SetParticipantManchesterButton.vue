<script setup lang="ts">
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
  <div
    class="flex flex-justify-center p-10 flex-col font-light text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1">
    <p class="leading-relaxed">
      <strong>footballMatch::set_participant_manchester</strong> sets the
      participant who bets on manchester.
      <br />
      The caller of it bets on manchester
      <br />
      The address of the caller will be safed.
    </p>
    <button @click="set_participant_manchester" type="button"
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1">
      setParticpantManchester
    </button>
    <p v-if="participantManchester === null">None bid on Manchester</p>
    <p v-if="participantManchester !== null">
      {{ participantManchester }} bid on Manchester
    </p>
  </div>
</template>
