<script setup lang="ts">

import {
  DropdownMenu,
  DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { ChevronDown, ChevronUp, Columns3Cog } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useStorageStorage} from '@/stores/storageStorage.ts'
import { ref } from 'vue'

const storage = useStorageStorage()

const localOpen = ref()

</script>

<template>
  <div>
    <DropdownMenu v-model:open="localOpen" class="hidden">
      <DropdownMenuTrigger as-child>
        <Button variant="outline" class="md:ml-6">
          <Columns3Cog /> Columns <ChevronDown v-if="localOpen" class="ml-2 h-4 w-4" /><ChevronUp v-if="!localOpen" class="ml-2 h-4 w-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuLabel>Columns</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuCheckboxItem v-for="item in storage.headers" :key="item.id" v-model:model-value="item.selected" @select.prevent>
          {{ item.name }}
        </DropdownMenuCheckboxItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
</template>

<style scoped>

</style>