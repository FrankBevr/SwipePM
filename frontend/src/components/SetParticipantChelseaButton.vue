<script setup lang="ts">
import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { BN, BN_ONE } from "@polkadot/util";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { ContractPromise } from "@polkadot/api-contract";
import * as metadata from "./football_match.json";
import { Ref, ref } from "vue";
import { store } from "../store/store";

const participantChelsea: Ref<string | null> = ref(null);
const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

const call_setParticipantChelsea = async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const bob = keyring.addFromUri("//Bob", { name: "Bob" });

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
    if (res.isInBlock) {
      participantChelsea.value = "Bob";
    }
  });
};
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col font-light text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 outline outline-1 border-none">
    <p class="leading-relaxed">
      <strong>footballMatch::set_participant_chelsea</strong> sets the
      participant who bets on chelsea.
      <br />
      The caller of it bets on chelsea
      <br />
      The address of the caller will be safed.
    </p>
    <button type="button" @click="call_setParticipantChelsea"
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1">
      setParticpantChelsea
    </button>
    <p v-if="participantChelsea === null">None bid on Chelsea</p>
    <p v-if="participantChelsea !== null">
      {{ participantChelsea }} bid on Chelsea
    </p>
  </div>
</template>
