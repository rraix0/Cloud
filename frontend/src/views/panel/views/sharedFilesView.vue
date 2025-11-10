<script setup lang="ts">
import { ChevronRight } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useComponentsManager} from '@/stores/componentsManager.ts'
const components_manager = useComponentsManager();
</script>

<template>
  <Button class="md:hidden top-2 left-2" @click="components_manager.toggleSidebarOpen()" variant="outline"><ChevronRight class="h-5 w-5" /></Button>
  SHAREDFilesView - in production
</template>

<style scoped></style>
