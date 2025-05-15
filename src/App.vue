<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Plus } from "@element-plus/icons-vue";
import { useTabListStore, TabItem } from "@/stores/app"
import { Action, ElMessageBox, TabsPaneContext } from "element-plus"
import { appWindow } from "@tauri-apps/api/window";


const launchAppMsg = ref("");
const data = ref("")
const tabListStore = useTabListStore()
const activeTab = ref('')


async function launchApp() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    // launchAppMsg.value = await invoke("launch_app", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    // launchAppMsg.value = await invoke("launch_app", { path: "C:\\Users\\Administrator\\Desktop\\阿里云盘.lnk" });
    launchAppMsg.value = await invoke("launch_app", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
    // data.value = await invoke("load_app_list")
}

// 加载应用列表
async function loadAppList() {
    let app_list_result: TauriResult = await invoke("load_app_list")
    if (!app_list_result.status) {
        ElMessageBox.alert(`加载应用列表时出现问题：${app_list_result.info}`, "提示", {
            confirmButtonText: "退出程序",
            type: "error",
            callback: () => {
                appWindow.close()
            }
        })
        return
    }
    console.log("加载应用列表")
    console.log(app_list_result.data)
    let app_list = JSON.parse(app_list_result.data) as Array<TabItem>
    activeTab.value = app_list[0].name
    tabListStore.setTabList(app_list)
}
function handleChangeTab(tab: TabsPaneContext, event: Event) {
    console.log(tab)
}
onMounted(async () => {
    await loadAppList()

});

</script>

<template>
    <div class="box">
        <el-container class="box">
            <el-header>
                <el-button type="primary" :icon="Plus" v-on:click="launchApp" />
                <p>{{ launchAppMsg }}</p>
                <p>{{ data }}</p>
            </el-header>
            <!-- 走马灯、tab标签页都不行，继续按照翻页的思路走，逻辑是分页的逻辑，样式按照走马灯 -->
            <el-main class="main">
                <div v-for="(tab, index) in tabListStore.tabList" >
                    

                </div>
                
                
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

    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 10px;
    width: 100%;
    margin: 0 auto;
}

.grid-item {
    background-color: #3498db;
    color: white;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 1.5em;
    border-radius: 5px;
    /* 确保所有盒子都是正方形 */
    aspect-ratio: 1 / 1;
}

/* 小盒子 (1x1) */
.size-1 {
    grid-column: span 1;
    grid-row: span 1;
}

/* 中等盒子 (2x2) */
.size-2 {
    grid-column: span 2;
    grid-row: span 2;
}

/* 大盒子 (4x4) */
.size-3 {
    grid-column: span 4;
    grid-row: span 4;
}

</style>