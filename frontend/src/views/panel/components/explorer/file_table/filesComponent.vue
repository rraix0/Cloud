<script setup lang="ts">
import {
  Link2,
  CalendarIcon,
  Settings,
  FileText,
  FileSpreadsheet,
  FileChartColumnIncreasing,
  Type,
  FileImage,
  Eye,
  Pencil,
  FileVideo,
  FileAudio,
  FileCode,
  ClipboardCheck,
  Trash2,
  FileArchive,
  Box,
  File,
  Menu,
  Download,
  PencilLine,
  Move,
  RotateCcw, CalendarOff
} from 'lucide-vue-next'
import { Separator } from '@/components/ui/separator'
import { computed, type Ref, ref } from 'vue'
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
  FormField,
  FormItem,
  FormMessage,
} from "@/components/ui/form"
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

const schemaCreateFolder = toTypedSchema(z.object({
  name: z.string().min(2).max(50),
}));

const getImgFromBase64 = (thumbnail: any) => {
  const uint8Array = new Uint8Array(thumbnail)
  const blob = new Blob([uint8Array], { type: 'image/webp' })
  return URL.createObjectURL(blob)
}
import type { Folder as FolderType, File as FileType, ShareDataForm } from '@/stores/allTypes.ts'
import { toDate } from 'reka-ui/date'



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
  <TableRow v-for="(item, index) in storage.files.filter(x => x.show !== false )" :key="index">
    <TableCell v-show="storage.headers[0]?.selected" class="p-0">
      <label class=" px-2 py-3 flex items-center">
        <Checkbox v-model="item.selected" />
      </label>
    </TableCell>
    <TableCell v-show="storage.headers[1]?.selected" class="p-0">
      <img
        v-if="item.thumbnail"
        :src="getImgFromBase64(item.thumbnail)"
        alt="thumbnail"
        class="thumbnail h-[50px] w-[50px]"
      />
      <!-- VIDEO -->
      <FileVideo class="ml-2" v-else-if="item.name.match(/\.(mp4|mkv|mov|avi|wmv|flv|webm)$/i)" />
      <!-- AUDIO -->
      <FileAudio class="ml-2" v-else-if="item.name.match(/\.(mp3|wav|ogg|flac|aac|m4a|opus)$/i)" />
      <!-- IMAGES (with noe thumbnail) -->
      <FileImage class="ml-2" v-else-if="item.name.match(/\.(jpg|jpeg|png|gif|bmp|svg|webp|tiff|ico)$/i)" />
      <!-- TEXT -->
      <FileText class="ml-2" v-else-if="item.name.match(/\.(txt|md|rtf|log)$/i)" />
      <!-- PDF -->
      <FileArchive class="ml-2" v-else-if="item.name.match(/\.pdf$/i)" />
      <!-- DOCS -->
      <FileSpreadsheet class="ml-2" v-else-if="item.name.match(/\.(xls|xlsx|ods|csv)$/i)" />
      <FileChartColumnIncreasing class="ml-2" v-else-if="item.name.match(/\.(ppt|pptx|odp)$/i)" />
      <FileText class="ml-2" v-else-if="item.name.match(/\.(doc|docx|odt)$/i)" />
      <!-- SOURCE CODE -->
      <FileCode class="ml-2" v-else-if="item.name.match(/\.(js|ts|jsx|tsx|vue|html|css|scss|json|xml|yml|yaml|sh|bash|zsh|php|py|rb|go|rs|c|cpp|h|hpp|java|cs)$/i)" />
      <!-- ARCHIVE -->
      <FileArchive class="ml-2" v-else-if="item.name.match(/\.(zip|rar|7z|tar|gz|bz2|xz|iso)$/i)" />
      <!-- MODELS 3D -->
      <Box class="ml-2" v-else-if="item.name.match(/\.(obj|fbx|glb|gltf|blend|stl)$/i)" />
      <!-- FONTS -->
      <Type class="ml-2" v-else-if="item.name.match(/\.(ttf|otf|woff|woff2)$/i)" />
      <!-- CONFIGS / SYSTEM -->
      <Settings class="ml-2" v-else-if="item.name.match(/\.(ini|conf|cfg|env|toml)$/i)" />
      <!-- UNKNOWN / DEFAULT -->
      <File class="ml-2" v-else />

    </TableCell>
    <TableCell v-show="storage.headers[2]?.selected" class="truncate py-0 pl-1 md:pl-2">
      <span class="truncate hidden md:block">{{ item.name }}</span>
      <div class="leading-none h-[50px] md:hidden">
        <span class="truncate text-lg">{{ item.name }}</span><br>
        <div class=" text-xs p-0 m-0 ml-3 flex">
          <span>{{ new Date(item.created_at).toLocaleDateString() }}</span>
          <div class="ml-auto pr-5">{{ formatBytes(item.size) }}</div>
        </div>
      </div>
    </TableCell>
    <TableCell v-show="storage.headers[3]?.selected">{{ new Date(item.created_at).toLocaleDateString() }}</TableCell>
    <TableCell v-show="storage.headers[4]?.selected">{{ formatBytes(item.size) }}</TableCell>
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
    <TableCell v-show="storage.headers[6]?.selected" class="h-[50px] flex justify-center items-center">
      <DropdownMenu v-model:open="item.open_dropdown_menu">
        <DropdownMenuTrigger>
          <Menu />
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem @click="storage.getDownloadUri(item.id)"><Download /> Download</DropdownMenuItem>

          <!-- RENAME -->
          <DropdownMenuItem>
            <Form v-slot="{ handleSubmit }" as="" keep-values :validation-schema="schemaCreateFolder">
              <Dialog>
                <DialogTrigger @click.stop class="flex gap-2">
                  <PencilLine /> Rename
                </DialogTrigger>
                <DialogContent class="sm:max-w-[425px]">
                  <DialogHeader>
                    <DialogTitle>Rename file</DialogTitle>
                  </DialogHeader>
                  <form id="dialogForm" @submit.prevent="handleSubmit(values => storage.renameFile(values, item.id))">
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

          <DropdownMenuItem @click="() => {storage.move_objects.files = [item]; storage.move_objects.is_moving = true; }"><Move /> Move</DropdownMenuItem>

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
                    <Button @click="share_data.share_as = 'url'" :variant="share_data.share_as === undefined ? 'default' : (share_data.share_as == 'url' ? 'default' : 'outline')">Link</Button>
                    <Button @click="share_data.share_as = 'user'" :variant="share_data.share_as == 'user'? 'default' : 'outline'">User</Button>
                  </div>
                  <div v-if="share_data.share_as =='user'">
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
                  <span v-if="share_data.share_as == 'user'">Make sure that email is correct.</span>
                  <span v-else>Everybody with this link might get access to your folder.</span>
                  <br>Make sure to assign the appropriate permissions to ensure giving correct level of access.
                </DialogDescription>
                <DialogFooter>
                  <DialogClose>
                    <Button variant="outline" @click="item.open_dropdown_menu = false">
                      Cancel
                    </Button>
                  </DialogClose>
                  <DialogClose>
                    <Button type="submit" form="dialogForm" @click="() => {item.open_dropdown_menu = false; storage.shareSomething({file: item, share_data: share_data})}">
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
                <Trash2 class="text-red-500" /> Delete
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

<style scoped>

</style>