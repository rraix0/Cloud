<script setup lang="ts">
import { ArrowUpDown, ChevronDown, Check, Folder, File, ArrowDownUp, Menu, Trash, Download, PencilLine, Move  } from 'lucide-vue-next'
import { computed, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import {
  Dialog,
  DialogContent,
  DialogClose,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuSeparator,
  DropdownMenuLabel,
  DropdownMenuContent,
  DropdownMenuTrigger,
  DropdownMenuItem,
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
import { useStorageStorage } from '@/stores/storageStorage.ts'
const storage = useStorageStorage()

storage.getFolders();
storage.getFiles();

import UploadMultipartFile from '@/views/panel/components/uploadMultipartFile.vue'

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


</script>

<template>
  <div class="h-full max-h-full overflow-auto p-3">
    <div class="flex items-center py-4">
      <Input class="max-w-sm" placeholder="Filter files..." />
      <Button
        variant="outline"
        size="sm"
        class="ml-5"
        :disabled="false"
        @click="() => {storage.goUp();}"
      >
        Go up
      </Button>
      <Button
        variant="outline"
        size="sm"
        class="ml-5"
        :disabled="false"
        @click=""
      >
        Create folder
      </Button>
      <AlertDialog>
        <AlertDialogTrigger>Open</AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete your account
              and remove your data from our servers.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction>Continue</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>


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

    <div class="rounded-md border overflow-y-auto">
      <Table class="table-fixed w-full">
        <TableHeader>
          <TableRow>
            <TableHead v-for="item in storage.headers" :key="item.id" v-show="item.selected" :class="item.style">
                <template v-if="item.col_type === 1">
                  {{ item.name }}
                </template>
                <Checkbox v-model="allSelected" v-if="item.col_type === 2" />
                <Button variant="ghost" v-if="item.col_type === 3">{{ item.name }}
                  <component :is="item.icon" />
                </Button>
            </TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <template v-if="true">
            <!-- ================== FOLDERS ================== -->
            <TableRow v-for="(item, index) in storage.folders" :key="index">
              <TableCell v-show="storage.headers[0]?.selected" >
                <Checkbox v-model="item.selected" />
              </TableCell>
              <TableCell v-show="storage.headers[1]?.selected">
                <Folder />
              </TableCell>
              <TableCell v-show="storage.headers[2]?.selected" @click="() => {storage.getFolders(item.id); storage.getFiles(item.id); }">{{ item.name }}</TableCell>
              <TableCell v-show="storage.headers[3]?.selected">{{ new Date(item.created_at).toLocaleDateString() }}</TableCell>
              <TableCell v-show="storage.headers[4]?.selected">{{  }}</TableCell>
              <TableCell v-show="storage.headers[5]?.selected">{{ item.share_id }}</TableCell>
            </TableRow>
            <!-- ================== FILES ================== -->
            <TableRow v-for="(item, index) in storage.files" :key="index">
              <TableCell v-show="storage.headers[0]?.selected" >
                <Checkbox v-model="item.selected" />
              </TableCell>
              <TableCell v-show="storage.headers[1]?.selected">
                <File />
              </TableCell>
              <TableCell v-show="storage.headers[2]?.selected">{{ item.name }}</TableCell>
              <TableCell v-show="storage.headers[3]?.selected">{{ new Date(item.created_at).toLocaleDateString() }}</TableCell>
              <TableCell v-show="storage.headers[4]?.selected">{{ formatBytes(item.size) }}</TableCell>
              <TableCell v-show="storage.headers[5]?.selected">{{ item.share_id }}</TableCell>
              <TableCell v-show="storage.headers[6]?.selected">
                <DropdownMenu>
                  <DropdownMenuTrigger>
                    <Menu />
                  </DropdownMenuTrigger>
                  <DropdownMenuContent>
                    <DropdownMenuItem @click="storage.getDownloadUri(item.id)"><Download /> Download</DropdownMenuItem>
                    <DropdownMenuItem><PencilLine /> Rename</DropdownMenuItem>
                    <DropdownMenuItem><Move /> Move</DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem class="text-red-500">
                      <AlertDialog>
                        <AlertDialogTrigger @click.stop class="flex gap-2">
                          <Trash class="text-red-500" /> Delete
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                            <AlertDialogDescription>
                              This action cannot be undone. This will permanently delete your file!
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction class="text-white bg-red-500 hover:bg-red-400"><Trash class="text-white" /> Delete</AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </TableCell>

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
        {{ storage.folders.filter((item) => item.selected === true).length + storage.files.filter((item) => item.selected === true).length }} of
        {{ storage.folders.length + storage.files.length }} item(s) selected.
      </div>
    </div>

    <hr>
    <upload-multipart-file></upload-multipart-file>

  </div>
</template>