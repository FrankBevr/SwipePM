<script setup lang="ts">
import GetGameButton from "./GetGameButton.vue";

import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { BN, BN_ONE } from "@polkadot/util";
import type { WeightV2 } from "@polkadot/types/interfaces";
import { ContractPromise } from "@polkadot/api-contract";
import * as metadata from "./football_match.json";

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

const call_setWinner = async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const address = "5EkfPTWeH5UmTEAHQGDaGvNXGU3EGzQsTSXKiuSk6r9ADXZf";
  const contract = new ContractPromise(api, metadata, address);

  const storageDepositLimit = null;

  const { gasRequired } = await contract.query["footballMatch::setWinner"](
    alice.address,
    {
      gasLimit: api?.registry.createType("WeightV2", {
        refTime: MAX_CALL_WEIGHT,
        proofSize: PROOFSIZE,
      }) as WeightV2,
      storageDepositLimit,
    },
    1,
  );

  const gasLimit = api?.registry.createType(
    "WeightV2",
    gasRequired,
  ) as WeightV2;

  await contract.tx["footballMatch::setWinner"](
    {
      gasLimit,
      storageDepositLimit,
    },
    5,
  ).signAndSend(alice);
};
</script>

<template>
  <h1 class="flex flex-justify-center p-10 underline">
    Playground Call Buttons
  </h1>
  <GetGameButton></GetGameButton>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3"
  >
    <p class="leading-relaxed">
      <strong>footballMatch::set_winner</strong> sets the winner.
      <br />
      Only the admin sends the result.
      <br />
      The admin types in the result of match.
      <br />
      If Admin sets value 1, manchester won.
      <br />
      If Admin sets value 2, united won.
    </p>
    <button
      @click="call_setWinner"
      type="button"
      class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-bold text-2xl p-5 b-rd-3 max-w-md"
    >
      setWinner
    </button>
  </div>
  <div class="flex flex-justify-center p-10">
    <button type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
      setParticpantChelsea
    </button>
  </div>
  <div class="flex flex-justify-center p-10">
    <button type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
      setParticpantManchester
    </button>
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
