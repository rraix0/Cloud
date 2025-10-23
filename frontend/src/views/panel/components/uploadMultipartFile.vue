<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

    const storage = useStorageStorage()
    const selectedFile = ref<File | null>(null)
    const uploadProgress = ref<number>(0)
    const errorMessage = ref<string>('')

    const handleFileChange = (event: Event) => {
      const target = event.target as HTMLInputElement
      if (target.files && target.files.length > 0) {
        selectedFile.value = target.files[0] ?? null;
      }
    }

    const uploadFile = async () => {
      if (!selectedFile.value) return

      const file = selectedFile.value
      const chunkSize = 16 * 1024 * 1024 // 16 MB
      const totalChunks = Math.ceil(file.size / chunkSize)
      const title = file.name

      errorMessage.value = ''
      uploadProgress.value = 0

      try {

        if (!file) {
          errorMessage.value = 'File not selected'
          return
        }
        if (!localStorage.getItem("jwt")) {
          errorMessage.value = 'Authorization failed, try logging in again'
          return
        }
        // STEP 1
        const formData1 = new FormData()
        formData1.append('title', title)
        formData1.append('folder', JSON.stringify(storage?.current_folder_parent?.id))

        formData1.append('size', String(file?.size))
        const initResponse = await axios.post(
          import.meta.env.VITE_API_URL + '/storage/create_multipart_upload',
          formData1,
          {
            headers: {
              'Content-Type': 'multipart/form-data',
              Authorization: localStorage.getItem('jwt'),
            },
          },
        )
        const uploadId = initResponse.data

        // STEP 2
        for (let i = 0; i < totalChunks; i++) {
          const start = i * chunkSize
          const end = Math.min(start + chunkSize, file.size)
          const chunk = file.slice(start, end)

          const formData2 = new FormData()
          formData2.append('file', chunk, `${title}.part${i + 1}`)
          formData2.append('part_number', (i + 1).toString())
          formData2.append('upload_id', uploadId)

          await axios.post(import.meta.env.VITE_API_URL + '/storage/multipart_upload', formData2, {
            headers: {
              'Content-Type': 'multipart/form-data',
              Authorization: localStorage.getItem('jwt'),
            },
            onUploadProgress: (progressEvent) => {
              if (progressEvent.total) {
                uploadProgress.value = Math.round(
                  ((i + progressEvent.loaded / progressEvent.total) / totalChunks) * 100,
                )
              }
            },
          })
        }

        // STEP 3
        const formData3 = new FormData()
        formData3.append('upload_id', uploadId)
        await axios.post(import.meta.env.VITE_API_URL + '/storage/complete_multipart_upload', formData3, {
          headers: {
            'Content-Type': 'multipart/form-data',
            Authorization: localStorage.getItem('jwt'),
          },
        }).then(() => {
          storage.getFiles(storage?.current_folder_parent?.id)
        })

        uploadProgress.value = 100
        console.log('Upload completed successfully')
      } catch (error) {
        errorMessage.value = 'Error during upload: ' + (error as Error).message
        console.error(error)
      }
    }

</script>
<template>
  <div class="flex gap-2">
    <Input type="file" class="w-min" @change="handleFileChange" />
    <Button variant="outline" @click="uploadFile" :disabled="!selectedFile">Upload</Button>
    <p v-if="uploadProgress">Upload progress: {{ uploadProgress }}%</p>
    <p v-if="errorMessage">{{ errorMessage }}</p>
  </div>
</template>