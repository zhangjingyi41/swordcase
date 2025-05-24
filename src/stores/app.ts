import {defineStore} from "pinia"

// type TabItem = {
//     name: string,
//     sq: number,
//     childList: Array<ChildItem>
// }

type AppItem = {
    name: string,
    sq: number,
    type: string, // 1:文件夹 2:文件
    childList: Array<AppItem>,
    link: string,
    size: number,
}

function defaultAppList(): Array<AppItem> {
    const appList: Array<AppItem> = new Array<AppItem>();
    // const appItemInfo:AppItem = {
    //     name: "default",
    //     sq:0,
    //     childList:[]
    // }
    // appList.push(appItemInfo);
    return appList;
}

const useAppListStore = defineStore("appList", {
    state: ()=>({
        appList: defaultAppList(),
        currentTab: "default",
    }),
    actions: {
        setAppList(appList:Array<AppItem>){
            this.appList = appList;
        },
        
    }

})

export { useAppListStore }
export type {AppItem}