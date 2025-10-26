<script setup lang="ts">

import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { FolderPlus} from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { ref } from 'vue'
const storage = useStorageStorage()

const localOpen = ref(false)
const emit = defineEmits(['closeDialog'])

function openDialog() {
  localOpen.value = true

}

function closeDialog() {
  localOpen.value = false
  emit('closeDialog')
}


const schemaCreateFolder = toTypedSchema(z.object({
  name: z.string().min(2).max(50),
}));

</script>

<template>
  <Form v-slot="{ handleSubmit }" as="" keep-values :validation-schema="schemaCreateFolder">
    <Dialog v-model:open="localOpen">
      <DialogTrigger class="flex gap-2" @click="openDialog" @click.stop>
        <FolderPlus /> Create Folder
      </DialogTrigger>
      <DialogContent class="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create Folder</DialogTitle>
        </DialogHeader>
        <form id="dialogForm" @submit="handleSubmit($event, storage.createFolder)">
          <FormField v-slot="{ componentField }" name="name">
            <FormItem>
              <FormLabel>Folder name</FormLabel>
              <FormControl>
                <Input type="text" placeholder="Folder name" v-bind="componentField" />
              </FormControl>
              <FormMessage />
            </FormItem>
          </FormField>
        </form>
        <DialogFooter>
          <DialogClose>
            <Button variant="outline" @click="closeDialog">Cancel</Button>
          </DialogClose>
          <DialogClose>
            <Button type="submit" form="dialogForm" @click="closeDialog">
              Create folder
            </Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </Form>
</template>

<style scoped>

</style>