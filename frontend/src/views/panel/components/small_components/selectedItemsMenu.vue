<script setup lang="ts">

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator, DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { Download, Menu, Move, PencilLine, Trash, List, Folder, File, SquareDashed } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
  AlertDialogFooter, AlertDialogHeader,
  AlertDialogTitle, AlertDialogTrigger
} from '@/components/ui/alert-dialog'

import { useStorageStorage} from '@/stores/storageStorage.ts'
import { ScrollArea } from '@/components/ui/scroll-area'

const storage = useStorageStorage();

</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger class="md:mr-3 flex column gap-1" v-if="(storage.folders.filter((item) => item.selected === true).length + storage.files.filter((item) => item.selected === true).length) > 0">
      <Button>
        {{ storage.folders.filter((item) => item.selected === true).length + storage.files.filter((item) => item.selected === true).length }}/{{ storage.folders.length + storage.files.length }}
        <List />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuLabel class="flex gap-2 justify-center">
        <span class="flex items-center" >{{ storage.folders.filter((item) => item.selected).length}} <Folder class="ml-1" :size="16" /></span>/
        <span class="flex items-center" >{{ storage.files.filter((item) => item.selected).length}} <File class="ml-1" :size="16" /></span>
      </DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuItem @click="storage.selectedDownload()" :disabled="(storage.folders.filter((item) => item.selected === true).length) != 0">
        <Download /> Download
      </DropdownMenuItem>

      <DropdownMenuItem @click="storage.selectedToMove()"><Move /> Move</DropdownMenuItem>

      <DropdownMenuItem @click="storage.selectBetween()" :disabled="(storage.folders.filter((item) => item.selected === true).length + storage.files.filter((item) => item.selected === true).length) != 2">
        <SquareDashed /> Select Between
      </DropdownMenuItem>

      <DropdownMenuSeparator />
      <DropdownMenuItem class="text-red-500"> <!-- DELETE -->
        <AlertDialog>
          <AlertDialogTrigger @click.stop class="w-full flex gap-2">
            <Trash class="text-red-500" /> Delete
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle class="flex gap-2"><Trash class="text-red-500" />Are you sure?</AlertDialogTitle>
                <ScrollArea class="h-[50vh]">
                  <div class="flex flex-col" v-if="storage.folders.filter((item) => item.selected).length > 0">
                    Folders:<br>
                    <span class="text-red-500" v-for="item in storage.folders.filter((x) => x.selected)">{{item.name}}</span>
                  </div>
                  <div class="flex flex-col" v-if="storage.files.filter((item) => item.selected).length > 0">
                    Files:<br>
                    <div class="flex flex-col">
                      <span class="text-red-500" v-for="item in storage.files.filter((x) => x.selected)">{{item.name}}</span>
                    </div>
                  </div>
                </ScrollArea>
              <AlertDialogDescription>
                This action cannot be undone. This will permanently delete your data!
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction class="text-white bg-red-500 hover:bg-red-400" @click="storage.selectedDelete()"><Trash class="text-white" /> Delete</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>

<style scoped>

</style>