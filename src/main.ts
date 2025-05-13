import { createApp } from "vue"
import App from "./App.vue"
// import "@/assets/css/main.css"
import "./assets/css/main.css"
import { createPinia } from "pinia"
import ElementPlus  from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

const pinia = createPinia()
const app = createApp(App)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

app.use(pinia)
app.use(ElementPlus)
app.mount("#app")