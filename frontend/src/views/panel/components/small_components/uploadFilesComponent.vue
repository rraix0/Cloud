<script setup lang="ts">
import {X, FilePlus} from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import { ref } from 'vue'
import { ScrollArea } from '@/components/ui/scroll-area'
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
} from "@/components/ui/alert-dialog"
import { Button } from "@/components/ui/button"
import { useUploaderStorage } from '@/stores/uploaderStorage.ts'
const uploader_storage = useUploaderStorage()

const emit = defineEmits(['closeDialog'])

const files = ref<File[]>([])
const chunkSize = 16 * 1024 * 1024 // 16 MB
const openUploadFilesDialog = ref<boolean>(false);


const setFiles = (infiles: FileList) => {
  openUploadFilesDialog.value = true
  files.value = Array.from(infiles);
}

const removeFile = (index: number) => {
  files.value.splice(index, 1)
}

const uploadFiles = () => {
  openUploadFilesDialog.value = false
  emit('closeDialog')
  uploader_storage.addFiles(files.value)
}

const closeUploadFiles = () => {
  openUploadFilesDialog.value = false
  emit('closeDialog')
}

</script>
<template>
  <label for="inputFiles" class="w-full h-full flex gap-2"  @click.stop>
    <FilePlus /> Upload files
    <Input type="file" id="inputFiles" multiple @change="setFiles($event.target.files)" class="w-min" hidden/>
  </label>

  <AlertDialog v-model:open="openUploadFilesDialog">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Selected files:</AlertDialogTitle>
        <AlertDialogDescription>
          <ScrollArea class="h-[50vh] w-full rounded-md border p-2">
          <div v-for="(item, index) in files" key="index" class="w-full flex items-center rounded-md shadow-sm mb-1 pl-2">
            <span>{{ item?.name }}</span>
            <Button variant="ghost" class="block ml-auto" @click="removeFile(index)"><X /></Button>
          </div>
          </ScrollArea>

        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel @click="closeUploadFiles">Cancel</AlertDialogCancel>
        <AlertDialogAction @click="uploadFiles">Upload</AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>



</template>

<style scoped>

</style>