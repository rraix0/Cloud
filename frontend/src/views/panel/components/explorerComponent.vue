<script setup lang="ts">

import { ArrowUpDown, ChevronDown, Check, Folder, File, ArrowDownUp } from 'lucide-vue-next'
import { ref } from 'vue'

import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuSeparator,
  DropdownMenuLabel,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Input } from '@/components/ui/input'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

import { useStorageStorage } from '@/stores/storageStorage.ts'

const storage = useStorageStorage()

storage.getRootFolder();


import test_component_to_delete from './test_component_to_delete.vue'
import UploadMultipartFile from '@/views/panel/components/uploadMultipartFile.vue'

</script>

<template>
  <div class="w-full p-3">
    <div class="flex items-center py-4">
      <Input class="max-w-sm" placeholder="Filter emails..." />
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="outline" class="ml-auto">
            Columns <ChevronDown class="ml-2 h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuLabel>Panel Position</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuCheckboxItem v-for="item in storage.headers" :key="item.id" v-model:model-value="item.selected">
            {{ item.name }}
          </DropdownMenuCheckboxItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>

    <div class="rounded-md border">
      <Table class="table-fixed w-full">
        <TableHeader>
          <TableRow>
            <TableHead v-for="item in storage.headers" :key="item.id" v-show="item.selected" :class="item.style">
                <template v-if="item.col_type === 1">
                  {{ item.name }}
                </template>
                <Checkbox v-if="item.col_type === 2" />
                <Button variant="ghost" v-if="item.col_type === 3">{{ item.name }}
                  <component :is="item.icon" />
                </Button>
            </TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="true">

            <TableRow v-for="item in storage.folders" :key="item.id">
              <TableCell v-show="storage.headers[0]?.selected" >
                <Checkbox :default-value=item.selected />
              </TableCell>
              <TableCell v-show="storage.headers[1]?.selected">
                <Folder />
              </TableCell>
              <TableCell v-show="storage.headers[2]?.selected">{{ item.name }}</TableCell>
              <TableCell v-show="storage.headers[3]?.selected">{{ item.date }}</TableCell>
              <TableCell v-show="storage.headers[4]?.selected">{{ item.size }}</TableCell>
              <TableCell v-show="storage.headers[5]?.selected">{{ item.shared }}</TableCell>
            </TableRow>
            <TableRow v-for="item in storage.files" :key="item.id">
              <TableCell v-show="storage.headers[0]?.selected" >
                <Checkbox :default-value=item.selected />
              </TableCell>
              <TableCell v-show="storage.headers[1]?.selected">
                <File />
              </TableCell>
              <TableCell v-show="storage.headers[2]?.selected">{{ item.name }}</TableCell>
              <TableCell v-show="storage.headers[3]?.selected">{{ item.date }}</TableCell>
              <TableCell v-show="storage.headers[4]?.selected">{{ item.size }}</TableCell>
              <TableCell v-show="storage.headers[5]?.selected">{{ item.shared }}</TableCell>
            </TableRow>

          </template>

          <TableRow v-else>
            <TableCell :colspan="storage.headers.length" class="h-24 text-center"> No results. </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <div class="flex items-center justify-end space-x-2 py-4">
      <div class="flex-1 text-sm text-muted-foreground">
        {{ storage.folders.filter((item) => item.selected === true).length }} of
        {{ storage.folders.length }} row(s) selected.
      </div>
      <div class="space-x-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="false"
          @click=""
        >
          Previous
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="false"
          @click=""
        >
          Next
        </Button>
      </div>
    </div>

    <hr>
    <upload-multipart-file></upload-multipart-file>

  </div>
</template>