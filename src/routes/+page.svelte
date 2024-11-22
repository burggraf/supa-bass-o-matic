<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
      import Button from '$lib/components/ui/button/button.svelte';
    
      async function doSomething() {
        const output = document.getElementById('output');
        if (!output) {
          return;
        }
        output.innerHTML = "Hello, world!  I am a Svelte app!";
      }
  </script>
  
  <main class="container">
    <h1>Welcome to Supa Bass-a-matic! v0</h1>
    <div class="row">
      <p>
        This is a Tauri app built with Svelte 5 and TypeScript.
        <Button onclick={doSomething}>Click me!</Button>
      </p>
    </div>
    <div id="output"></div>
</main>
  
  <style>
    .container {
      margin: 0;
      padding-top: 10vh;
      display: flex;
      flex-direction: column;
      justify-content: center;
      text-align: center;
    }
  
    .row {
      display: flex;
      justify-content: center;
    }
  
    h1 {
      text-align: center;
    }
  </style>