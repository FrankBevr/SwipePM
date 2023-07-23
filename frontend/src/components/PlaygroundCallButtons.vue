<script setup lang="ts">
import { onMounted } from "vue";
import { store } from "../store/store";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
onMounted(async () => {
  const wsProvider = new WsProvider();
  const api = await ApiPromise.create({ provider: wsProvider });
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  console.log(alice.address);

  const contract = new ContractPromise(
    api,
    store.metadata,
    "5CCMZSbtF18EkGkvA58UuHVmtZCZ5P4DfkhGoidQBdMAaSWC",
  );
  const contractCall = await contract.query["footballMatch::getGame"](
    alice.address,
    {
      gasLimit: -1,
      storageDepositLimit: null,
    },
  );
  console.log(contractCall.result.isErr);
});
</script>
<template>
  <h1 class="flex flex-justify-center p-10 underline">
    Playground Call Buttons
  </h1>
  <div class="flex flex-justify-center p-10">
    <button type="button" class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3">
      getGame
    </button>
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
      {{ store.accounts }}
    </pre>
  </div>
  <h1 class="flex flex-justify-center">selectedAccount</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      {{ store.selectedAccount }}
    </pre>
  </div>
  <h1 class="flex flex-justify-center">Metadata</h1>
  <div class="flex flex-justify-center p-10">
    <pre class="bg-#CCCCCC font-light text-sm p-5 b-rd-3">
      {{ store.metadata }}
    </pre>
  </div>
</template>
