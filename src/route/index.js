import {createRouter, createWebHistory} from 'vue-router'
const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: "Layout",
            component: () => import("../Layout.vue"),
            redirect:'/convert',
            children: [
                {
                    path: '/convert',
                    name: "Convert",
                    component: () => import("../views/Convert.vue")
                },
                {
                    path: '/compression',
                    name: "Compression",
                    component: () => import("../views/Compression.vue")
                },
                {
                    path: '/dpi',
                    name: "DPI",
                    component: () => import("../views/Dpi.vue")
                },
                {
                    path: '/sr',
                    name: "SuperResolution",
                    component: () => import("../views/SuperResolution.vue")
                },
                {
                    path: '/detection',
                    name: "Detection",
                    component: () => import("../views/Detection.vue")
                },
            ]
        }
    ]
});
export default router;