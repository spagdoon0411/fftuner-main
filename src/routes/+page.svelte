<script lang="ts">
  import { Button } from '$lib/components/ui/button'
</script>

<main class="container">
  <div class="box">
    <h1 class="text-primary-foreground">Welcome to Tauri!</h1>
    <Button>Click me</Button>
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
  }

  /* Center things vertically in box and fit */
  .box {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
    max-width: 400px;
    gap: 1rem;
  }

  h1 {
    text-align: center;
  }
</style>
