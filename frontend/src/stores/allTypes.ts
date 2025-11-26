export interface Thing {
  id: {
    String: string
  },
  tb: string,
}

export interface Folder {
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
  open_dropdown_menu: boolean,
  share_as: "url" | "user" | undefined,
  share_permissions_list: string[],
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