<script setup lang="ts">
import explorerSidebarComponent from '@/views/panel/components/explorerSidebarComponent.vue'
import explorerComponent from '../components/explorerComponent.vue';
import explorerNavComponent from '../components/explorerNavComponent.vue';

</script>

<template>
  <div class="w-full h-screen flex flex-col">
    <!--
    <explorerNavComponent />
      <explorerSidebarComponent />
    </div>
    <div class="flex-1 w-full flex ">
    -->
      <explorerComponent />
  </div>




</template>

<style scoped></style>
