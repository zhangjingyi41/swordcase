<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Plus } from "@element-plus/icons-vue";
import {useTabListStore} from "@/stores/app"
import {Action, ElMessageBox} from "element-plus"
import { appWindow } from "@tauri-apps/api/window";


const launchAppMsg = ref("");
const data = ref("")
const tabListStore = useTabListStore()


async function launchApp() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    // launchAppMsg.value = await invoke("launch_app", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    // launchAppMsg.value = await invoke("launch_app", { path: "C:\\Users\\Administrator\\Desktop\\阿里云盘.lnk" });
    launchAppMsg.value = await invoke("launch_app_v2", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    // data.value = await invoke("load_app_list")
}

// 加载应用列表
async function loadAppList() {
    let app_list_result:TauriResult = await invoke("load_app_list")
    if(!app_list_result.status){
        ElMessageBox.alert(`加载应用列表时出现问题：${app_list_result.info}`, "提示", {
            confirmButtonText: "退出程序",
            type: "error",
            callback:()=>{
                appWindow.close()
            }
        })
        return
    }
    tabListStore.setTabList(app_list_result.data)
}
onMounted(async () => {
    await loadAppList()
   
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