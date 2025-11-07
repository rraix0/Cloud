<script setup lang="ts">
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Button } from '@/components/ui/button'

import { File, Folder, Move, Eye, Ban, Trash, ChevronUp, ChevronDown } from 'lucide-vue-next'

import { useStorageStorage } from '@/stores/storageStorage.ts'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { ref } from 'vue'
const storage = useStorageStorage()

const openDialog = ref<boolean>()

</script>

<template>
  <DropdownMenu v-if="storage.move_objects.is_moving" v-model:open="openDialog">
    <DropdownMenuTrigger class="ml-2 md:ml-auto">
      <Button>
        <Move /> Moving <ChevronDown v-if="openDialog" /><ChevronUp v-if="!openDialog" />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuLabel class="flex justify-center items-center gap-2">
        {{ storage.move_objects.folders.length }} <Folder :size="16" />/
        {{ storage.move_objects.files.length }} <File :size="16" />
      </DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuItem>
        <AlertDialog>
          <AlertDialogTrigger @click.stop class="w-full flex gap-2">
            <Move /> Move here
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle class="flex gap-2"
                ><Move />Selected data to move</AlertDialogTitle>
              <ScrollArea class="h-[50vh]">
                <div class="flex flex-col gap-1 mb-3"
                  v-if="storage.move_objects.folders.length > 0">
                  <span class="flex align-center gap-2"><Folder /> Folders:</span>
                  <div class="flex flex-col ml-2">
                    <span v-for="item in storage.move_objects.folders">{{ item.name }}</span>
                    </div>
                </div>
                <div class="flex flex-col gap-1"
                  v-if="storage.move_objects.files.length > 0">
                  <span class="flex align-center gap-2"><File />Files:</span>
                  <div class="flex flex-col ml-2">
                    <span
                      v-for="item in storage.move_objects.files">
                      {{ item.name }}
                    </span>
                  </div>
                </div>
              </ScrollArea>
              <AlertDialogDescription>
                Selected folders/files you picked.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction @click="storage.moveToDirectory(storage.current_folder_parent?.id, storage.move_objects?.files.map((item) => item.id), storage.move_objects?.folders.map((item) => item.id))">Move here</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuItem
        @click="
          () => {
            storage.move_objects.files = []
            storage.move_objects.folders = []
            storage.move_objects.is_moving = false
          }
        "
      >
        <Ban /> Cancel
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<style scoped></style>
