<script setup lang="ts">

import { computed, type Ref, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Spinner } from '@/components/ui/spinner'

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { useUploaderStorage} from '@/stores/uploaderStorage.ts'

import { toTypedSchema } from '@vee-validate/zod'

import * as z from 'zod'

import { useComponentsManager} from '@/stores/componentsManager.ts'

const storage = useStorageStorage()
const components_manager = useComponentsManager();
const uploader_storage = useUploaderStorage()

function formatBytes(bytes:number) {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  const exponent = Math.floor(Math.log(bytes) / Math.log(1024));
  const size = bytes / Math.pow(1024, exponent);
  return `${size.toFixed(2)} ${units[exponent]}`;
}
const allSelected = computed({
  get: () => storage.folders.every(item => item.selected) && storage.files.every(item => item.selected),
  set: (value: boolean) => {
    storage.folders.forEach(item => item.selected = value);
    storage.files.forEach(item => item.selected = value);
  }
});

const schemaCreateFolder = toTypedSchema(z.object({
  name: z.string().min(2).max(50),
}));

const getImgFromBase64 = (thumbnail: any) => {
  const uint8Array = new Uint8Array(thumbnail)
  const blob = new Blob([uint8Array], { type: 'image/webp' })
  return URL.createObjectURL(blob)
}
import type {Folder as FolderType,  File as FileType} from '@/stores/allTypes.ts'
import FoldersComponent from '@/views/panel/components/explorer/file_table/foldersComponent.vue'
import FilesComponent from '@/views/panel/components/explorer/file_table/filesComponent.vue'


// DROPDOWN MENU SHARE BUTTON LOGIC
const getPermission = (item:FolderType | FileType, perm: string):boolean => {
  if (item.share_permissions_list && item.share_permissions_list.includes(perm)) {
    return true
  } else {
    return false
  }
}
const togglePermission = (item:FolderType | FileType, perm: string) => {
  if (item.share_permissions_list) {
    if (item.share_permissions_list.includes(perm)) {
      item.share_permissions_list = item.share_permissions_list.filter((x) => x != perm)
    } else {
      item.share_permissions_list.push(perm)
    }
  } else {
    item.share_permissions_list = [perm]
  }
}

const share_expiration_date = ref();
</script>

<template>
  <div class="rounded-md border overflow-y-auto w-full md:w-min">
    <Table class="table-fixed w-full md:w-min">
      <TableHeader class="w-full">
        <TableRow class="w-full">
          <TableHead v-for="item in storage.headers" :key="item.id" v-show="item.selected" :class="item.style">
            <template v-if="item.col_type === 1">
              {{ item.name }}
            </template>
            <Checkbox v-model="allSelected" v-if="item.col_type === 2" />
            <Button variant="ghost" v-if="item.col_type === 3" @click="item.action?.()">
              {{ item.name }}
              <component v-if="item.icon" :is="item.icon()" />
            </Button>
          </TableHead>
          <!-- probably useless
          <TableHead></TableHead>
          -->
        </TableRow>
      </TableHeader>

      <!-- TABLE -->
      <TableBody>
        <template v-if="storage.folders.length > 0 || storage.files.length > 0">
          <!-- FOLDERS  -->
            <foldersComponent />
          <!-- FILES -->
            <filesComponent />
        </template>

        <TableRow v-else-if="storage.is_waiting_for_server">
          <TableCell :colspan="storage.headers.length" class="h-24 text-center">
            <div class="flex justify-center items-center gap-2">Loading data <Spinner class="size-6" /></div>
          </TableCell>
        </TableRow>

        <TableRow v-else>
          <TableCell :colspan="storage.headers.length" class="h-24 text-center"> No results. </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>

<style scoped>

</style>