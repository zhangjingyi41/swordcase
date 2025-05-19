<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Plus } from "@element-plus/icons-vue";
import { useTabListStore, TabItem } from "@/stores/app"
import { Action, ElMessageBox, TabsPaneContext } from "element-plus"
import { appWindow } from "@tauri-apps/api/window";
import { path } from "@tauri-apps/api";


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

const app_icon_ref = ref("")
async function getAppIcon() {
    let icon_result:TauriResult = await invoke("get_app_icon", {path:"D:\\Develop\\IDE\\cursor\\Cursor.exe"})
    console.log(icon_result)
    if(icon_result.status){
        app_icon_ref.value = icon_result.data
    }else{
        ElMessageBox.alert(`获取应用图标时出现问题：${icon_result.info}`, "提示", {
            confirmButtonText: "退出程序",
            type: "error",
            callback: () => {
                appWindow.close()
            }
        })
    }
}

function handleChangeTab(tab: TabsPaneContext, event: Event) {
    console.log(tab)
}
onMounted(async () => {
    await loadAppList()
    await getAppIcon()
});

</script>

<template>
    <div class="box">
        <el-container class="box">
            <el-header>
                <img :src="app_icon_ref" alt="" >
            </el-header>
            <!-- 走马灯、tab标签页都不行，继续按照翻页的思路走，逻辑是分页的逻辑，样式按照走马灯 -->
            <el-main class="main">
                <div class="card-container">
                    <el-card 
                        v-for="(item, item_index) in tabListStore.tabList[0].childList" 
                        :key="item_index"
                        class="square-card">
                        <p>{{ item.name }}</p>
                    </el-card>
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
    width: 100%;
    padding: 10px;
}

.card-container {
    display: flex;
    flex-wrap: wrap;
    gap: 15px;
    justify-content: flex-start;
}

.square-card {
    width: 100px;
    height: 100px;
    margin-bottom: 10px;
}
</style>