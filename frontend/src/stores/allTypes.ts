
import type { Ref } from 'vue'
import { CalendarDate, type DateValue } from '@internationalized/date'

export interface Thing {
  id: {
    String: string
  },
  tb: string,
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
  share_id: [object],
  updated_at: Date,
  user_id: Thing,
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
  share_id: [object],
  size: number,
  tags: string[],
  updated_at: Date,
  user_id: Thing,
  thumbnail: number[][] | null, // change it to bytes
}

export interface ShareData {
  share_as: "url" | "user",
  share_permissions_list?: string[],
  share_expiration_date?: any,
  share_description?: string | undefined,
  share_user_email?: string | undefined,
}