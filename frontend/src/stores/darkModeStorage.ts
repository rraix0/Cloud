import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useDarkModeStore = defineStore('dark-mode', () => {
  const darkModeStorageValue = localStorage.getItem('dark-mode')
  const darkMode = ref(darkModeStorageValue === "true")

  if (darkMode.value) {
    document.documentElement.classList.add('dark')
  } else {
    document.documentElement.classList.remove('dark')
  }

  const toggleDarkMode = () => {
    darkMode.value = !darkMode.value
    localStorage.setItem('dark-mode', darkMode.value.toString())

    document.documentElement.classList.toggle('dark', darkMode.value)
  }

  return { darkMode, toggleDarkMode }
})
