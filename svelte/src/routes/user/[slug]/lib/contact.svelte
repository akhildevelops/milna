<script lang="ts">
    export let address: string;
    export let mobile: string;
</script>

<div class="container">
    <div class="address">{address}</div>
    <div class="mobileNumber">{mobile}</div>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: #f5f5f5;
        padding: 16px;
        border-radius: 8px;
        box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
        opacity: 0;
        animation: fadeIn 0.5s ease-out forwards;
    }

    .address {
        font-size: 18px;
        margin-bottom: 8px;
        color: #333;
        text-align: center;
        text-transform: uppercase;
        letter-spacing: 2px;
    }

    .mobileNumber {
        font-size: 16px;
        color: #666;
        margin-top: 8px;
        text-align: center;
    }

    .container:hover {
        transform: translateY(-4px);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
</style>
