import {createMemoryHistory, createRouter} from "vue-router";
import Index from '../pages/index.vue'
import Converter from '../pages/converter.vue'
import Crop from '../pages/crop.vue'
import Settings from '../pages/settings.vue'

const routes = [
    {path: '/', component: Index, name: "Dashboard"},
    {path: '/converter', component: Converter, name: "Converter"},
    {path: '/crop', component: Crop, name: "Crop"},
    {path: '/settings', component: Settings, name: "Settings"},
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})


export default router