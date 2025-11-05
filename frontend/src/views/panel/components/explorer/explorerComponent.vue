<script setup lang="ts">
import { ArrowUpDown, ChevronDown, CircleX, RefreshCcw, Check, Columns3Cog, Plus, Folder, File, ChevronsUp, Menu, Trash, Download, PencilLine, Move, ChevronUp  } from 'lucide-vue-next'
import { computed, ref } from 'vue'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Spinner } from '@/components/ui/spinner'
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
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form"
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { useUploaderStorage} from '@/stores/uploaderStorage.ts'

const storage = useStorageStorage()
const uploader_storage = useUploaderStorage()



import { toTypedSchema } from '@vee-validate/zod'

import { ErrorMessage } from 'vee-validate'
import UploadFilesComponent from '@/views/panel/components/small_components/uploadFilesComponent.vue'
import AddNewComponent from '@/views/panel/components/small_components/addNewComponent.vue'
import * as z from 'zod'
import ColumnsDropDownMenuComopnent from '@/views/panel/components/small_components/columnsDropDownMenuComopnent.vue'
import UploadQueueComponent from '@/views/panel/components/small_components/uploadQueueComponent.vue'
import SelectedItemsMenu from '@/views/panel/components/small_components/selectedItemsMenu.vue'
import SelectedToMoveComponent from '@/views/panel/components/small_components/selectedToMoveComponent.vue'
import PathDirectoryComponent from '@/views/panel/components/small_components/pathDirectoryComponent.vue'

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

</script>

<template>
  <div class="h-full max-h-full overflow-auto px-3">
    <div class="flex flex-col gap-2">
      <span v-if="storage.error_message" class="flex w-full border border-red-500 rounded-md p-1 text-center text-red-400 font-bold pl-3">
        {{ storage.error_message }}
        <CircleX class="ml-auto mr-1 hover:text-white hover:cursor-pointer" @click="storage.error_message = ''" />
      </span>
      <span v-if="storage.info_message" class="flex w-full border border-green-500 rounded-md p-1 text-center text-green-400 font-bold pl-3">
        {{ storage.info_message }}
        <CircleX class="ml-auto mr-1 hover:text-white hover:cursor-pointer" @click="storage.info_message = ''" />
      </span>
    </div>

    <div class="flex items-center py-4">
      <SelectedItemsMenu />

      <Input class="max-w-sm" placeholder="Filter files..." @input="storage.filterValues($event.target.value)" />
      <Button variant="outline" class="ml-5" @click="() => {storage.goUp();}">
        <ChevronsUp /> Go up
      </Button>

      <addNewComponent />

      <SelectedToMoveComponent></SelectedToMoveComponent>

      <uploadQueueComponent v-if="uploader_storage.uploadQueue.length > 0" />

      <Button variant="outline" class="ml-auto" @click="() => {storage.refreshData();}">
        <RefreshCcw /> Refresh
      </Button>

      <ColumnsDropDownMenuComopnent />
    </div>

    <PathDirectoryComponent />


    <div class="rounded-md border overflow-y-auto">
      <Table class="table-fixed w-full">
        <TableHeader>
          <TableRow>
            <TableHead v-for="item in storage.headers" :key="item.id" v-show="item.selected" :class="item.style">
                <template v-if="item.col_type === 1">
                  {{ item.name }}
                </template>
                <Checkbox v-model="allSelected" v-if="item.col_type === 2" />
                <Button variant="ghost" v-if="item.col_type === 3" @click="item.action?.()">
                  {{ item.name }}
                  <component :is="item.icon" />
                </Button>
            </TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>
                    <!-- TABLE -->
        <TableBody>
          <template v-if="storage.folders.length > 0 || storage.files.length > 0">
            <!-- ================== FOLDERS ================== -->
            <TableRow v-for="(item, index) in storage.folders.filter(x => x.show !== false)" :key="index">
              <TableCell v-show="storage.headers[0]?.selected">
                <Checkbox v-model="item.selected" />
              </TableCell>
              <TableCell v-show="storage.headers[1]?.selected">
                <Folder />
              </TableCell>
              <TableCell v-show="storage.headers[2]?.selected" class="hover:cursor-pointer" @click="() => {storage.getFolders(item.id); storage.getFiles(item.id); }">{{ item.name }}</TableCell>
              <TableCell v-show="storage.headers[3]?.selected">{{ new Date(item.created_at).toLocaleDateString() }}</TableCell>
              <TableCell v-show="storage.headers[4]?.selected">{{  }}</TableCell>
              <TableCell v-show="storage.headers[5]?.selected">{{ item.share_id }}</TableCell>
              <TableCell v-show="storage.headers[6]?.selected">
                <DropdownMenu>
                  <DropdownMenuTrigger>
                    <Menu />
                  </DropdownMenuTrigger>
                  <DropdownMenuContent>
                    <DropdownMenuItem @click="storage.getDownloadUri(item.id)"><Download /> Download</DropdownMenuItem>

                    <DropdownMenuItem>       <!-- RENAME -->
                      <Form v-slot="{ handleSubmit }" as="" keep-values :validation-schema="schemaCreateFolder">
                        <Dialog>
                          <DialogTrigger @click.stop class="flex gap-2">
                            <PencilLine /> Rename
                          </DialogTrigger>
                          <DialogContent class="sm:max-w-[425px]">
                            <DialogHeader>
                              <DialogTitle>Rename file</DialogTitle>
                            </DialogHeader>
                            <form id="dialogForm" @submit="handleSubmit(values => storage.renameFolder(values, item.id))">
                              <FormField v-slot="{ componentField }" name="name">
                                <FormItem>
                                  <FormControl>
                                    <Input type="text" placeholder="File name" v-bind="componentField" :defaultValue="item.name" />
                                  </FormControl>
                                  <FormMessage />
                                </FormItem>
                              </FormField>
                            </form>
                            <DialogFooter>
                              <DialogClose>
                                <Button variant="outline">Cancel</Button>
                              </DialogClose>
                              <DialogClose>
                                <Button type="submit" form="dialogForm">
                                  Submit
                                </Button>
                              </DialogClose>
                            </DialogFooter>
                          </DialogContent>
                        </Dialog>
                      </Form>
                    </DropdownMenuItem>

                    <DropdownMenuItem @click="() => {storage.move_objects.folders = [item]}"><Move /> Move</DropdownMenuItem>

                    <DropdownMenuSeparator />
                    <DropdownMenuItem class="text-red-500">
                      <AlertDialog>
                        <AlertDialogTrigger @click.stop class="w-full flex gap-2">
                          <Trash class="text-red-500" /> Delete
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>Delete <span class="text-red-500">{{ item.name }}</span> ?</AlertDialogTitle>
                            <AlertDialogDescription>
                              This action cannot be undone. This will permanently delete your folder!<br>It only deletes empty folders!
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction class="text-white bg-red-500 hover:bg-red-400" @click="storage.deleteFolders([item.id])">
                              <Trash class="text-white" /> Delete
                            </AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </TableCell>
            </TableRow>
            <!-- ================== FILES ================== -->
            <TableRow v-for="(item, index) in storage.files.filter(x => x.show !== false )" :key="index">
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

                    <DropdownMenuItem>       <!-- RENAME -->
                      <Form v-slot="{ handleSubmit }" as="" keep-values :validation-schema="schemaCreateFolder">
                        <Dialog>
                          <DialogTrigger @click.stop class="flex gap-2">
                            <PencilLine /> Rename
                          </DialogTrigger>
                          <DialogContent class="sm:max-w-[425px]">
                            <DialogHeader>
                              <DialogTitle>Rename file</DialogTitle>
                            </DialogHeader>
                            <form id="dialogForm" @submit="handleSubmit(values => storage.renameFile(values, item.id))">
                              <FormField v-slot="{ componentField }" name="name">
                                <FormItem>
                                  <FormControl>
                                    <Input type="text" placeholder="File name" v-bind="componentField" :defaultValue="item.name" />
                                  </FormControl>
                                  <FormMessage />
                                </FormItem>
                              </FormField>
                            </form>
                            <DialogFooter>
                              <DialogClose>
                                <Button variant="outline">Cancel</Button>
                              </DialogClose>
                              <DialogClose>
                                <Button type="submit" form="dialogForm">
                                  Submit
                                </Button>
                              </DialogClose>
                            </DialogFooter>
                          </DialogContent>
                        </Dialog>
                      </Form>
                    </DropdownMenuItem>

                    <DropdownMenuItem @click="() => {storage.move_objects.files = [item]; storage.move_objects.is_moving = true; }"><Move /> Move</DropdownMenuItem>

                    <DropdownMenuSeparator />
                    <DropdownMenuItem class="text-red-500"> <!-- DELETE -->
                      <AlertDialog>
                        <AlertDialogTrigger @click.stop class="w-full flex gap-2">
                          <Trash class="text-red-500" /> Delete
                        </AlertDialogTrigger>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>Delete <span class="text-red-500">{{item.name}}</span> ?</AlertDialogTitle>
                            <AlertDialogDescription>
                              This action cannot be undone. This will permanently delete your file!
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogCancel>Cancel</AlertDialogCancel>
                            <AlertDialogAction class="text-white bg-red-500 hover:bg-red-400" @click="storage.deleteFiles([item.id])"><Trash class="text-white" /> Delete</AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </TableCell>

            </TableRow>

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
  </div>
</template>