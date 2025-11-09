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
import { Separator } from "@/components/ui/separator"

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
}
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'

import { Pencil, LogOut, ChevronRight, KeyRound, CircleX } from 'lucide-vue-next'

import { useComponentsManager} from '@/stores/componentsManager.ts'

const components_manager = useComponentsManager()
const user_data = ref<UserData>();
const change_pass_open = ref<boolean>(false);

const edit_open = ref<boolean>(false);
const password_old = ref<string>();
const password_new = ref<string>();
const password_new1 = ref<string>();

const error_message = ref<string>();
const info_message = ref<string>();


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
  error_message.value = err.response.data;
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
    info_message.value = res.data.message;
    console.log(res.data);
  }).catch(err => {
    error_message.value = err.response.data;
    if (err.status === 401) {
      logout()
    }
    console.log(err);
  })
}

const reset_token = () => {
  axios.post(import.meta.env.VITE_API_URL + "/users/reset_token", {},
    {
      headers: {
        Authorization: localStorage.getItem('jwt'),
      }}

  ).then(res => {
    logout();
    console.log(res.data);
  }).catch(err => {
    error_message.value = err.response.data;
    if (err.status === 401) {
      logout()
    }
    console.log(err);
  })
}

const change_password = () => {
  if (password_new.value !== password_new1.value) {
    error_message.value = "Passwords do not match";
    change_pass_open.value = false;
    return;
  }
  if (password_new.value && password_new.value.length < 8) {
    change_pass_open.value = false;
    error_message.value = "Password must be at least 8 characters";
    return;
  }
  axios.post(import.meta.env.VITE_API_URL + "/users/change_password", {
      old_pass: password_old.value,
      new_pass: password_new.value,
    },
    {
      headers: {
        Authorization: localStorage.getItem('jwt'),
      }}

  ).then(res => {
    change_pass_open.value = false;
    info_message.value = res.data.message;
    console.log(res.data);
  }).catch(err => {
    change_pass_open.value = false;
    error_message.value = err.response.data;
    if (err.status === 401) {
      logout()
    }
    console.log(err);
  })
}

</script>

<template>
  <Button class="md:hidden fixed top-2 left-2" @click="components_manager.toggleSidebarOpen()" variant="outline"><ChevronRight class="h-5 w-5" /></Button>
  <div class="w-full h-screen flex justify-center items-center">
    <Card class="w-full m-2 md:w-150 h-min">

      <span v-if="error_message" class="mx-2 flex w-full border border-red-500 rounded-md p-1 text-center text-red-400 font-bold pl-3">
        {{ error_message }}
        <CircleX class="ml-auto mr-1 hover:text-white hover:cursor-pointer" @click="error_message = ''" />
      </span>
      <span v-if="info_message" class="mx-2 flex w-full border border-green-500 rounded-md p-1 text-center text-green-400 font-bold pl-3">
        {{ info_message }}
        <CircleX class="ml-auto mr-1 hover:text-white hover:cursor-pointer" @click="info_message = ''" />
      </span>

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
      <CardFooter class="flex flex-col">
        <!-- Change password -->
        <div class="flex w-full">
          <Dialog v-model:open="change_pass_open">
            <DialogTrigger>
              <Button><KeyRound /> Change password</Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Edit profile</DialogTitle>
                <DialogDescription>
                  Make changes to your profile here. Click save when you're done.
                </DialogDescription>
              </DialogHeader>
              <div class="grid gap-4 py-4">
                <div class="flex flex-col  gap-4">
                  <Label for="name" class="text-right">
                    Old password
                  </Label>
                  <Input id="name" class="col-span-3" v-model="password_old" />
                </div>
                <div class="flex flex-col  gap-4">
                  <Label for="name" class="text-right">
                    New password
                  </Label>
                  <Input id="name" class="col-span-3" v-model="password_new" />
                </div>
                <div class="flex flex-col  gap-4">
                  <Label for="name" class="text-right">
                    Repeat new password
                  </Label>
                  <Input id="name" class="col-span-3" v-model="password_new1" />
                </div>
              </div>

              <DialogFooter class="flex flex-row">
                <div class="w-min">
                <DialogClose class="w-min">
                  <Button type="button" variant="secondary" @click="change_pass_open = false">
                    Close
                  </Button>
                </DialogClose>
                </div>
                <div class="w-min ml-auto">
                  <Button class="w-min" @click="change_password">Change</Button>
                </div>
              </DialogFooter>
            </DialogContent>
          </Dialog>


          <!-- Edit profile -->
          <Dialog v-model:open="edit_open">
            <DialogTrigger class="ml-auto">
              <Button><Pencil /> Edit Profile</Button>
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
              <DialogFooter class="flex flex-row">
                <div class="w-min">
                  <DialogClose class="w-min">
                    <Button type="button" variant="secondary" @click="edit_open = false">
                      Close
                    </Button>
                  </DialogClose>
                </div>
                <div class="w-min ml-auto">
                  <Button class="w-min" @click="edit_user_data">Save</Button>
                </div>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
        <Separator class="m-3" />
        <!-- Reset user token -->
        <AlertDialog>
          <AlertDialogTrigger>
            <Button variant="destructive"><LogOut /> Logout all devices</Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Are you sure?</AlertDialogTitle>
              <AlertDialogDescription>
                This will logout every your device account from every device.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Cancel</AlertDialogCancel>
              <AlertDialogAction @click="reset_token">
                <span class="text-red-500">Continue</span>
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>

      </CardFooter>
    </Card>
  </div>
</template>

<style scoped></style>
