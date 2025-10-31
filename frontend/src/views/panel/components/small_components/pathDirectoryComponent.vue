<script setup lang="ts">
import {
  Breadcrumb, BreadcrumbEllipsis,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator
} from '@/components/ui/breadcrumb'
import { Slash } from 'lucide-vue-next'
import { ref } from 'vue'

import { useStorageStorage } from '@/stores/storageStorage.ts'
import router from '@/router'
const storage = useStorageStorage()

</script>

<template>
  <Breadcrumb>
    <BreadcrumbList class="ml-2 mb-3">

      <template class="flex" v-for="(item, index) in storage.path" :key="index">
        <BreadcrumbItem class="hover:cursor-pointer hover:text-primary" @click="storage.goTo(item.id)">
          <BreadcrumbPage v-if="index == storage.path.length - 1"  class="hover:font-bold">
            {{ item.name }}
          </BreadcrumbPage>
          <template v-else>
            {{ item.name }}
          </template>
        </BreadcrumbItem>
        <BreadcrumbSeparator v-if="index != storage.path.length - 1">
          <Slash />
        </BreadcrumbSeparator>
      </template>

    </BreadcrumbList>
  </Breadcrumb>
</template>

<style scoped>

</style>