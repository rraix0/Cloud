<script setup lang="ts">

import { CircleX } from 'lucide-vue-next'
import { useStorageStorage } from '@/stores/storageStorage.ts'

const storage = useStorageStorage()
</script>

<template>
  <div class="flex flex-col gap-2">
      <span v-if="storage.error_message" class="flex w-full border border-red-500 rounded-md p-1 text-center text-red-400 font-bold pl-3">
        {{ storage.error_message }}
        <CircleX class="ml-auto mr-1 hover:text-white hover:cursor-pointer" @click="storage.error_message = ''" />
      </span>
    <span v-if="storage.info_message" class="flex w-full border border-green-500 rounded-md p-1 text-center text-green-400 font-bold pl-3">
        {{ storage.info_message }}
        <CircleX class="ml-auto mr-1 hover:text-white hover:cursor-pointer" @click="storage.info_message = ''" />
      </span>
  </div>
</template>

<style scoped>

</style>