<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { ChevronLeft } from 'lucide-vue-next'
import axios from 'axios'
import router from '@/router'
import { useStorageStorage } from '@/stores/storageStorage.ts'


const email = ref('')
const password = ref('')
const err_info = ref('')

console.log("ENV: " + import.meta.env.VITE_API_URL + " END")

if (localStorage.getItem('jwt')) {
  router.push('/panel/storage')
}

const handleLogin = () => {
  axios.post(import.meta.env.VITE_API_URL + "/users/login", {
    email: email.value,
    password: password.value,
  })
    .then(function (response) {
      localStorage.setItem('jwt', response.data)
      const storage = useStorageStorage();
      storage.error_message = "";
      storage.info_message = "";
      router.push('/panel/storage')
    })
    .catch(function (error) {
      console.log(error);
      err_info.value = "Email or password incorrect";
    });
}


</script>

<template>
  <div class="flex h-screen items-center justify-center bg-background p-4">
    <Card class="w-full max-w-sm">
      <CardHeader class="space-y-1 text-center">
        <CardTitle class="text-2xl font-bold">Login</CardTitle>
      </CardHeader>
      <CardContent class="text-red-500" v-if="err_info">
        {{ err_info }}
      </CardContent>

      <CardContent class="grid gap-4">
        <form @submit.prevent="handleLogin" class="grid gap-4">
          <div class="grid gap-2">
            <Input id="email" type="email" v-model="email" placeholder="Email" required />
          </div>
          <div class="grid gap-2">
            <Input id="password" type="password" v-model="password" placeholder="Password" required />
          </div>
          <Button type="submit" class="w-full">Login</Button>
        </form>
      </CardContent>

      <div class="flex flex-col items-center gap-4 p-6">
        <p class="text-sm text-muted-foreground">
          You don't have an account?
          <RouterLink to="/register" class="text-primary hover:underline">Create an account</RouterLink>
        </p>
        <RouterLink to="/" class="flex items-center text-sm font-medium text-primary hover:underline">
          <ChevronLeft class="mr-1 h-4 w-4" /> Back
        </RouterLink>
      </div>
    </Card>
  </div>
</template>