<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { useDarkModeStore } from '@/stores/darkModeStorage.ts'
import { Sun, Moon } from 'lucide-vue-next'

</script>

<template>
  <div class="flex h-screen items-center justify-center bg-background p-4">
    <Card class="w-full max-w-sm">
      <CardHeader class="space-y-1 text-center">
        <CardTitle class="text-2xl font-bold">Raix's Cloud</CardTitle>
        <CardDescription>Log in to your cloud or register.</CardDescription>
      </CardHeader>

      <CardContent class="grid gap-4">
        <RouterLink to="/login" class="w-full">
          <Button variant="outline" class="w-full">Login</Button>
        </RouterLink>
        <RouterLink to="/register" class="w-full">
          <Button class="w-full">Sign in</Button>
        </RouterLink>
      </CardContent>
      <CardContent class="mt-3">
        <Button v-if="useDarkModeStore().darkMode" @click="useDarkModeStore().toggleDarkMode()" variant="ghost" class="w-full">
          <Moon class="h-5 w-5" /> Dark mode
        </Button>
        <Button v-else @click="useDarkModeStore().toggleDarkMode()" variant="ghost" class="w-full">
          <Sun class="h-5 w-5" /> Light mode
        </Button>
      </CardContent>
    </Card>
  </div>
</template>