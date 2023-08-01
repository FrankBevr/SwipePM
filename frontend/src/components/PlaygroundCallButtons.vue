<script setup lang="ts">
import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
import { BN, BN_ONE } from "@polkadot/util";
import type { WeightV2 } from "@polkadot/types";
import { ContractPromise } from "@polkadot/api-contract";
import * as metadata from "./football_match.json";
import { Ref, ref } from "vue";

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);

const getGame: Ref<null | number> = ref(null);

async function call() {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  const address = "5EkfPTWeH5UmTEAHQGDaGvNXGU3EGzQsTSXKiuSk6r9ADXZf";
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

  console.log(result.toHuman());

  if (result.isOk) {
    console.log("Success", output?.toJSON());
    const outy = output?.toJSON() as { ok: { ok: number } };
    if (outy !== null && outy !== undefined) {
      getGame.value = outy["ok"]["ok"];
    }
  } else {
    console.error("Error", result.asErr);
  }
}
call();
</script>
<template>
  <h1 class="flex flex-justify-center p-10 underline">
    Playground Call Buttons
  </h1>
  <div class="flex flex-justify-center p-10 flex-col text-center">
    Get Game Returns 0, if both participants are set it spits out 1 or 2
    <button type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
      getGame
    </button>
    {{ getGame }}
  </div>
  <div class="flex flex-justify-center p-10">
    <button type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
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
