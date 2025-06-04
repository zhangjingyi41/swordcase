import {defineStore} from "pinia"
import md5 from 'md5'

// type TabItem = {
//     name: string,
//     sq: number,
//     childList: Array<ChildItem>
// }

type AppItem = {
    id: string,
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
        addApp(appItem:AppItem){
            if(appItem.type=='app'){
                appItem.id = md5(`${appItem.name}-${appItem.link}`)
            }
            this.appList.push(appItem);
        },
        removeApp(appItem:AppItem){
            const index = this.appList.findIndex(item => item.name === appItem.name);
            if (index !== -1) {
                this.appList.splice(index, 1);
            }
        },
    }

})

export { useAppListStore }
export type {AppItem}