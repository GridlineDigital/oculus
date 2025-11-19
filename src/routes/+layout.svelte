<script>
	import '../app.css';
  import TitleBar from "../components/TitleBar.svelte";
	let { children } = $props();
</script>

<TitleBar />
<div class="min-h-screen bg-neutral-950 text-neutral-50 font-sans selection:bg-primary-500/30">
  {@render children()}
</div>
