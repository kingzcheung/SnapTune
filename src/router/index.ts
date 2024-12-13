import {createMemoryHistory, createRouter} from "vue-router";

const routes = [
    {path: '/', component: ()=>import('@/pages/index.vue'), name: "Dashboard"},
    {path: '/converter', component: ()=>import('@/pages/converter.vue'), name: "Converter"},
    {path: '/crop', component: ()=>import('@/pages/crop.vue'), name: "Crop"},
    {path: '/settings', component: ()=>import('@/pages/settings.vue'), name: "Settings"},
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})


export default router