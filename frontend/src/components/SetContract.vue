<script setup lang="ts">
import { ref } from "vue";
import { store } from "../store/store";
let input_value = ref('')
const submitForm = () => {
  store.contractAddress = input_value.value;
  console.log('Saved Contract Address is the following:\n', store.contractAddress);
};
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light mt-20">
    <p class="leading-relaxed">
      <strong>Instantiate your Contract</strong> before continue.
    <ol class="text-left leading-10">
      <li> change directory to
        <code class="font-mono">./contracts/football_match</code>
      </li>
      <li>
        run <code class="font-mono"> substrate-contracts-node --dev</code>
      </li>
      <li>
        run
        <code class="font-mono">cargo contract instantiate --suri //Alice -x</code>
      </li>
      <li>
        press y for submit transaction
      </li>
      <li>
        copy the contract Address
      </li>
      <li>
        paste it in the input box below
      </li>
      <li>
        press Safe
      </li>
    </ol>
    </p>
    <form @submit.prevent="submitForm">
      <div>
        <label for="address" class="block text-sm font-light">Contract Address</label>
        <input
          class="text-white w-full leading-tight focus:outline-none focus:shadow-outline opacity-20 bg-black leading-5 m-0 p-10px"
          type="text" name="address" v-model="input_value" />
        <button
          class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1 mt-10"
          type="submit">Safe</button>
      </div>
    </form>
  </div>
</template>
