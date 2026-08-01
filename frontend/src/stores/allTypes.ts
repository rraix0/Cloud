
import type { Ref } from 'vue'
import { CalendarDate, type DateValue } from '@internationalized/date'

export interface Thing {
  id: {
    String: string
  },
  tb: string,
}

export interface SharedData {
  id: Thing,
  permissions: string[],
  description?: string,
  expiration_date?: Date,
  link_token?: string,
  share_user_id?: Thing[],
  link_copied?: boolean,
}

export interface Folder {
  type: "folder",
  open_dropdown_menu: boolean,
  share_as: "url" | "user" | undefined,
  share_permissions_list: string[],
  selected: boolean,
  show: boolean,
  created_at: Date,
  id: Thing,
  name: string,
  parent_id: Thing,
  share_id: Thing[],
  updated_at: Date,
  user_id: Thing,
  share?: SharedData[],
}

export interface File {
  type: "file",
  open_dropdown_menu: boolean,
  selected: boolean,
  show: boolean,
  created_at: Date,
  id: Thing,
  metadata: object,
  name: string,
  parent_id: Thing,
  share_id: Thing[],
  size: number,
  tags: string[],
  updated_at: Date,
  user_id: Thing,
  thumbnail: number[][] | null, // change it to bytes
  share?: SharedData[],
}

export interface ShareDataForm {
  share_as: "url" | "user",
  share_permissions_list?: string[],
  share_expiration_date?: any,
  share_description?: string | undefined,
  share_user_email?: string | undefined,
}