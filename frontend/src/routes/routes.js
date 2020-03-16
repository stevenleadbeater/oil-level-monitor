import oil from '../assets/oil.png';
import home from '../assets/home.png';
import Home from "../home/Home";
import Oil from "../oil/Oil";

const routes = [
    {
        path: "/oil",
        name: "Oil",
        icon: oil,
        component: Oil
    },
    {
        path: "/",
        name: "Home",
        icon: home,
        component: Home
    },
];
export default routes;