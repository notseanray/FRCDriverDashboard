<script>
  import Greet from "$lib/Greet.svelte";
  import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
  import { writable } from "svelte/store";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  const webview = new WebviewWindow('main')
  let new_ip = "";
  let active = true;
  let init = false;
  let data = "";
  let timestamp = Date.now();
  let connected = false;
  let rotation = 0;

  let flywheelcolor = "badred";
  let flywheel_temp = 0.0;
  let flywheel_temp_f = 0.0;
  let flywheel_rpm = 0.0;

  onMount(async () => {
      // default ip
      appWindow.emit('set_ip', { message: '10.10.2.2:1735' })
      appWindow.listen('new_data', (e) => {
        if (!active) return;
        data = e.payload;
        const rpm = data.flywheel_rpm;
        let new_rotation = data.rotation_2d;
        if (new_rotation > 360) {
            new_rotation = new_rotation - Math.floor(new_rotation / 360) * 360;
        }
        rotation = new_rotation;
        if (rpm != undefined) {
            flywheel_temp_f = (data.flywheel_temp * 1.8 + 32).toFixed(1);
            flywheel_temp = data.flywheel_temp.toFixed(1);
            flywheel_rpm = rpm.toFixed(1);
            connected = true;
            timestamp = Date.now();
            if (rpm >= 5000) {
                flywheelcolor = "fuckcolor";
            }
            else if (rpm >= 2800) {
                flywheelcolor = "goodgreen";
            } else if (rpm >= 1800) {
                flywheelcolor = "midyellow";
            } else {
                flywheelcolor = "badred";
            }
        }
      })
      invoke("init_process", { window: webview, ip: '' }).then(() => {
          init = true;
      });
      setInterval(() => {
        if (Date.now() - timestamp > 2000) {
            connected = false;
        }
        if (!active) {
            data = {};
        }
      }, 1000);
  })

  // emits the `click` event with the object payload
</script>

<br />

<div class="flywheelsection">
    RPM: {flywheel_rpm <= 0.0 ? flywheel_rpm + " BAD BAD" : flywheel_rpm}
    <div class={"flywheelindicator " + flywheelcolor}>
    h
    </div>
    <br />
    TEMP: {flywheel_temp} C {flywheel_temp_f} F
</div>

<div class="climberindicator">
    SOLENOID1 DOWN
    <br />
    <div class={"climberbox " + (data.forward_solenoid ? "goodgreen" : "badred")}>
        f
    </div>
    SOLENOID2 DOWN
    <br />
    <div class={"climberbox " + (data.reverse_solenoid ? "goodgreen" : "badred")}>
        r
    </div>
</div>
<div class="compressorindicator">
    COMPRESSOR
    <div class={"compressorbox " + (data.compressor_enabled ? "goodgreen" : "badred")}>
    h
    </div>
    COMPRESSOR CURRENT:
    <br />
    {data.compressor_current == undefined ? 0.0 : data.compressor_current.toFixed(2)}
    <br />
    COMPRESSOR VOLTAGE:
    <!-- <br /> -->
    <!-- {data.compressor_voltage == undefined ? 0.0 : data.compressor_voltage.toFixed(2)} -->
</div>
<div class="motorindicator">
    FRONT LEFT voltage:
    <br />
    {data.left_front_voltage == undefined ? 0.0 : data.left_front_voltage.toFixed(2)}
    <div class={"climberbox " + (data.left_front_voltage > 2 ? "goodgreen" : "badred")}>
        r
    </div>
    BACK LEFT voltage:
    <br />
    {data.left_back_voltage == undefined ? 0.0 : data.left_back_voltage.toFixed(2)}
    <div class={"climberbox " + (data.left_back_voltage > 2 ? "goodgreen" : "badred")}>
        r
    </div>
    FRONT RIGHT voltage:
    <br />
    {data.right_front_voltage == undefined ? 0 : data.right_front_voltage.toFixed(2)}
    <div class={"climberbox " + (data.right_front_voltage > 2 ? "goodgreen" : "badred")}>
        r
    </div>
    BACK RIGHT voltage:
    <br />
    {data.right_back_voltage == undefined ? 0.0 : data.right_back_voltage.toFixed(2)}
    <div class={"climberbox " + (data.right_back_voltage > 2 ? "goodgreen" : "badred")}>
        r
    </div>
</div>
<div class="intakeindicator">
    INTAKE
    <div class={"compressorbox " + (data.intake_power > 0.0 ? "goodgreen" : "badred")}>
    h
    </div>
    POWER:
    <br />
    {data.intake_power && data.intake_power.toFixed(2)}
    <br />
    ALIVE:
    {data.intake_alive}
</div>
<div class="flexcenter middle">
    RIGHT POS:
    <br />
    {data.right_pos && data.right_pos.toFixed(2)}
    <br />
    LEFT POS:
    <br />
    {data.left_pos && data.left_pos.toFixed(2)}
    <br />
</div>
<div class="flexcenter">
    <div class="main">
        <h1>SEANBOARD</h1>
        <div class="flexcenter">
            tauri backend connection:
            <div class={init ? "goodgreen" : "badred"}>
            {init ? "connected" : "disconnected"}
            </div>
        </div>
        <div class="flexcenter">
            robot connection:
            <div class={connected ? "goodgreen" : "badred"}>
            {connected ? "connected" : "disconnected"}
            </div>
        </div>
        <div class="flexcenter">
            active:
            <br />
            <input type="checkbox" bind:checked={active} />
        </div>
        <div class="flexcenter">
            ip:
            <input bind:value={new_ip}/>
            <button on:click={() => {
                appWindow.emit('set_ip', { message: `${new_ip}:1735`})
                new_ip = "";
            }}>+</button>
        </div>
        <br />
        <img style="transform: rotate({rotation}deg)" src="/arrow.png" alt="" />
        <!-- {JSON.stringify(data, null, 2)} -->
    </div>
</div>

<style>
    .middle {
        top: 50vh;
    }
    .flex {
        display: flex;
    }
    .climberbox {
        width: 50px;
        height: 50px;
    }
    .motorindicator {
        position: absolute;
        bottom: 2vh;
    }
    .intakeindicator {
        position: absolute;
        bottom: 2vh;
        left: 20vw;
    }
    .compressorindicator {
        position: absolute;
        right: 0px;
        bottom: 2vh;
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
        top: 2vh;
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
        top: 2vh;
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
