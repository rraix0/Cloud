<script setup lang="ts">
import { Check, Search } from "lucide-vue-next"

import { Combobox, ComboboxAnchor, ComboboxEmpty, ComboboxGroup, ComboboxInput, ComboboxItem, ComboboxItemIndicator, ComboboxList } from "@/components/ui/combobox"

const frameworks = [
  { value: "next.js", label: "Next.js" },
  { value: "sveltekit", label: "SvelteKit" },
  { value: "nuxt", label: "Nuxt" },
  { value: "remix", label: "Remix" },
  { value: "astro", label: "Astro" },
]
import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from '@/components/ui/menubar'

</script>

<template>
 <div class="w-full h-min flex border-b">
   <Combobox by="label">
     <ComboboxAnchor class="w-full border-r">
       <div class="relative w-full max-w-sm items-center">
         <ComboboxInput class="pl-9" :display-value="(val) => val?.label ?? ''" placeholder="Select framework..." />
         <span class="absolute start-0 inset-y-0 flex items-center justify-center px-3">
          <Search class="size-4 text-muted-foreground" />
        </span>
       </div>
     </ComboboxAnchor>

     <ComboboxList>
       <ComboboxEmpty>
         No framework found.
       </ComboboxEmpty>

       <ComboboxGroup>
         <ComboboxItem
           v-for="framework in frameworks"
           :key="framework.value"
           :value="framework"
         >
           {{ framework.label }}

           <ComboboxItemIndicator>
             <Check class="ml-auto size-4" />
           </ComboboxItemIndicator>
         </ComboboxItem>
       </ComboboxGroup>
     </ComboboxList>
   </Combobox>


   <Menubar>
     <MenubarMenu>
       <MenubarTrigger>File</MenubarTrigger>
       <MenubarContent>
         <MenubarItem>
           New Tab <MenubarShortcut>⌘T</MenubarShortcut>
         </MenubarItem>
         <MenubarItem>New Window</MenubarItem>
         <MenubarSeparator />
         <MenubarItem>Share</MenubarItem>
         <MenubarSeparator />
         <MenubarItem>Print</MenubarItem>
       </MenubarContent>
     </MenubarMenu>
   </Menubar>

 </div>
</template>

<style scoped></style>
