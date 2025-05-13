<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Plus } from "@element-plus/icons-vue";

const greetMsg = ref("");
const launchAppMsg = ref("");
const name = ref("");

async function greet() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("greet", { name: name.value });
}

async function launchApp() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    // launchAppMsg.value = await invoke("launch_app", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    // launchAppMsg.value = await invoke("launch_app", { path: "C:\\Users\\Administrator\\Desktop\\阿里云盘.lnk" });
    launchAppMsg.value = await invoke("launch_app_v2", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });

}

</script>

<template>
    <div class="common-layout">
        <el-container>
            <el-header>Header</el-header>
            <el-main class="main">
                <el-button type="primary" :icon="Plus" v-on:click="launchApp" />
                <p>{{ launchAppMsg }}</p>
            </el-main>
        </el-container>
    </div>
    <!-- <main class="container">

        <form class="row" @submit.prevent="greet">
            <input id="greet-input" v-model="name" placeholder="Enter a name..." />
            <button type="submit">Greet</button>
        </form>
        <p>{{ greetMsg }}</p>
    </main> -->
</template>

<style scoped>
.common-layout {
    width: 100%;
    height: 100%;
}

</style>