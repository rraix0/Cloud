<script setup lang="ts">
import explorerSidebarComponent from '@/views/panel/components/explorerSidebarComponent.vue'
import explorerComponent from '../components/explorerComponent.vue';
import explorerNavComponent from '../components/explorerNavComponent.vue';

</script>

<template>
  <div class="w-full min-h-screen flex flex-col">
    <explorerNavComponent />
    <div class="flex-1 w-full flex">
      <explorerSidebarComponent />
      <explorerComponent />
    </div>
  </div>




</template>

<style scoped></style>
