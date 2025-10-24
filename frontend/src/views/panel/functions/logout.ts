import router from '@/router'

export const logout = () => {
  localStorage.removeItem('authorization')
  localStorage.removeItem('user_id')
  localStorage.removeItem('jwt')
  localStorage.clear()
  router.push('/')
}