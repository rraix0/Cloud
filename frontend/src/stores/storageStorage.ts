import { ref } from 'vue'
import { defineStore } from 'pinia'
import { ArrowDownUp } from 'lucide-vue-next'
import axios from 'axios'

export const useStorageStorage = defineStore('storage', () => {
  const headers = ref([
      {
        id: 0,
        name: 'Select',
        col_type: 2,
        style: "w-8",
        selected: true,
      },
      {
        id: 1,
        name: 'Icon',
        col_type: 1,
        style: "w-15",
        selected: true,
      },
      {
        id: 2,
        name: 'Name',
        col_type: 3,
        style: "w-80",
        icon: ArrowDownUp,
        selected: true,
      },
      {
        id: 3,
        name: 'date',
        col_type: 3,
        style: "w-35",
        icon: ArrowDownUp,
        selected: true,
      },
      {
        id: 4,
        name: 'size',
        col_type: 3,
        style: "w-25",
        icon: ArrowDownUp,
        selected: true,
      },{
      id: 5,
      name: 'shared',
      col_type: 1,
      style: "w-20",
      selected: true,
    },
    ],
  )
  const folders = ref([
    {
      id: 1,
      selected: false,
      name: 'Folder 1',
      date: '2021-02-10',
      size: '2GB',
      shared: true,
    },
    {
      id: 1,
      selected: false,
      name: 'Folder 2',
      date: '2021-02-10',
      size: '1GB',
      shared: true,
    },
  ]);
  const files = ref([
    {
      id: 1,
      selected: false,
      name: 'Name',
      shortcut: 'c',
      date: '2021-02-10',
      size: '2GB',
      shared: true,
    },
  ]);

  const getRootFolder = () => {
    axios.post(import.meta.env.VITE_HOST + "/storage/get_folders", { //TODO change struct of folder to correct arguments
      authorization: {
        "user_id": localStorage.getItem('user_id'),
        "jwt": localStorage.getItem('jwt')
      }
    }).then(res => {
      folders.value = res.data;
      console.log(res);
    }).catch(err => {
      console.log(err);
    })
  }


  return { folders, files, headers, getRootFolder }
})
