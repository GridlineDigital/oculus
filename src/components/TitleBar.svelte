<script lang="ts">
  // No script needed for now, just a draggable div
</script>

<!-- 
  The title bar needs to be tall enough to cover the traffic lights area 
  and provide a grab handle. 
  On macOS, traffic lights are ~28px high. 
  We use data-tauri-drag-region to make it draggable.
-->
<div 
  data-tauri-drag-region 
  class="fixed top-0 left-0 w-full h-8 z-50 bg-transparent"
>
</div>
