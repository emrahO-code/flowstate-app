import { defineConfig } from 'vite';

export default defineConfig({
  // Configure the build output location
  build: {
    // This tells Vite to output the final, built files one level up in the 'dist' folder.
    outDir: '../dist', 
    emptyOutDir: true, // Clean the dist folder before building
  },
  // Ensure we are using the correct base path for the webview to load files
  base: './', 
});