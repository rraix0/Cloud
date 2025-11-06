import axios from 'axios'
import type { FileObject } from '@/stores/uploaderStorage.ts'
import { useStorageStorage} from '@/stores/storageStorage.ts'

export const uploadFile = async (item: FileObject) => {
  const storage = useStorageStorage()
  const formData = new FormData();
  formData.append('file', item.file);
  formData.append('title', item.file.name);
  formData.append('folder', JSON.stringify(item.parent_id));

  const thumbnailArray = await generateThumbnailArray(item.file);
  if (thumbnailArray) {
    formData.append('thumbnail', JSON.stringify(thumbnailArray));
    console.log("OK")
  }

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
    console.log(err)
    item.errorMessage = err.response.data;
    storage.error_message = err.response.data;
  })
}


const generateThumbnailArray = async (file: File): Promise<number[] | null> => {
  const MAX_THUMBNAIL_SIZE = 3 * 1024;
  if (!file.type.startsWith('image/')) return null;

  return new Promise((resolve) => {
    const img = new Image();
    const reader = new FileReader();

    reader.onload = (e) => {
      img.src = e.target?.result as string;
    };

    img.onload = () => {
      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d')!;
      const size = 50;
      canvas.width = size;
      canvas.height = size;

      ctx.drawImage(img, 0, 0, size, size);

      canvas.toBlob(async (blob) => {
        if (!blob) return resolve(null);
        console.log("BLOB SIZE: " + blob.size )
        if (blob.size > MAX_THUMBNAIL_SIZE) {
          console.warn(`Thumbnail za duży: ${blob.size} bajtów`);
          return resolve(null); // odrzuć
        }
        // UGH just why: Blob → ArrayBuffer → Uint8Array → Array<number>
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const array = Array.from(uint8Array); // [82, 73, 70, 70, ...]

        resolve(array);
      }, 'image/webp', 0.5);
    };

    reader.readAsDataURL(file);
  });
};