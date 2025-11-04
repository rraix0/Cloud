import axios from 'axios'
import type { FileObject } from '@/stores/uploaderStorage.ts'
import { useStorageStorage} from '@/stores/storageStorage.ts'


export const uploadMultipartFile = async (item: FileObject) => {
  const storage = useStorageStorage()
  const chunkSize = 16 * 1024 * 1024 // 16 MB
  let file = item.file
  const title = item.file.name
  const totalChunks = Math.ceil(file.size / chunkSize)

  // STEP 1
  const formData1 = new FormData()
  formData1.append('title', title)
  formData1.append('folder', JSON.stringify(item.parent_id))
  formData1.append('size', String(item.file.size))

  let initResponse;
   await axios.post(
    import.meta.env.VITE_API_URL + '/storage/create_multipart_upload',
    formData1,
    {
      headers: {
        'Content-Type': 'multipart/form-data',
        Authorization: localStorage.getItem('jwt'),
      },
    },
  ).then((response) => {
    initResponse = response.data
   }).catch((err) => {
     item.errorMessage = err.response.data;
     storage.error_message = err.response.data;
   })
  const uploadId = initResponse;
  if (!uploadId) return;

  // STEP 2
  for (let i = 0; i < totalChunks; i++) {
    const start = i * chunkSize
    const end = Math.min(start + chunkSize, file.size)
    const chunk = file.slice(start, end)
    const size = chunk.size ?? 0
    console.log(size, String(size))

    const formData2 = new FormData()
    formData2.append('file', chunk, `${title}.part${i + 1}`)
    formData2.append('part_number', (i + 1).toString())
    formData2.append('upload_id', uploadId)
    formData2.append('size', String(size))

    await axios.post(import.meta.env.VITE_API_URL + '/storage/multipart_upload', formData2, {
      headers: {
        'Content-Type': 'multipart/form-data',
        Authorization: localStorage.getItem('jwt'),
      },
      onUploadProgress: (progressEvent) => {
        if (progressEvent.total) {
          item.upload_progress = Math.round(
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
    item.upload_progress = 100;
    item.upload_status = "done"
  }).catch((err) => {
    item.errorMessage = err.response.data;
  })
}