<script setup lang="ts">
import { Input } from '@/components/ui/input'
import { ref } from 'vue'
import { Button } from '@/components/ui/button'

const files = ref<File | null []>([null])
const chunkSize = 16 * 1024 * 1024 // 16 MB
const testfile = ref<any>()




</script>
<template>
  <Input type="file" multiple v-model="testfile" v-on:change="setFiles($event.target.files)" />
  <Button @click="testit">Test</Button>
</template>

<style scoped>

</style>