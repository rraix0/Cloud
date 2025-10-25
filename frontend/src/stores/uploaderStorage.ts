import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useStorageStorage } from '@/stores/storageStorage.ts'


export const uploaderStorage = defineStore('uploaderStorage', () => {
  const storage = useStorageStorage()

  // Settings:
  const chunkSize = 16 * 1024 * 1024 // 16 MB

  interface FileObject {
    file: File,
    upload_progress: number,
    errorMessage: string,
  }
  const uploadQueue = ref<File | null []>([null]);






  return { uploadQueue }
})