import { createRouter ,createWebHashHistory,createWebHistory} from "vue-router"
import hijack from "@/components/pollute/index.vue"
import speed from "@/components/speed/index.vue"
import about from "@/components/about/index.vue"
import settings from "@/components/settings/index.vue"
// 2. 定义一些路由
// 每个路由都需要映射到一个组件。
// 我们后面再讨论嵌套路由。
const routes = [
  // { path: '/', component: overview,name:"overview" },
  { path: '/pollute', component: hijack ,name:"pollute"},
  {path:'/',component:speed,name:"speed"},
  {path:'/about',component:about,name:"about"},
  {path:'/settings',component:settings,name:"settings"}
]

const router = createRouter({
  history:createWebHistory(),
  routes, 
})

export default router