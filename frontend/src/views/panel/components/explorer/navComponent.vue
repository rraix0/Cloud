<script setup lang="ts">

import AddNewComponent from '@/views/panel/components/small_components/addNewComponent.vue'
import UploadQueueComponent from '@/views/panel/components/small_components/uploadQueueComponent.vue'
import { ChevronRight, ChevronsUp, CircleX, RefreshCcw } from 'lucide-vue-next'
import ColumnsDropDownMenuComopnent from '@/views/panel/components/small_components/columnsDropDownMenuComopnent.vue'
import { Input } from '@/components/ui/input'
import SelectedItemsMenu from '@/views/panel/components/small_components/selectedItemsMenu.vue'
import SelectedToMoveComponent from '@/views/panel/components/small_components/selectedToMoveComponent.vue'
import { Button } from '@/components/ui/button'
import { useComponentsManager } from '@/stores/componentsManager.ts'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { useUploaderStorage } from '@/stores/uploaderStorage.ts'

const components_manager = useComponentsManager();
const storage = useStorageStorage();
const uploader_storage = useUploaderStorage()
</script>

<template>
  <div class="flex items-center pt-2 md:hidden" v-show="components_manager.filter_files_input">
    <!-- PHONE -->
    <Input class="max-w-sm md:hidden" placeholder="Filter files..." @input="storage.filterValues($event.target.value)" />
    <Button variant="outline" class="p-0 ml-2" @click="components_manager.toggleFilterInputPhone(); storage.filterValues('')"><circle-x /></Button>
  </div>

  <div class="flex items-center py-3">
    <!-- PHONE -->
    <Button class="md:hidden" @click="components_manager.toggleSidebarOpen()" variant="outline"><ChevronRight class="h-5 w-5" /></Button>
    <SelectedItemsMenu v-if="!components_manager.isPhone" />

    <!-- PC -->
    <Input class="max-w-sm hidden md:block" placeholder="Filter files..." @input="storage.filterValues($event.target.value)" />

    <Button variant="outline" class="ml-2 md:ml-5" @click="() => {storage.goUp();}">
      <ChevronsUp /> Go up
    </Button>
    <addNewComponent />
    <SelectedToMoveComponent v-if="!components_manager.isPhone" />
    <uploadQueueComponent v-if="uploader_storage.uploadQueue.length > 0 && !components_manager.isPhone" />
    <Button variant="outline" class="ml-auto" @click="() => {storage.refreshData();}">
      <RefreshCcw /> Refresh
    </Button>

    <ColumnsDropDownMenuComopnent class="hidden md:block" />
  </div>
  <div class="flex items-center">
    <SelectedItemsMenu v-if="components_manager.isPhone" />
    <SelectedToMoveComponent v-if="components_manager.isPhone" />
    <uploadQueueComponent v-if="uploader_storage.uploadQueue.length > 0 && components_manager.isPhone" />
  </div>
</template>

<style scoped>

</style>