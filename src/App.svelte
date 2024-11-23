<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
	import Button from '$lib/components/ui/button/button.svelte';
</script>

<main class="container">
  <h1>Welcome to supa-bass-o-matic!</h1>
  <div class="row">
    <p>
      This is a Tauri app built with Svelte 5 and TypeScript.
      <Button onclick={async () => await invoke('greet', { name: 'world' })}>Click me!</Button>
    </p>
  </div>
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
