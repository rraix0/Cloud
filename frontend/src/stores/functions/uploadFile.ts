import axios from 'axios'
import type { FileObject } from '@/stores/uploaderStorage.ts'

export const uploadFile = async (item: FileObject) => {

  const formData = new FormData();
  formData.append('file', item.file);
  formData.append('title', item.file.name);
  formData.append('folder', JSON.stringify(item.parent_id));
  await axios.post(import.meta.env.VITE_API_URL + '/storage/upload_object',
    formData,
    {
      headers: {
        Authorization: localStorage.getItem('jwt'),
      },
      onUploadProgress: (progressEvent) => {
        if (progressEvent.total) {
          item.upload_progress = Math.round(
            ((progressEvent.loaded / progressEvent.total)) * 100,
          )
        }
      },
    }).then((res) => {
    item.upload_progress = 100;
    item.upload_status = "done"
  }).catch((err) => {
    item.errorMessage = err.message;
  })

}