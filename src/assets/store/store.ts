import {defineStore} from "pinia"

type TabItem = {
    name: string,
    sq: number,
    childList: Array<ChildItem>
}

type ChildItem = {
    name: string,
    sq: number,
    type: string, // 1:文件夹 2:文件
    childList: Array<ChildItem>,
    link: string
}

export const appListStore = defineStore("appList", {
    state: ()=>({}),

})