<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { ChevronLeft } from 'lucide-vue-next'
import axios from 'axios'

const email = ref('')
const name = ref('')
const password = ref('')
const confirmPassword = ref('')
const ticket = ref('')


const err_info = ref('')

const handleRegister = () => {
  if (password.value !== confirmPassword.value) {
    err_info.value = "Password doesn't match"
    return
  }
  axios.post(import.meta.env.HOST + '/users/register', {
    ticket: ticket.value,
    email: email.value,
    name: name.value,
    password: password.value,
  })
}
</script>

<template>
  <div class="flex h-screen items-center justify-center bg-background p-4">
    <Card class="w-full max-w-sm">
      <CardHeader class="space-y-1 text-center">
        <CardTitle class="text-2xl font-bold">Singin</CardTitle>
      </CardHeader>

      <CardContent class="text-red-500" v-if="err_info">
        {{ err_info }}
      </CardContent>

      <CardContent class="grid gap-4">
        <form @submit.prevent="handleRegister" class="grid gap-4">
          <div class="grid gap-2">
            <Label for="ticket">Ticket - code required to create an account</Label>
            <Input id="ticket" type="text" v-model="ticket" required placeholder="Ticket / Code" />
          </div>
          <div class="grid gap-2">
            <Label for="email">Email</Label>
            <Input id="email" type="email" v-model="email" required placeholder="Your email." />
          </div>
          <div class="grid gap-2">
            <Label for="name">Username</Label>
            <Input id="name" type="text" v-model="name" required placeholder="Your name." />
          </div>
          <div class="grid gap-2">
            <Label for="password">Password</Label>
            <Input id="password" type="password" v-model="password" required placeholder="Password" />
          </div>
          <div class="grid gap-2">
            <Label for="confirm-password">Retype password</Label>
            <Input id="confirm-password" type="password" v-model="confirmPassword" required placeholder="Retype password" />
          </div>
          <Button type="submit" class="w-full mt-2">Create an account</Button>
        </form>
      </CardContent>

      <div class="flex flex-col items-center gap-4 p-6">
        <p class="text-sm text-muted-foreground">
          Already have an account?
          <RouterLink to="/login" class="text-primary hover:underline">Login</RouterLink>
        </p>
        <RouterLink to="/" class="flex items-center text-sm font-medium text-primary hover:underline">
          <ChevronLeft class="mr-1 h-4 w-4" /> Back
        </RouterLink>
      </div>
    </Card>
  </div>
</template>