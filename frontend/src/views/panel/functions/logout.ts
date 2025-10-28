import router from '@/router'
import { useStorageStorage } from '@/stores/storageStorage.ts'

export const logout = () => {
  const storage = useStorageStorage();
  storage.error_message = undefined;
  storage.info_message = undefined;
  localStorage.removeItem('authorization')
  localStorage.removeItem('user_id')
  localStorage.removeItem('jwt')
  localStorage.clear()
  router.push('/')
}