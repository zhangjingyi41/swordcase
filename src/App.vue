<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Plus } from "@element-plus/icons-vue";
import {tabListStore} from "@/stores/app"


const launchAppMsg = ref("");
const data = ref("")


async function launchApp() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    // launchAppMsg.value = await invoke("launch_app", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    // launchAppMsg.value = await invoke("launch_app", { path: "C:\\Users\\Administrator\\Desktop\\阿里云盘.lnk" });
    launchAppMsg.value = await invoke("launch_app_v2", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    data.value = await invoke("load_app_list")
}
onMounted(async () => {
   data.value = await invoke("load_app_list")
});

</script>

<template>
    <div class="box">
        <el-container>
            <el-header>Header</el-header>
            <el-main class="main">
                <el-button type="primary" :icon="Plus" v-on:click="launchApp" />
                <p>{{ launchAppMsg }}</p>
                <p>{{ data }}</p>
            </el-main>
        </el-container>
    </div>
</template>

<style scoped>
.box {
    width: 100%;
    height: 100%;
}

.main {
    border: 1px solid #ccc;
}
</style>