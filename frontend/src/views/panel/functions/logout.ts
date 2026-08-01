import router from '@/router'
import { useStorageStorage } from '@/stores/storageStorage.ts'
import { useRoute } from 'vue-router'

export const logout = () => {
  const storage = useStorageStorage();
  storage.error_message = "";
  storage.info_message = "";
  localStorage.removeItem('authorization')
  localStorage.removeItem('user_id')
  localStorage.removeItem('jwt')
  localStorage.clear()
}