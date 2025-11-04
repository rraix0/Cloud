import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import { ArrowDownUp, ArrowUpWideNarrow, ArrowDownWideNarrow } from 'lucide-vue-next'
import axios from 'axios'
import { logout } from '@/views/panel/functions/logout.ts'

export const useStorageStorage = defineStore('storage', () => {
  const move_objects = ref<{is_moving: boolean, files: File[], folders: Folder[]}>({is_moving: false, files: [], folders: []})

  const sorted_by = ref<SortedBy>('name');
  const sortValues = (sort_by: SortedBy) => {
    switch (sort_by) {
      case 'name':
        folders.value = folders.value.sort((a:Folder, b:Folder) => a.name.localeCompare(b.name));
        files.value = files.value.sort((a:File, b:File) => a.name.localeCompare(b.name));
        sorted_by.value = 'name'
        break;
      case 'r_name':
        folders.value = folders.value.sort((a:Folder, b:Folder) => b.name.localeCompare(a.name));
        files.value = files.value.sort((a:File, b:File) => b.name.localeCompare(a.name));
        sorted_by.value = 'r_name'
        break;
      case 'date':
        folders.value = folders.value.sort((a:Folder, b:Folder) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
        files.value = files.value.sort((a:File, b:File) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
        sorted_by.value = 'date'
        break;
      case 'r_date':
        folders.value = folders.value.sort((a:Folder, b:Folder) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
        files.value = files.value.sort((a:File, b:File) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
        sorted_by.value = 'r_date'
        break;
      case 'size':
        files.value = files.value.sort((a:File, b:File) => a.size - b.size);
        sorted_by.value = 'size'
        break;
      case 'r_size':
        files.value = files.value.sort((a:File, b:File) => b.size - a.size);
        sorted_by.value = 'r_size'
        break;
    }

  }
  const headers = computed(() => [
      {
        id: 0,
        name: 'Select',
        col_type: 2,
        style: "w-7",
        selected: true,
      },
      {
        id: 1,
        name: 'Icon',
        col_type: 1,
        style: "w-10",
        selected: true,
      },
      {
        id: 2,
        name: 'Name',
        col_type: 3,
        style: "w-100",
        icon: sorted_by.value == 'name' ? ArrowUpWideNarrow : sorted_by.value == 'r_name' ? ArrowDownWideNarrow : ArrowDownUp,
        selected: true,
        action: () => sorted_by.value == 'name'?  sortValues('r_name'): sortValues('name'),
      },
      {
        id: 3,
        name: 'date',
        col_type: 3,
        style: "w-25",
        icon: sorted_by.value == 'date' ? ArrowUpWideNarrow : sorted_by.value == 'r_date' ? ArrowDownWideNarrow : ArrowDownUp,
        selected: true,
        action: () => sorted_by.value == 'date'?  sortValues('r_date'): sortValues('date'),
      },
      {
        id: 4,
        name: 'size',
        col_type: 3,
        style: "w-25",
        icon: sorted_by.value == 'size' ? ArrowUpWideNarrow : sorted_by.value == 'r_size' ? ArrowDownWideNarrow : ArrowDownUp,
        selected: true,
        action: () => sorted_by.value == 'size'?  sortValues('r_size'): sortValues('size'),
      },
      {
      id: 5,
      name: 'shared',
      col_type: 1,
      style: "w-15",
      selected: true,
      },
    {
      id: 6,
      name: 'menu',
      col_type: 1,
      style: "w-15",
      selected: true,
    },
    ],
  )

  interface Folder {
    selected: boolean,
    show: boolean,
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
    show: boolean,
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

  type SortedBy = 'name' | 'r_name' | 'date' | 'r_date' | 'size' | 'r_size';

  interface FolderTreeElement {
    name: string,
    id: object,
  }

  const info_message = ref<string>()
  const error_message = ref<string>()

  const folders = ref<Folder[]>([]);
  const files = ref<File[]>([]);
  const path = ref<FolderTreeElement[]>([])

  const current_folder_parent = ref<Folder>();

  const getFolders = (parent_id: object | undefined = undefined) => {
    folders.value = [];
    axios.post(import.meta.env.VITE_API_URL + "/storage/get_folders", {
        parent_id: parent_id,
      },
    {
      headers: {
        Authorization: localStorage.getItem('jwt'),
      }}

    ).then(res => {
      folders.value = res.data[1];
      sortValues(sorted_by.value);
      current_folder_parent.value = res.data[0];
      path.value = res.data[2].reverse();
      console.log(res.data);
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
    })
  }

  const getFiles = (parent_id: object | undefined = undefined) => {
    files.value = [];
    axios.post(import.meta.env.VITE_API_URL + "/storage/get_objects", {
        parent_id: parent_id,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}

    ).then(res => {
      files.value = res.data;
      sortValues(sorted_by.value);
      console.log(res.data);
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
    })
  }

  const goUp = () => {
    getFolders(current_folder_parent.value?.parent_id);
    getFiles(current_folder_parent.value?.parent_id);
  };

  const goTo = (id: object) => {
    getFolders(id);
    getFiles(id);
  };

  const refreshData = () => {
      getFolders(current_folder_parent.value?.id);
      getFiles(current_folder_parent.value?.id);
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
      const a = document.createElement('a');
      a.href = link;
      a.download = '';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      console.log(err);
      error_message.value = err.message;
      throw err;
    })
  }

  function createFolder(values: { name?: string }) {
    let name = values.name;
    if (!name) return;
    axios.post(import.meta.env.VITE_API_URL + "/storage/create_folder", {
        parent_id: current_folder_parent.value?.id,
        name: name
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}
    ).then(res => {
      if (current_folder_parent.value) {
        getFolders(current_folder_parent.value.id);
      } else {
        getFolders(undefined);
      }
      console.log(res.data);
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
      throw err;
    })
  }

  function deleteFolders(folder_id: object[]) {
    axios.post(import.meta.env.VITE_API_URL + "/storage/delete_folders", {
        folders_ids: folder_id,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}
    ).then(res => {
      if (current_folder_parent.value) {
        getFolders(current_folder_parent.value.id);
      } else {
        getFolders(undefined);
      }
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
      throw err;
    })
  }
  function deleteFiles(file_ids: object[]) {
    axios.post(import.meta.env.VITE_API_URL + "/storage/delete_objects", {
        object_ids: file_ids,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}
    ).then(res => {
      if (current_folder_parent.value) {
        getFiles(current_folder_parent.value.id);
      } else {
        getFiles(undefined);
      }
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
      throw err;
    })
  }

  const filterValues = (phrase: string) => {
    console.log(phrase);
    folders.value.map((item: Folder) => {
      if (!phrase) {
        item.show = true;
      } else if (item.name.toLowerCase().includes(phrase.toLowerCase())) {
        item.show = true;
      } else {
        item.show = false;
      }
    });
    files.value.map((item: File) => {
      if (!phrase) {
        item.show = true;
      } else if (item.name.toLowerCase().includes(phrase.toLowerCase())) {
        item.show = true;
      } else {
        item.show = false;
      }
    });
  }

  const renameFolder = (values: { name?: string }, folder_id: object) => {
    if (!values.name) return;
    axios.post(import.meta.env.VITE_API_URL + "/storage/rename_folder", {
        folder_id: folder_id,
        name: values.name,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}
    ).then(res => {
      getFolders(current_folder_parent.value?.id);
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
      throw err;
    })
  }

  const renameFile = (values: { name?: string }, object_id: object) => {
    if (!values.name) return;
    axios.post(import.meta.env.VITE_API_URL + "/storage/rename_object", {
        object_id: object_id,
        name: values.name,
      },
      {
        headers: {
          Authorization: localStorage.getItem('jwt'),
        }}
    ).then(res => {
      getFiles(current_folder_parent.value?.id);
    }).catch(err => {
      if (err.status === 401) {
        logout()
      }
      error_message.value = err.message;
      console.log(err);
      throw err;
    })
  }

  const selectBetween = () => {
    let is_selecting = false;
    folders.value.forEach((object: Folder, index) => {
      if (!is_selecting && object.selected) {
        is_selecting = true;
      } else if (is_selecting && object.selected) {
        is_selecting = false;
      } else if (is_selecting) {
        object.selected = true;
      }
    })
    files.value.forEach((object: File, index) => {
      if (!is_selecting && object.selected) {
        is_selecting = true;
      } else if (is_selecting && object.selected) {
        is_selecting = false;
      } else if (is_selecting) {
        object.selected = true;
      }
    })
  }

  const selectedDownload = async () => { // TODO Repair it later to download as zip
    const file_ids:File[] = files.value.filter((item: File) => item.selected)

    file_ids.forEach((item) => {
      axios.post(import.meta.env.VITE_API_URL + "/storage/get_download_uri", {
          object_id: item.id,
        },
        {
          headers: {
            Authorization: localStorage.getItem('jwt'),
          }}

      ).then(async res => {
        console.log(res.data);
        const presignedUrl: string = res.data;

        const fileResponse = await axios.get(presignedUrl, {
          responseType: 'blob',
        });

        const blob = fileResponse.data;
        const blobUrl = URL.createObjectURL(blob);

        const a = document.createElement('a');
        a.href = blobUrl;
        a.download = item.name;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(blobUrl);
      }).catch(err => {
        if (err.status === 401) {
          logout()
        }
        console.log(err);
        error_message.value = err.message;
        throw err;
      })
    })
  }

  const selectedToMove = () => {
    move_objects.value.files = files.value.filter((item: File) => item.selected)
    move_objects.value.folders = folders.value.filter((item: Folder) => item.selected)
    move_objects.value.is_moving = true;
  }

  const selectedDelete = async () => {
    const folders_to_delete: object[] = folders.value
      .filter((item) => item.selected)
      .map((item: Folder) => item.id);
    const files_to_delete: object[] = files.value
      .filter((item) => item.selected)
      .map((item: File) => item.id)

    if (folders_to_delete.length > 0) {
      deleteFolders(folders_to_delete)
      getFolders(current_folder_parent.value?.id);
    }
    if (files_to_delete.length > 0) {
      deleteFiles(files_to_delete)
      getFiles(current_folder_parent.value?.id);
    }
  }

  const moveToDirectory = (directory: object|undefined, files: (object | undefined)[], folders: (object | undefined)[]) => {
    move_objects.value = {is_moving: false, files: [], folders:[]}
    if (!directory) return
    if (files.length > 0) {
      axios.post(import.meta.env.VITE_API_URL + "/storage/move_objects", {
          parent_id: directory,
          object_ids: files
        },
        {
          headers: {
            Authorization: localStorage.getItem('jwt'),
          }}

      ).then(async res => {
        console.log(res.data);
        getFiles(current_folder_parent.value?.id);
      }).catch(err => {
        if (err.status === 401) {
          logout()
        }
        console.log(err);
        error_message.value = err.message;
        throw err;
      })
    }
    if (folders.length > 0) {
      axios.post(import.meta.env.VITE_API_URL + "/storage/move_folders", {
          parent_id: directory,
          folder_ids: folders
        },
        {
          headers: {
            Authorization: localStorage.getItem('jwt'),
          }}
      ).then(async res => {
        console.log(res.data);
        getFolders(current_folder_parent.value?.id);
      }).catch(err => {
        if (err.status === 401) {
          logout()
        }
        console.log(err);
        error_message.value = err.message;
        throw err;
      })
    }
  }

  getFolders();
  getFiles();

  return { folders, files, current_folder_parent, headers, error_message, info_message, move_objects, path, sorted_by,
    getFolders, getFiles, goUp, getDownloadUri, createFolder, deleteFolders, deleteFiles, refreshData, filterValues, goTo,
    renameFolder, renameFile,
    selectBetween, selectedDownload, selectedToMove, selectedDelete, moveToDirectory, sortValues
  }
})
