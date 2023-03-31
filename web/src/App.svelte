<script lang="ts">
    import Links from "./links.svelte";
    import { links_loader } from "./lib/load";
    import type { Facebook, Github, Contact } from "./lib/data";
    import Name from "./lib/name.svelte";
    let data: Promise<Array<Facebook | Github | Contact>> = links_loader();
</script>

<div>
    <Name name="Akhil" />
    {#await data}
        ...waiting
    {:then links}
        <!-- {console.log(links)} -->
        <div class="links">
            <Links data_links={{ links: links }} />
        </div>
    {/await}
</div>
