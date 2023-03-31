<script lang="ts">
    import { Facebook, Github, Contact as ContactCl } from "./lib/data";
    import Link from "./lib/link.svelte";
    import Contact from "./lib/contact.svelte";
    import { onMount } from "svelte";

    export let data_links: {
        links: Array<Facebook | Github | ContactCl>;
    };

    // Define a reactive statement that logs the value of data_links.links when it changes
    // $: console.log(data_links.links[0]);
</script>

<div>
    {#each data_links.links as link}
        {#if link instanceof Facebook || link instanceof Github}
            {link.link}
            <Link href={link.link} />
        {:else if link instanceof ContactCl}
            <Contact mobile={link.mobile} address={link.address} />
        {/if}
    {/each}
</div>
