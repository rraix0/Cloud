<script setup lang="ts">
import { Progress } from '@/components/ui/progress'
import { Button } from '@/components/ui/button'
import { Info, X } from 'lucide-vue-next'

import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '@/components/ui/sheet'

import { useUploaderStorage} from '@/stores/uploaderStorage.ts'
import { ScrollArea } from '@/components/ui/scroll-area'
const uploader_store = useUploaderStorage()

</script>

<template>
  <Sheet>
    <SheetTrigger asChild>
      <Button class="ml-auto" variant="secondary"><Info /> Queue: {{uploader_store.uploadQueue.filter(x => x.upload_status == "done").length}}/{{uploader_store.uploadQueue.length}}</Button>
    </SheetTrigger>
    <SheetContent>
      <SheetHeader>
        <SheetTitle>Files queued:</SheetTitle>
        <hr class="mb-2" />
        <SheetDescription>
          <ScrollArea class="w-full h-[90vh]">
            <div v-for="(item, index) in uploader_store.uploadQueue" key="index" class="mb-4 flex ">
              <div class="w-full">
                <span class="text-base text-primary">{{ item.file.name }}</span><br>
                <span class="text-muted-foreground ml-2 text-sm">Status: {{item.upload_status}}</span>
                <Progress :model-value="item.upload_progress" class="h-1 ml-2" />
              </div>
              <div class="flex  justify-end mt-2 mr-2">
                <X class="hover:text-red-500" />
              </div>
            </div>
          </ScrollArea>
        </SheetDescription>
      </SheetHeader>
    </SheetContent>
  </Sheet>



</template>

<style scoped>

</style>