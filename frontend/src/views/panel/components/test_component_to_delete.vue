<script setup lang="ts">
import { ref } from "vue";

const file = ref<File | null>(null);
const url = ref("");
const status = ref("");

function onFileChange(e: Event) {
  const target = e.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    file.value = target.files[0] ?? null;
  }
}

async function uploadFile() {
  if (!file.value || !url.value) {
    status.value = "❌ Podaj URL i wybierz plik";
    return;
  }

  status.value = "⏳ Upload...";
  try {
    await fetch(url.value, {
      method: "PUT",
      body: file.value,
      headers: {
        "Content-Type": file.value.type
      }
    });
    status.value = "✅ Upload zakończony";
  } catch (err) {
    console.error(err);
    status.value = "❌ Błąd przy uploadzie";
  }
}
</script>

<template>
  <div class="p-4 space-y-4">
    <div>
      <label>Presigned URL:</label>
      <input
        v-model="url"
        type="text"
        placeholder="wklej tu link"
        class="border p-1 w-full"
      />
    </div>

    <div>
      <label>Plik:</label>
      <input type="file" @change="onFileChange" />
    </div>

    <button
      @click="uploadFile"
      class="bg-blue-500 text-white px-3 py-1 rounded"
    >
      Upload
    </button>

    <div>{{ status }}</div>
  </div>
</template>
