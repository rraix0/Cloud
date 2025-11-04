<script setup lang="ts">

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import axios from 'axios'
import { logout } from '@/views/panel/functions/logout.ts'
import { ref } from 'vue'

interface UserData {
  id: object,
  created_at: string,
  name: string,
  email: string,
  storage_capacity: number
};

const user_data = ref<UserData>();
const edit_open = ref<boolean>(false);

function formatBytes(bytes:number | undefined) {
  if (bytes) {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    const exponent = Math.floor(Math.log(bytes) / Math.log(1024));
    const size = bytes / Math.pow(1024, exponent);
    return `${size.toFixed(2)} ${units[exponent]}`;
  }
}


axios.post(import.meta.env.VITE_API_URL + "/users/get_user_info", {},
  {
    headers: {
      Authorization: localStorage.getItem('jwt'),
    }}

).then(res => {
  let data:UserData = res.data;
  data.created_at = new Date(data.created_at).toLocaleDateString()
  user_data.value = data;
  console.log(res.data);
}).catch(err => {
  if (err.status === 401) {
    logout()
  }
  console.log(err);
})


const edit_name = ref<string>();
const edit_user_data = () => {
  if (!edit_name.value) return;
  axios.post(import.meta.env.VITE_API_URL + "/users/edit_user", {
    name: edit_name.value,
    },
    {
      headers: {
        Authorization: localStorage.getItem('jwt'),
      }}

  ).then(res => {
    let data:UserData = res.data;
    data.created_at = new Date(data.created_at).toLocaleDateString()
    user_data.value = data;
    edit_open.value = false
    console.log(res.data);
  }).catch(err => {
    if (err.status === 401) {
      logout()
    }
    console.log(err);
  })
}

</script>

<template>
  <div class="w-full h-screen flex justify-center items-center">
    <Card class="w-[400px] h-min">
      <CardHeader>
        <CardTitle class="font-bold mx-auto">Hi {{user_data?.name}}</CardTitle>
        <CardDescription></CardDescription>
      </CardHeader>
      <CardContent>
        <div class="flex justify-end">
          <span>Email: </span>
          <span class="ml-auto">{{user_data?.email}}</span>
        </div>
        <hr />
        <div class="flex justify-end">
          <span>Username: </span>
          <span class="ml-auto">{{user_data?.name}}</span>
        </div>
        <hr />
        <div class="flex justify-end">
          <span>Account created: </span>
          <span class="ml-auto">{{user_data?.created_at}}</span>
        </div>
        <hr />
        <div class="flex justify-end">
          <span>Storage capacity: </span>
          <span class="ml-auto">{{formatBytes(user_data?.storage_capacity)}}</span>
        </div>
      </CardContent>
      <CardFooter>

        <Dialog v-model:open="edit_open">
          <DialogTrigger class="ml-auto">
            <Button>Edit Profile</Button>
          </DialogTrigger>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Edit profile</DialogTitle>
              <DialogDescription>
                Make changes to your profile here. Click save when you're done.
              </DialogDescription>
            </DialogHeader>
            <div class="grid gap-4 py-4">
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="name" class="text-right">
                  Name
                </Label>
                <Input id="name" class="col-span-3" :defaultValue="user_data?.name" v-model="edit_name" />
              </div>
            </div>
            <DialogFooter class="sm:justify-start">
              <DialogClose as-child>
                <Button type="button" variant="secondary" @click="edit_open = false">
                  Close
                </Button>
              </DialogClose>
              <Button class="ml-auto" @click="edit_user_data">Save</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>

      </CardFooter>
    </Card>
  </div>
</template>

<style scoped></style>
