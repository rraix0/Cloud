<template>
  <div>
    <input type="file" @change="handleFileChange" />
    <button @click="uploadFile" :disabled="!selectedFile">Upload</button>
    <p v-if="uploadProgress">Upload progress: {{ uploadProgress }}%</p>
    <p v-if="errorMessage">{{ errorMessage }}</p>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import axios from 'axios'


export default defineComponent({
  setup() {
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
        const authorization_user_id: string | null = localStorage.getItem('user_id')
        const authorization_jwt: string | null = localStorage.getItem('jwt')
        if (!file) {
          errorMessage.value = 'File not selected'
          return
        }
        if (!authorization_user_id || !authorization_jwt) {
          errorMessage.value = 'Authorization failed, try logging in again'
          return
        }
        // STEP 1

        const formData1 = new FormData()
        formData1.append('title', title)
        formData1.append(
          'folder',
          '{\n' +
          '            "tb": "folders",\n' +
          '            "id": {\n' +
          '                "String": "01998ce3-ef40-7244-898a-7c5f281cb815"\n' +
          '            }\n' +
          '        }',
        )
        // @ts-ignore
        formData1.append('size', file.size)
        formData1.append('authorization.user_id', authorization_user_id)
        formData1.append('authorization.jwt', authorization_jwt)
        const initResponse = await axios.post(
          import.meta.env.VITE_HOST + '/storage/create_multipart_upload',
          formData1,
          {
            headers: { 'Content-Type': 'multipart/form-data' },
          },
        )
        const uploadId = initResponse.data

        // STEP 2
        for (let i = 0; i < totalChunks; i++) {
          const start = i * chunkSize
          const end = Math.min(start + chunkSize, file.size)
          const chunk = file.slice(start, end)

          const formData2 = new FormData()
          formData2.append('authorization.user_id', authorization_user_id)
          formData2.append('authorization.jwt', authorization_jwt)
          formData2.append('file', chunk, `${title}.part${i + 1}`)
          formData2.append('part_number', (i + 1).toString())
          formData2.append('upload_id', uploadId)

          await axios.post(import.meta.env.VITE_HOST + '/storage/multipart_upload', formData2, {
            headers: { 'Content-Type': 'multipart/form-data' },
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
        formData3.append('authorization.user_id', authorization_user_id)
        formData3.append('authorization.jwt', authorization_jwt)
        await axios.post(import.meta.env.VITE_HOST + '/storage/complete_multipart_upload', formData3, {
          headers: { 'Content-Type': 'multipart/form-data' },
        })

        uploadProgress.value = 100
        console.log('Upload completed successfully')
      } catch (error) {
        errorMessage.value = 'Error during upload: ' + (error as Error).message
        console.error(error)
      }
    }

    return { selectedFile, handleFileChange, uploadFile, uploadProgress, errorMessage }
  },
})
</script>
