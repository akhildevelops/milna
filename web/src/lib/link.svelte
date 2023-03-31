<script lang="ts">
    export let href: string;
</script>

<div>
    <a {href} class="link" target="_blank">
        {href}
    </a>
</div>

<style>
    .link {
        text-decoration: none;
        color: #0077cc;
        font-weight: bold;
        transition: all 0.2s ease-in-out;
    }

    .link:hover {
        color: #005fa3;
        transform: scale(1.05);
    }
</style>
