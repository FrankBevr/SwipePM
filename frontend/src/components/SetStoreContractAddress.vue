<script setup lang="ts">
import { store } from "../store/store.ts";
</script>
<template>
  <div
    class="flex flex-justify-center p-10 flex-col text-center outline items-center max-w-xl m-auto backdrop-blur b-rd-3 border-none outline outline-1 text-base font-light items-stretch">
    <p class="leading-relaxed">
      <strong>store.contractAddress</strong> holds the contract address
      globally.
      <br />
      After redeploy it allows to reset the contract to the new value.
    </p>
    <form @submit.prevent>
      <div>
        <label for="address" class="block text-sm font-light">Contract Address</label>
        <input
          class="text-white w-full leading-tight focus:outline-none focus:shadow-outline opacity-20 bg-black leading-5 m-0 p-10px"
          type="text" name="address" v-model="store.contractAddress" />
      </div>
      SwipePM's contract address is {{ store.contractAddress }}
    </form>
  </div>
</template>
