import oil from './oil.png';
import home from './home.png';
import Home from "./Home";
import Oil from "./Oil";

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