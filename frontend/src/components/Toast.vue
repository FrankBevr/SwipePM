<script setup lang="ts">
import { store } from "../store/store";
import { ref, watch } from "vue";
const toastStyle = ref("hidden");

watch(store, () => {
  if (store.participantChelsea !== "") {
    toastStyle.value =
      "border-solid border-1 border-green p-5 absolute top-0 right-0 z-1 m-0 mt-5 mr-5 backdrop-blur b-rd-3 flex flex-justify-center text-black animate-bounce";
    setTimeout(() => {
      toastStyle.value = "hidden";
    }, 3000);
  }
  if (store.participantManchester !== "") {
    toastStyle.value =
      "border-solid border-1 border-green p-5 absolute top-0 right-0 z-1 m-0 mt-5 mr-5 backdrop-blur b-rd-3 flex flex-justify-center text-black animate-bounce";
    setTimeout(() => {
      toastStyle.value = "hidden";
    }, 3000);
  }
});
</script>
<template>
  <div :class="toastStyle" v-if="store.participantChelsea !== ''">
    🎉 {{ store.participantChelsea }} is an amazing voter 🎉
  </div>
  <div :class="toastStyle" v-if="store.participantManchester !== ''">
    🎉 {{ store.participantManchester }} is an amazing voter 🎉
  </div>
</template>
