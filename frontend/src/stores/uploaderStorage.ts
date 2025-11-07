import { defineStore } from 'pinia'
import { ref } from 'vue'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { file } from 'zod'
import { uploadFile } from '@/stores/functions/uploadFile.ts'
import { uploadMultipartFile } from '@/stores/functions/uploadMultipartFile.ts'
interface Thing {
  id: {
    String: string
  },
  tb: string,
}
export interface FileObject {
  file: File,
  upload_status: "pending"|"running"|"done"|"error"|"aborted",
  upload_progress: number,
  errorMessage: string,
  parent_id: Thing
}

export const useUploaderStorage = defineStore('uploaderStorage', () => {
  const storage = useStorageStorage()


  const isUploading = ref(false)


  const uploadQueue = ref<FileObject[]>([]);
  const addFiles = (files: File[]) => {
    let folder_id = storage.current_folder_parent?.id;
    if (!folder_id) {
      storage.error_message = "Couldn't find actual folder id!"
      return;
    }
    const files_to_add = files.map((file: File) => {
      const single_file: FileObject = {
        file: file,
        upload_status: "pending",
        upload_progress: 0,
        errorMessage: '',
        parent_id: folder_id,
      }
      return single_file
    })
    uploadQueue.value.push(...files_to_add);

    startUpload()

    console.log(uploadQueue.value);
  }


  const startUpload = async () => {
    if (isUploading.value) return;
    if (uploadQueue.value.length == 0) return;
    let actual_file_index = 0;

    while (actual_file_index < uploadQueue.value.length) {
      let file_object: FileObject | undefined = uploadQueue.value[actual_file_index];
      if (!file_object) return

      console.log("Attempting to upload file");
      console.log(file_object.file.name);

      if (file_object.file.size < (16 * 1024 * 1024)) {
        await uploadFile(file_object);
      } else {
        await uploadMultipartFile(file_object)
      }


      actual_file_index++;
      console.log("DONE")
      //storage.getFiles(storage.current_folder_parent?.id);

    }

    uploadQueue.value = [];
  }





  return { uploadQueue, addFiles }
})