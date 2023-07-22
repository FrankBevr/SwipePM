<script setup>
const logMe = () => {
  console.log("Log Me");
};
</script>
<template>
  <div class="flex flex-justify-center mb-10 mt-10">
    <button
      @click="logMe"
      class="bg-#CCCCCC font-bold text-2xl p-5 b-rd-3 hover:bg-#ffffff"
    >
      Connect
    </button>
  </div>
</template>
