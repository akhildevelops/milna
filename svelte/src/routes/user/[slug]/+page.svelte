<script lang="ts">
    import Links from "./links.svelte";
    import { links_loader } from "./lib/load";
    import type { Facebook, Github, Contact } from "./lib/data";
    import Name from "./lib/name.svelte";
    let data: Promise<{
        name: string;
        links: Array<Facebook | Github | Contact>;
    }> = links_loader();
</script>

<div>
    {#await data}
        ...waiting
    {:then data_res}
        <Name name={data_res.name} />
        <div class="links">
            <Links data_links={{ links: data_res.links }} />
        </div>
    {/await}
</div>
