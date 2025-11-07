<script setup lang="ts">
import { useDarkModeStore } from '@/stores/darkModeStorage.ts'
import {
  ChevronLeft,
  ChevronRight,
  Database,
  HelpCircle,
  Files,
  LogOut,
  Moon,
  Sun,
  UserCircle,
  Users,
  Cloud,
  CircleX,
  Search,
} from 'lucide-vue-next'
import { Separator } from '@/components/ui/separator'
import { RouterLink, useRoute } from 'vue-router'
import { Button } from '@/components/ui/button'
import { ref } from 'vue'
import router from '@/router'
import { useComponentsManager } from '@/stores/componentsManager.ts'
import { logout } from '@/views/panel/functions/logout.ts'
import { useStorageStorage} from '@/stores/storageStorage.ts'
import ColumnsDropDownMenuComopnent from '@/views/panel/components/small_components/columnsDropDownMenuComopnent.vue'
import { onClickOutside } from '@vueuse/core'
import { useTemplateRef } from 'vue'



const storage= useStorageStorage();
const components_manager = useComponentsManager()
const route = useRoute()





</script>

<template>
  <aside
    class="z-10 fixed md:static flex flex-col h-full justify-between border-r bg-card p-4 shadow-sm w-min"
    v-show="components_manager.sidebar_open"
  >
    <div class="space-y-6">
      <!-- PHONE -->
      <Button
        @click="components_manager.toggleSidebarOpen()"
        variant="ghost"
        class="w-full justify-start gap-3 md:hidden"
      >
        <ChevronLeft v-show="components_manager.sidebar_collapse" class="h-5 w-5" />
        <span v-show="components_manager.sidebar_collapse">Hide</span>
      </Button>
      <div class="flex justify-center">
        <RouterLink to="/" class="text-2xl font-bold text-primary flex f-row gap-3">
          <Cloud class="h-7 w-7 text-brand-text" />
          <span v-show="components_manager.sidebar_collapse" class="text-brand-text">Cloud</span>
        </RouterLink>
      </div>
      <nav class="space-y-1">
        <RouterLink to="/panel/storage">
          <Button
            variant="ghost"
            class="w-full justify-start gap-3"
            :class="
              route.name === 'storage'
                ? 'text-muted-foreground transition-colors hover:text-primary'
                : ''
            "
          >
            <Files class="h-5 w-5" />
            <span v-show="components_manager.sidebar_collapse">My files</span>
          </Button>
        </RouterLink>
        <RouterLink to="/panel/shared">
          <Button
            variant="ghost"
            class="w-full justify-start gap-3"
            :class="
              route.name === 'shared'
                ? 'text-muted-foreground transition-colors hover:text-primary'
                : ''
            "
          >
            <Users class="h-5 w-5" />
            <span v-show="components_manager.sidebar_collapse">Shared</span>
          </Button>
        </RouterLink>
        <RouterLink to="/panel/account">
          <Button
            variant="ghost"
            class="w-full justify-start gap-3"
            :class="
              route.name === 'account'
                ? 'text-muted-foreground transition-colors hover:text-primary'
                : ''
            "
          >
            <UserCircle class="h-5 w-5" />
            <span v-show="components_manager.sidebar_collapse">Account</span>
          </Button>
        </RouterLink>
      </nav>
      <!-- PHONE -->
      <div class="md:hidden">
        <Separator />
        <Button
          variant="ghost"
          class="pt-4 mb-2"
          @click="() => {
            components_manager.toggleFilterInputPhone(); components_manager.toggleSidebarOpen(); storage.filterValues('');
          }"
          >
          <Search />
          Find file
        </Button>
        <ColumnsDropDownMenuComopnent />
      </div>
    </div>
    <!-- END -->

    <div class="space-y-2">
      <Separator class="my-4" />
      <Button
        v-if="useDarkModeStore().darkMode"
        @click="useDarkModeStore().toggleDarkMode()"
        variant="ghost"
        class="w-full justify-start gap-3"
      >
        <Moon class="h-5 w-5" />
        <span v-show="components_manager.sidebar_collapse"> Dark mode</span>
      </Button>
      <Button
        v-else
        @click="useDarkModeStore().toggleDarkMode()"
        variant="ghost"
        class="w-full justify-start gap-3"
      >
        <Sun class="h-5 w-5" />
        <span v-show="components_manager.sidebar_collapse"> Light mode</span>
      </Button>
      <Button @click="logout()" variant="ghost" class="w-full justify-start gap-3 text-red-500">
        <LogOut class="h-5 w-5" />
        <span v-show="components_manager.sidebar_collapse">Log out</span>
      </Button>
      <!-- PC -->
      <Button
        @click="components_manager.toggleSidebarMode()"
        variant="ghost"
        class="w-full justify-start gap-3 hidden md:flex"
      >
        <ChevronLeft v-show="components_manager.sidebar_collapse" class="h-5 w-5" />
        <ChevronRight v-show="!components_manager.sidebar_collapse" class="h-5 w-5" />
        <span v-show="components_manager.sidebar_collapse">Hide</span>
      </Button>
    </div>
  </aside>
</template>