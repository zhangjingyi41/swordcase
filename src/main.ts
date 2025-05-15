import { createApp } from "vue"
import App from "./App.vue"
// import "@/assets/css/main.css"
import "./assets/css/main.css"
import { createPinia } from "pinia"
import ElementPlus  from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import TabItemComponent from "@/components/TabItemComponent.vue"

const pinia = createPinia()
const app = createApp(App)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.component("TabItemComponent", TabItemComponent)
app.use(pinia)
app.use(ElementPlus)
app.mount("#app")