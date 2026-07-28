<script setup lang="ts">

import PathDirectoryComponent from '@/views/panel/components/small_components/pathDirectoryComponent.vue'
import TableComponent from '@/views/panel/components/explorer/file_table/tableComponent.vue'
import NotificationsComponent from '@/views/panel/components/explorer/notificationsComponent.vue'
import NavComponent from '@/views/panel/components/explorer/navComponent.vue'

</script>

<template>
  <div class="h-full max-h-full overflow-auto md:px-3">
    <!-- error / ok message -->
    <notificationsComponent />

    <!-- navbar (search, go up, new, refresh, columns -->
    <navComponent />

    <!-- path -->
    <PathDirectoryComponent />

    <!-- main file table -->
    <TableComponent />

  </div>
</template>