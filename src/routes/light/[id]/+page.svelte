<script lang="ts">
    import LightDetail from "../../../components/LightDetail.svelte";
    import type { PageData } from './$types';

    let { data } = $props<{ data: PageData }>();
</script>

<LightDetail light={data.light} />
