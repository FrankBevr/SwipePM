<script setup lang="ts"></script>
<template>
  <main>
    <div class="bg-#ffffff/[.40] bg-r-5 p-25 b-rd-3 outline outline-1 backdrop-blur mt-20 max-w-screen-lg mx-auto">
      <div class="flex flex-row flex-justify-center mt-6">
        <div
          class="flex flex-col outline outline-1 w-72 b-rd-3 bg-#FF8080 hover:bg-#FFa0a0 hover:px-9 duration-500 p-7 gap-5">
          <img src="/logo-manchester.png" alt="manchester" class="" />
          <button type="button"
            class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1 mt-auto m-5">
            PICK
          </button>
        </div>
        <img src="/vs.svg" alt="vs" class="p-10" />
        <div
          class="flex flex-col outline outline-1 w-72 b-rd-3 bg-#80D4FF hover:bg-#a0F4FF hover:px-9 duration-500 p-7 gap-5">
          <img src="/logo-chelsea.png" alt="chelsea" />
          <button type="button"
            class="bg-#CCCCCC hover:bg-#DDDDDD active:bg-#FFFFFF font-light text-base p-5 b-rd-3 max-w-md border-none outline outline-1 mt-auto m-5">
            PICK
          </button>
        </div>
      </div>
    </div>
  </main>
</template>
