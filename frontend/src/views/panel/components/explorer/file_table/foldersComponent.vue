<script setup lang="ts">
import {
  ArrowUpDown,
  RotateCcw,
  CalendarOff,
  ChevronDown,
  Link,
  Link2,
  CalendarIcon,
  CircleX,
  RefreshCcw,
  Check,
  Columns3Cog,
  Plus,
  Settings,
  FileText,
  FileSpreadsheet,
  FileChartColumnIncreasing,
  Type,
  FileImage,
  FileVideo,
  FileAudio,
  FileCode,
  FileArchive,
  Box,
  FileVideoCamera,
  Folder,
  File,
  ChevronsUp,
  Menu,
  Trash,
  Download,
  PencilLine,
  Move,
  ChevronUp,
  ChevronRight,
  ClipboardCheck, Eye, Trash2, Pencil
} from 'lucide-vue-next'

import { computed, type Ref, ref } from 'vue'
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
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Calendar } from '@/components/ui/calendar'

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
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { useUploaderStorage} from '@/stores/uploaderStorage.ts'

import { toTypedSchema } from '@vee-validate/zod'

import * as z from 'zod'
import { useComponentsManager} from '@/stores/componentsManager.ts'


const storage = useStorageStorage()
const components_manager = useComponentsManager();
const uploader_storage = useUploaderStorage()

const schemaCreateFolder = toTypedSchema(z.object({
  name: z.string().min(2).max(50),
}));

import type {Folder as FolderType,  File as FileType, ShareDataForm} from '@/stores/allTypes.ts'
import { toDate } from 'reka-ui/date'
import { Separator } from '@/components/ui/separator'
import { CalendarDate } from '@internationalized/date'
import type { DateValue } from "@internationalized/date";


const share_data: ShareDataForm = {
  share_as: 'url',
  share_permissions_list: ['read'],
  share_description: "",
  share_expiration_date: null,
};

// DROPDOWN MENU SHARE BUTTON LOGIC
const getPermission = (item:FolderType | FileType, perm: string):boolean => {
  if (share_data.share_permissions_list && share_data.share_permissions_list.includes(perm)) {
    return true
  } else {
    return false
  }
}
const togglePermission = (perm: string) => {
  if (share_data.share_permissions_list) {
    if (share_data.share_permissions_list.includes(perm)) {
      share_data.share_permissions_list = share_data.share_permissions_list.filter((x) => x != perm)
    } else {
      share_data.share_permissions_list.push(perm)
    }
  } else {
    share_data.share_permissions_list = [perm]
  }
}


</script>

<template>
  <TableRow v-for="(item, index) in storage.folders.filter(x => x.show !== false)" :key="index">
    <TableCell v-show="storage.headers[0]?.selected" class="p-0">
      <label class=" px-2 py-3 flex items-center">
        <Checkbox v-model="item.selected" />
      </label>
    </TableCell>
    <TableCell v-show="storage.headers[1]?.selected">
      <Folder />
    </TableCell>
    <TableCell v-show="storage.headers[2]?.selected" class="hover:cursor-pointer truncate py-0 pl-1 md:pl-2" @click="() => {storage.getFolders(item.id); storage.getFiles(item.id); }">
      <span class="truncate hidden md:block">{{ item.name }}</span>
      <div class="leading-none md:hidden">
        <span class="truncate text-lg">{{ item.name }}</span><br>
        <div class=" text-xs p-0 m-0 ml-3 flex">
          <span>{{ new Date(item.created_at).toLocaleDateString() }}</span>
        </div>
      </div>
    </TableCell>
    <TableCell v-show="storage.headers[3]?.selected">{{ new Date(item.created_at).toLocaleDateString() }}</TableCell>
    <TableCell v-show="storage.headers[4]?.selected">{{  }}</TableCell>
    <TableCell v-show="storage.headers[5]?.selected">

      <div class="flex items-center">


        <Dialog v-if="item.share?.length">
          <DialogTrigger>
            <Link2 />
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>This file <span class="text-muted-foreground p-1">{{ item.name }}</span> is shared:</DialogTitle>
              <DialogDescription>

                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead class="w-[100px]">Description</TableHead>
                      <TableHead class="w-[100px]">Permissions</TableHead>
                      <TableHead>Expire at</TableHead>
                      <TableHead>Method</TableHead>
                      <TableHead class="text-right"></TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    <TableRow v-for="share in item.share">
                      <TableCell class="font-medium">
                        {{ share.description }}
                      </TableCell>
                      <TableCell class="font-medium">
                        <div class=" flex gap-1">
                          <Eye v-if="share.permissions.includes('read')" />
                          <Pencil v-if="share.permissions.includes('write')" />
                          <Trash2 v-if="share.permissions.includes('delete')" />
                        </div>
                      </TableCell>
                      <TableCell>
                        <span v-if="share.expiration_date">{{ share.expiration_date }}</span>
                        <span v-else>ー</span>
                      </TableCell>
                      <TableCell>
                        <div class="flex gap-2 items-center">
                          <Button v-if="share.link_token" variant="outline" @click="storage.copyShareLink(share)">
                            <Link2 v-if="!share.link_copied" />
                            <ClipboardCheck v-else />
                            copy
                          </Button>
                        </div>

                      </TableCell>

                      <TableCell>
                        <div class="w-full flex justify-end items-center gap-2">

                          <!-- EDIT SHARE
                          <AlertDialog>
                            <AlertDialogTrigger>
                              <PencilLine />
                            </AlertDialogTrigger>
                            <AlertDialogContent>
                              <AlertDialogHeader>
                                <AlertDialogTitle>Editing share</AlertDialogTitle>
                                <AlertDialogDescription>
                                  Feature will be available in the future
                                </AlertDialogDescription>
                              </AlertDialogHeader>
                              <AlertDialogFooter>
                                <AlertDialogCancel>Cancel</AlertDialogCancel>
                                <AlertDialogAction @click="">Update</AlertDialogAction>
                              </AlertDialogFooter>
                            </AlertDialogContent>
                          </AlertDialog>
                          -->
                          <!-- DELETE SHARE -->
                          <AlertDialog>
                            <AlertDialogTrigger>
                              <Trash2 class="text-red-500" />
                            </AlertDialogTrigger>
                            <AlertDialogContent>
                              <AlertDialogHeader>
                                <AlertDialogTitle>Delete share?</AlertDialogTitle>
                                <AlertDialogDescription>
                                  This action cannot be undone.
                                </AlertDialogDescription>
                              </AlertDialogHeader>
                              <AlertDialogFooter>
                                <AlertDialogCancel>Cancel</AlertDialogCancel>
                                <AlertDialogAction class="text-white bg-red-500 hover:bg-red-400" @click="storage.deleteShare(share.id)"><Trash2 class="text-white" /> Delete</AlertDialogAction>
                              </AlertDialogFooter>
                            </AlertDialogContent>
                          </AlertDialog>
                        </div>
                      </TableCell>

                    </TableRow>
                  </TableBody>
                </Table>

              </DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <DialogClose as-child>
                <Button variant="outline">
                  Close
                </Button>
              </DialogClose>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>



    </TableCell>
    <TableCell v-show="storage.headers[6]?.selected" class="h-[45px] p-0 flex justify-center items-center">

      <!-- DROPDOWN MENU -->
      <DropdownMenu v-model:open="item.open_dropdown_menu">
        <DropdownMenuTrigger>
          <Menu />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem @click="storage.getDownloadUri(item.id)"><Download /> Download</DropdownMenuItem>

          <!-- RENAME -->
          <DropdownMenuItem>
            <Form v-slot="{ handleSubmit }" keep-values :validation-schema="schemaCreateFolder">
              <Dialog>
                <DialogTrigger @click.stop class="flex gap-2">
                  <PencilLine /> Rename
                </DialogTrigger>
                <DialogContent class="sm:max-w-[425px]">
                  <DialogHeader>
                    <DialogTitle>Rename file</DialogTitle>
                  </DialogHeader>
                  <form id="dialogForm" @submit.prevent="handleSubmit(values => storage.renameFolder(values, item.id))">
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
                      <Button variant="outline" @click="item.open_dropdown_menu = false">
                        Cancel
                      </Button>
                    </DialogClose>
                    <DialogClose>
                      <Button type="submit" form="dialogForm" @click="item.open_dropdown_menu = false">
                        Submit
                      </Button>
                    </DialogClose>
                  </DialogFooter>
                </DialogContent>
              </Dialog>
            </Form>
          </DropdownMenuItem>
          <!-- MOVE -->
          <DropdownMenuItem @click="() => {storage.move_objects.folders = [item]}"><Move /> Move</DropdownMenuItem>

          <!-- SHARE -->
          <DropdownMenuItem>
            <Dialog>
              <DialogTrigger @click.stop class="flex gap-2">
                <Link2 /> Share
              </DialogTrigger>
              <DialogContent class="">
                <DialogHeader>
                  <DialogTitle>Share file</DialogTitle>
                </DialogHeader>
                <div class="flex flex-col gap-3">
                  <div class="flex items-center gap-3">
                    <span>Share by:</span>
                    <Button @click="item.share_as = 'url'" :variant="item.share_as === undefined ? 'default' : (item.share_as == 'url' ? 'default' : 'outline')">Link</Button>
                    <Button @click="item.share_as = 'user'" :variant="item.share_as == 'user'? 'default' : 'outline'">User</Button>
                  </div>
                  <div v-if="item.share_as =='user'">
                    <Input type="email" placeholder="User's email"></Input>
                  </div>
                  <Separator />
                  <div class="flex items-center gap-6">
                    <label class="flex items-center gap-2"> <!-- Read -->
                      <span class="select-none">Read</span>
                      <Checkbox
                        :defaultValue="getPermission(item, 'read')"
                        @update:model-value="() => {togglePermission('read')}"
                      ></Checkbox>
                    </label>
                    <label class="flex items-center gap-2"> <!-- Write -->
                      <span class="select-none">Write</span>
                      <Checkbox
                        :defaultValue="getPermission(item, 'write')"
                        @update:model-value="() => {togglePermission('write')}"
                      ></Checkbox>
                    </label>
                    <label class="flex items-center gap-2"> <!-- Delete -->
                      <span class="select-none">Delete</span>
                      <Checkbox
                        :defaultValue="getPermission(item, 'delete')"
                        @update:model-value="() => {togglePermission('delete')}"
                      ></Checkbox>
                    </label>
                    <label class="flex items-center gap-2"> <!-- Recursive -->
                      <span>
                        <TooltipProvider>
                        <Tooltip>
                          <TooltipTrigger >
                              <label for="some_label" class="select-none">Recursive</label>
                          </TooltipTrigger>
                          <TooltipContent>
                            With subfolders
                          </TooltipContent>
                        </Tooltip>
                      </TooltipProvider>
                      </span>
                      <Checkbox
                        :defaultValue="getPermission(item, 'recursive')"
                        @update:model-value="() => {togglePermission('recursive')}"
                        id="some_label"
                      ></Checkbox>
                    </label>
                  </div>
                  <Separator />
                  <div class="flex items-center gap-2">

                    <Popover v-slot="{ close }">
                      <PopoverTrigger as-child>
                        <Button
                          variant="outline"
                          class="w-min justify-start text-left font-normal"
                        >
                          <CalendarIcon v-if="share_data.share_expiration_date" />
                          <CalendarOff v-if="!share_data.share_expiration_date" />
                          {{ share_data.share_expiration_date ? `Expires at ${share_data.share_expiration_date}` : "No expiration date" }}
                        </Button>

                      </PopoverTrigger>
                      <PopoverContent class="w-auto p-0" align="start">
                        <Calendar
                          v-model="share_data.share_expiration_date"
                          layout="month-and-year"
                          initial-focus
                          @update:model-value="close"
                        />
                      </PopoverContent>
                    </Popover>

                    <Button
                      class="w-min "
                      variant="outline"
                      v-if="share_data.share_expiration_date"
                      @click="share_data.share_expiration_date = null"
                    >
                      <RotateCcw />
                    </Button>

                  </div>
                  <Separator />
                  <div class="flex items-center gap-2">
                    <Input type="text" placeholder="Share description" v-model="share_data.share_description" />
                    <Button
                      class="w-min "
                      variant="outline"
                      v-if="share_data.share_description"
                      @click="share_data.share_description = undefined"
                    >
                      <RotateCcw />
                    </Button>
                  </div>
                </div>
                <DialogDescription class="mt-3">
                  <span v-if="item.share_as == 'user'">Make sure that email is correct.</span>
                  <span v-else>Everybody with this link might get access to your folder.</span>
                  <br>Make sure to assign the appropriate permissions to ensure giving correct level of access.
                </DialogDescription>

                <DialogFooter>
                  <DialogClose>
                    <Button variant="outline" @click="() => {item.open_dropdown_menu = false; share_data.share_expiration_date = null}">
                      Cancel
                    </Button>
                  </DialogClose>
                  <DialogClose>
                    <Button type="submit" form="dialogForm" @click="() => {item.open_dropdown_menu = false; storage.shareSomething({folder: item, share_data: share_data})}">
                      Share
                    </Button>
                  </DialogClose>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </DropdownMenuItem>

          <DropdownMenuSeparator />

          <!-- DELETE -->
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
</template>

<style scoped>

</style>