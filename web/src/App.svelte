<script>
  import QRCode from "qrcode-generator";
  import { onMount } from "svelte";
  let qrCodeData;

  async function fetchQRCodeData() {
    const name = "Akhil"; // Replace this with dynamic name input
    const response = await fetch(`http://localhost:8080/api/userinfo/${name}`);
    const data = await response.json();
    let qrCodeString = "";
    data.forEach((profile) => {
      Object.values(profile).forEach((value) => {
        qrCodeString += `${value.link}\n`;
      });
    });
    qrCodeData = qrCodeString;
    generateQRCode(qrCodeData);
  }

  function generateQRCode(qrCodeData) {
    const qr = QRCode(0, "M");
    qr.addData(qrCodeData);
    qr.make();
    const qrCodeElement = document.getElementById("qrcode");
    qrCodeElement.innerHTML = qr.createSvgTag();
  }

  onMount(() => {
    fetchQRCodeData();
  });
</script>

<template>
  <div>
    <h1>Scan QR Code to get social media links</h1>
    {#if qrCodeData}
      <div id="qrcode" />
    {:else}
      <p>Loading...</p>
    {/if}
  </div>
</template>
