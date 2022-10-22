<script>
  import Greet from "$lib/Greet.svelte";
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
  import { writable } from "svelte/store";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  const ip = writable("");
  const webview = new WebviewWindow('main')
  let new_ip = "";
  let init = false;
  let data = "";

  let flywheelcolor = "goodgreen";
  let flywheel_rpm = 0;

  onMount(async () => {
      appWindow.emit('set_ip', { message: '127.0.0.1' })
      ip.subscribe(d => {
        webview.emit('set_ip', {
            message: d,
        })
      })
      appWindow.listen('new_data', (e) => {
        data = e.payload;
        const rpm = data.flywheel_rpm;
        flywheel_rpm = rpm;
        if (rpm != undefined) {
            if (rpm >= 6000) {
                flywheelcolor = "fuckcolor";
            }
            else if (rpm >= 4000) {
                flywheelcolor = "goodgreen";
            } else if (rpm >= 3000) {
                flywheelcolor = "midyellow";
            } else {
                flywheelcolor = "badred";
            }
        }
      })
      // webview.listen('new_data', (e) => {
      //   console.log(e.payload)
      // })
      invoke("init_process", { window: webview, ip: '' }).then(() => {
          init = true;
      });
    console.log("test")
  })

  // emits the `click` event with the object payload
</script>

<div class="flexcenter">
    <div class="main">
        <h1>SEANBOARD</h1>
        <div class="flexcenter">
            backend connection:
            <div class={init ? "goodgreen" : "badred"}>
            {init ? "connected" : "disconnected"}
            </div>
        </div>
        <br />
        {JSON.stringify(data, null, 2)}
    </div>
</div>
<br />

<div class="flywheelsection">
    RPM: {flywheel_rpm == 0 ? "0 BAD BAD" : flywheel_rpm}
    <div class={"flywheelindicator " + flywheelcolor} />
</div>

<div class="climberindicator">
    FORWARD SOLENOID
    <br />
    <div class={"climberbox " + (data.forward_solenoid ? "goodgreen" : "badred")}>
        foward
    </div>
    REVERSE SOLENOID
    <br />
    <div class={"climberbox " + (data.reverse_solenoid ? "goodgreen" : "badred")}>
        reverse
    </div>
</div>
<div class="compressorindicator">
    COMPRESSOR ACTIVE
    <div class={"compressorbox " + (data.compressor_enabled ? "goodgreen" : "badred")}>
    h
    </div>
    COMPRESSOR CURRENT: {data.compressor_current}
</div>
<!-- <input bind:value={new_ip} /> -->
<!-- <button on:click={() => { -->
<!--     console.log("button") -->
<!--     appWindow.emit('set_ip', { message: new_ip }) -->
<!-- }}>+</button> -->

<style>
    .flex {
        display: flex;
    }
    .climberbox {
        width: 50px;
        height: 50px;
    }
    .compressorindicator {
        position: absolute;
        right: 0px;
        bottom: 0px;
    }
    .compressorbox {
        width: 50px;
        height: 50px;
    }
    .flexcenter {
        display: flex;
        text-align: center;
        justify-content: center;
    }
    .main {
        width: 50vw;
    }
    .flywheelsection {
        position: absolute;
        font-size: 30px;
    }
    .flywheelindicator {
        width: 100px;
        height: 100px;
    }
    .climberindicator {
        width: 200px;
        height: 100px;
        position: absolute;
        right: 0px;
    }
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
  .fuckcolor {
    color: black;
    background-color: black;
  }
  .goodgreen {
      color: #20D54D;
      background-color: #20D54D;
  }
  .midyellow {
    color: yellow;
      background-color: yellow;
  }
  .badred {
    color: red;
      background-color: red;
  }
</style>
