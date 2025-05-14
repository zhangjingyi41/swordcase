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
    link: string,
    size: number,
}

function defaultTabList(): Array<TabItem> {
    const tabList: Array<TabItem> = new Array<TabItem>();
    const tabItem:TabItem = {
        name: "default",
        sq:0,
        childList:[]
    }
    tabList.push(tabItem);
    return tabList;
}

export const useTabListStore = defineStore("appList", {
    state: ()=>({
        tabList: defaultTabList(),
        currentTab: "default",
    }),
    actions: {
        setTabList(tabList:Array<TabItem>){
            this.tabList = tabList;
        }
    }

})