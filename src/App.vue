<script setup lang="ts">
import { ref, onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Plus,Picture as IconPicture } from "@element-plus/icons-vue";
import { useAppListStore, AppItem } from "@/stores/app"
import { Action, Column, DropdownInstance, ElMessageBox, TabsPaneContext, ElText, TableInstance } from "element-plus"
import { appWindow } from "@tauri-apps/api/window";
import { path, window } from "@tauri-apps/api";


const launchAppMsg = ref("");
const data = ref("")
const appListStore = useAppListStore()
const activeTab = ref('')
const dropdownMenu = ref<DropdownInstance>()
const dropdownItemList = ref(new Array<DropdownItem>())

// 添加位置状态
const menuPosition = reactive({
    x: 0,
    y: 0
})

async function launchApp() {
    launchAppMsg.value = await invoke("launch_app", { path: "D:\\3_14@17837140378_1811252a-a91d-4b72-bc71-a801a156702a.wav" });
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
    let app_list = JSON.parse(app_list_result.data) as Array<AppItem>
    activeTab.value = app_list[0].name
    appListStore.setAppList(app_list)
}

const app_icon_ref = ref("")
async function getAppIcon() {
    let icon_result:TauriResult = await invoke("get_app_icon", {path:"C:\\Program Files (x86)\\NeatReader\\NeatReader.exe"})
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
const add_app_dialog_visible = ref(false)

// 本地已安装应用列表——多选表格
const local_app_list = ref<Array<any>>([])
const selected_local_app_list = ref<Array<any>>([])
const selected_local_app_ref = ref<TableInstance>()
async function addAppInit() {
    let app_list:TauriResult = await invoke("get_installed_apps")
    console.log(111,app_list)
    add_app_dialog_visible.value = true
    local_app_list.value = app_list.data as Array<any>
}
/**
 * 处理表格选中行变化事件
 * @param val 选中的应用列表
 */
function handleTableSelectionChange(val:any[]){
    selected_local_app_list.value = val
}
/**
 * 处理添加本地应用确认事件
 */
function handleConfirmAddLocalApp(){
    // 将被选中的app添加到应用列表中
    if(selected_local_app_list.value.length > 0){
        console.log(selected_local_app_list.value)
        selected_local_app_list.value.forEach((item:any) => {
            let app_item:AppItem = {
                name: item.name,
            }
        })
    }
    add_app_dialog_visible.value = false
}
type DropdownItem = {
    label: string
    icon?: any
    disabled: boolean
    command: string
}
async function handleClickMenu(command: string | number | object) {
    switch (command) {
        case "open_app":
            console.log("打开应用")
            break
        case "remove":
            console.log("移除应用")
            break
        case "open_directory":
            console.log("打开目录")
            break
        case "rename":
            console.log("重命名")
            break
        case "add_app":
            console.log("添加应用")
            
            await addAppInit()
            break
        case "new_folder":
            console.log("新建文件夹")
            break
        default:
            break
    }
}
function showClickMenu(event: MouseEvent, item_type: string = "default") {
    event.preventDefault()
    event.stopPropagation()
    
    // 设置下拉菜单位置
    menuPosition.x = event.clientX
    menuPosition.y = event.clientY
    
    // 如果需要调整位置以避免菜单超出屏幕，可以添加
    // 如果接近右边界，向左偏移
    // if (menuPosition.x + 200 > window.innerWidth) {
    //   menuPosition.x = window.innerWidth - 200
    // }
    switch(item_type){
        case "app":
            dropdownItemList.value = [
                { label: "打开", command: "open_app", disabled:false},
                { label: "移除", command: "remove", disabled:false},
            ]
            break
        case "directory":
            dropdownItemList.value = [
                { label: "打开", command: "open_directory",disabled:false},
                { label: "移除", command: "remove",disabled:false},
                { label: "重命名", command: "rename",disabled:false},
                { label: "添加应用", command: "add_app",disabled:false},
            ]
            break
        default:
            dropdownItemList.value = [
                { label: "新建文件夹", command: "new_folder",disabled:false},
                { label: "添加应用", command: "add_app",disabled:false},
            ]
            break
    }
    
    // 显示菜单
    dropdownMenu.value?.handleOpen()
    console.log(event)
    console.log("右键菜单")
}
onMounted(async () => {
    await loadAppList()
    // await getAppIcon()
});

</script>

<template>
    <div class="box">
        <el-container class="box" @click.right="showClickMenu">
            <!-- <el-header>
                <img :src="app_icon_ref" alt="" >
                
            </el-header> -->
            <!-- 走马灯、tab标签页都不行，继续按照翻页的思路走，逻辑是分页的逻辑，样式按照走马灯 -->
            <el-main class="main">
                <el-dropdown ref="dropdownMenu" trigger="contextmenu" placement="bottom-start" @command="handleClickMenu"
                            :style="{position: 'fixed', left: menuPosition.x + 'px', top: menuPosition.y + 'px'}">
                    <span></span>
                    <template #dropdown>
                        <el-dropdown-menu>
                            <!-- <el-dropdown-item>Action 1</el-dropdown-item>
                            <el-dropdown-item>Action 2</el-dropdown-item> -->
                            <el-dropdown-item 
                                :command="dropdownItem.command"
                                :disabled="dropdownItem.disabled"
                                v-for="(dropdownItem, index) in dropdownItemList">
                                {{ dropdownItem.label }}
                            </el-dropdown-item>
                        </el-dropdown-menu>
                    </template>
                </el-dropdown>
                <div class="card-container">
                    <!-- <el-card 
                        v-for="(item, item_index) in tabListStore.tabList[0].childList" 
                        :key="item_index"
                        class="square-card">
                        <p>{{ item.name }}</p>
                    </el-card> -->
                    <div
                        v-for="(item, item_index) in appListStore.appList" 
                        :key="item_index"
                        class="square-card"
                        @click.right="showClickMenu($event,item.type)">
                        <p v-if="item.type=='app'">{{ item.name }}</p>
                        <p v-else>文件夹</p>
                    </div>
                </div>
            </el-main>
        </el-container>
        
    </div>
    <el-dialog v-model="add_app_dialog_visible" :show-close="false" width="800" align-center>
        
        <!-- <el-table-v2 :columns="table_column_list" :data="local_app_list" :width="768" :height="650"/> -->
        <el-table
            ref="selected_local_app_ref"
            :data="local_app_list"
            row-key="name"
            style="width: 100%;height: 450px;"
            @selection-change="handleTableSelectionChange"
        >
            <el-table-column type="selection" width="55" />
             <el-table-column label="应用名称" prop="name">
                <template #default="{ row }">
                    <div style="display: flex; align-items: center; gap: 8px;">
                        <el-image :src="row.icon" style="width: 20px; height: 20px; object-fit: cover;">
                            <template #error>
                                <div class="image-slot">
                                    <el-icon><icon-picture /></el-icon>
                                </div>
                            </template>
                        </el-image>
                        <el-text>{{ row.name }}</el-text>
                    </div>
                </template>
             </el-table-column>
            <el-table-column property="path" label="路径" />
        </el-table>
        <template #footer>
            <div class="my-header">
                <el-button type="primary" @click="handleConfirmAddLocalApp">
                    <el-icon class="el-icon--left"><CircleCheck /></el-icon>
                    确定
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<style scoped>
.box {
    width: 100%;
    height: 100%;
}

.main {
    border: 1px solid #ccc;
    width: 100%;
    padding: 0;
}

.card-container {
    display: flex;
    flex-wrap: wrap;
    gap: 15px;
    justify-content: flex-start;
    margin: 0 auto; /* 设置左右外边距为auto，实现水平居中 */
    width: 905px;
}

.square-card {
    width: 100px;
    height: 100px;
    margin-bottom: 10px;
    background-color: greenyellow;
}
</style>