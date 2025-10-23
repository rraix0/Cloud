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
      },
      {
      id: 5,
      name: 'shared',
      col_type: 1,
      style: "w-20",
      selected: true,
      },
    {
      id: 6,
      name: 'menu',
      col_type: 1,
      style: "w-20",
      selected: true,
    },
    ],
  )

  interface Folder {
    selected: boolean,
    created_at: Date,
    id: object,
    name: string,
    parent_id: object,
    share_id: [object],
    updated_at: Date,
    user_id: object,
  }

  interface File {
    selected: boolean,
    created_at: Date,
    id: object,
    metadata: object,
    name: string,
    parent_id: object,
    share_id: [object],
    size: number,
    tags: string[],
    updated_at: Date,
    user_id: object,
  }

  const folders = ref<Folder[]>([]);
  const files = ref<File[]>([]);

  const current_folder_parent = ref<Folder>();

  const getFolders = (parent_id: object | undefined = undefined) => {
    axios.post(import.meta.env.VITE_API_URL + "/storage/get_folders", {
        parent_id: parent_id,
      },
    {
      headers: {
        Authorization: localStorage.getItem('jwt'),
      }}

    ).then(res => {
      folders.value = res.data[1];
      current_folder_parent.value = res.data[0];
      console.log(res.data);
    }).catch(err => {
      console.log(err);
    })
  }

  const getFiles = (parent_id: object | undefined = undefined) => {
    axios.post(import.meta.env.VITE_API_URL + "/storage/get_objects", {
        parent_id: parent_id,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}

    ).then(res => {
      files.value = res.data;
      console.log(res.data);
    }).catch(err => {
      console.log(err);
    })
  }
  const goUp = () => {
    if (current_folder_parent.value) {
      getFolders(current_folder_parent.value.parent_id);
      getFiles(current_folder_parent.value.parent_id);
      console.log(current_folder_parent.value.parent_id);
    } else {
      getFolders(undefined);
      getFiles(undefined);
    }
  };

  const getDownloadUri = async (object_id: object) => {
    axios.post(import.meta.env.VITE_API_URL + "/storage/get_download_uri", {
        object_id: object_id,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}

    ).then(res => {
      console.log(res.data);
      let link: string = res.data as string;
      window.open(link, '_blank',  'noopener,noreferrer');
    }).catch(err => {
      console.log(err);
      throw err;
    })
  }


  return { folders, files, current_folder_parent, headers, getFolders, getFiles, goUp, getDownloadUri }
})
