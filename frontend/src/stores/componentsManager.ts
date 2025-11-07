import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useComponentsManager = defineStore('components-manager', () => {
  // SIDEBAR
  const isPhone = ref(window.innerWidth <= 1024)
  const sidebar_open = ref<boolean>(!isPhone.value)
  const sidebar_collapse = ref<boolean>(true);

  const toggleSidebarOpen = () => {
    sidebar_open.value = !sidebar_open.value;
  }
  const toggleSidebarMode = () => {
    sidebar_collapse.value = !sidebar_collapse.value;
  }
  // FILTER FILES INPUT -> PHONE
  const filter_files_input = ref<boolean>(false)

  const toggleFilterInputPhone = () => {
    filter_files_input.value = !filter_files_input.value;
  }


  return { sidebar_open, sidebar_collapse, filter_files_input, isPhone,
    toggleSidebarOpen, toggleSidebarMode, toggleFilterInputPhone
  }
})
