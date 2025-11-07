<script setup lang="ts">
import { FilePlus, FolderPlus, Plus, Trash } from 'lucide-vue-next'
import UploadFilesComponent from '@/views/panel/components/small_components/uploadFilesComponent.vue'
import CreateNewFolderComponent from '@/views/panel/components/small_components/createNewFolderComponent.vue'
import { DropdownMenu, DropdownMenuItem, DropdownMenuContent, DropdownMenuTrigger, DropdownMenuSeparator } from '@/components/ui/dropdown-menu'
import { ChevronDown, ChevronUp } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { ref } from 'vue'

const openDialog = ref<boolean>()
const closeDialog = () => {
  openDialog.value = false
}
</script>

<template>
  <DropdownMenu v-model:open="openDialog">
    <DropdownMenuTrigger as-child>
      <Button variant="outline" class="ml-2 md:ml-6">
        <Plus /> New <ChevronDown v-if="openDialog" class="ml-2 h-4 w-4" /><ChevronUp v-if="!openDialog" class="ml-2 h-4 w-4" />
      </Button>
    </DropdownMenuTrigger>

    <DropdownMenuContent>

      <DropdownMenuItem>
            <CreateNewFolderComponent @closeDialog="closeDialog" />
      </DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuItem>
          <UploadFilesComponent @closeDialog="closeDialog" />
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>



</template>

<style scoped>

</style>