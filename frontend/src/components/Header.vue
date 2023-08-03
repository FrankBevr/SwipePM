<script setup lang="ts">
import { ref } from "vue";
const message = ref("SwipePM");
</script>
<template>
  <header class="flex flex-1 flex-items-center bg-#FFDF80 b-rd-100 pl-10 pr-10 gap-10 max-w-screen-xl mx-auto">
    <img src="/Notifications.svg" alt="notification bell" />
    <h1 class="flex-1 text-center tracking-tighter">{{ message }}</h1>
    <img src="/Slider.svg" alt="settings" />
    <img src="/Info.svg" alt="information" />
  </header>
</template>
