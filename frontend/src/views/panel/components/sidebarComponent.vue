<script setup lang="ts">
import { useDarkModeStore } from '@/stores/darkModeStorage.ts'
import { ChevronLeft, ChevronRight, Database, HelpCircle, Files, LogOut, Moon, Sun, UserCircle, Users, Cloud } from 'lucide-vue-next'
import { Separator } from '@/components/ui/separator'
import { RouterLink, useRoute } from 'vue-router'
import { Button } from '@/components/ui/button'
import { ref } from 'vue'
import router from '@/router'

import { logout } from '@/views/panel/functions/logout.ts'
const route = useRoute()
const openSidebar = ref<boolean>(true)

const toggleSidebar = () => {
  openSidebar.value = !openSidebar.value;
}

</script>

<template>
  <aside class="flex flex-col min-h-screen justify-between border-r bg-card p-4 shadow-sm w-min">
    <div class="space-y-6">
      <div class="flex justify-center">
        <RouterLink to="/" class="text-2xl font-bold text-primary flex f-row gap-3">
          <Cloud class="h-7 w-7 text-brand-text" />
          <span v-show="openSidebar" class="text-brand-text">Cloud</span>
        </RouterLink>
      </div>
      <nav class="space-y-2">
        <RouterLink to="/panel/storage">
          <Button variant="ghost" class="w-full justify-start gap-3" :class="route.name === 'storage' ? 'text-muted-foreground transition-colors hover:text-primary' : ''">
            <Files class="h-5 w-5" />
            <span v-show="openSidebar">My files</span>
          </Button>
        </RouterLink>
        <RouterLink to="/panel/shared">
          <Button variant="ghost" class="w-full justify-start gap-3" :class="route.name === 'shared' ? 'text-muted-foreground transition-colors hover:text-primary' : ''">
            <Users class="h-5 w-5" />
            <span v-show="openSidebar">Shared</span>
          </Button>
        </RouterLink>
        <RouterLink to="/panel/account">
          <Button variant="ghost" class="w-full justify-start gap-3" :class="route.name === 'account' ? 'text-muted-foreground transition-colors hover:text-primary' : ''">
            <UserCircle class="h-5 w-5" />
            <span v-show="openSidebar">Account</span>
          </Button>
        </RouterLink>
      </nav>
    </div>

    <div class="space-y-2">
      <Separator class="my-4" />
      <Button v-if="useDarkModeStore().darkMode" @click="useDarkModeStore().toggleDarkMode()" variant="ghost" class="w-full justify-start gap-3">
        <Moon class="h-5 w-5" />
        <span v-show="openSidebar"> Dark mode</span>
      </Button>
      <Button v-else @click="useDarkModeStore().toggleDarkMode()" variant="ghost" class="w-full justify-start gap-3">
        <Sun class="h-5 w-5" />
        <span v-show="openSidebar"> Light mode</span>
      </Button>
      <Button @click="logout()" variant="ghost" class="w-full justify-start gap-3 text-red-500">
        <LogOut class="h-5 w-5" />
        <span v-show="openSidebar">Log out</span>
      </Button>
      <Button @click="toggleSidebar" variant="ghost" class="w-full justify-start gap-3">
        <ChevronLeft v-show="openSidebar" class="h-5 w-5" />
        <ChevronRight v-show="!openSidebar" class="h-5 w-5" />
        <span v-show="openSidebar">Hide</span>
      </Button>
    </div>
  </aside>
</template>