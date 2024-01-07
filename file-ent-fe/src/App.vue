<template>
  <div id="app">
    <button @click="selectFile">Select File</button>
    <input type="text" v-model="userQuestion" placeholder="Enter your question" />
    <button @click="sendFile" :disabled="!selectedFilePath">Send File</button>
    <p v-if="selectedFilePath">Selected File: {{ selectedFilePath }}</p>
  </div>
</template>

<script>
import { dialog, invoke } from '@tauri-apps/api';

export default {
  name: 'App',
  data() {
    return {
      selectedFilePath: null,
      userQuestion: null,
    };
  },
  methods: {
    async selectFile() {
      try {
        const selectedPaths = await dialog.open({ multiple: false });
        this.selectedFilePath = Array.isArray(selectedPaths) ? selectedPaths[0] : selectedPaths;
      } catch (error) {
        console.error('Error opening file dialog:', error);
      }
    },
    async sendFile() {
      if (this.selectedFilePath) {
        console.log('Sending file:', this.selectedFilePath);
        try {
          const response = await invoke('handle_file', { args: { file_path: this.selectedFilePath, question: this.userQuestion  } });
          console.log(response);
        } catch (error) {
          console.error('Error calling Tauri API:', error);
        }
      }
    }
  },
};
</script>

<style>
/* Global Styles */
body {
  background-color: #1a1a1a; /* Dark background */
  color: #fff; /* Light text color for readability */
  font-family: Arial, sans-serif;
}

/* App specific styles */
#app {
  text-align: center;
  padding: 20px;
}

button {
  background-color: #333; /* Darker background for buttons */
  color: #fff; /* Light text color for buttons */
  border: none;
  padding: 10px 20px;
  margin: 5px;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #555; /* Slightly lighter on hover */
}

button:disabled {
  background-color: #555;
  cursor: not-allowed;
}

/* Additional styles can be added as needed */
</style>
