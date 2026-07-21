import E404 from "./components/E404";
import Index from "./components/Index";

const routes = [
  {
    path: "*",
    component: E404,
  },
  {
    path: "/",
    component: Index,
  },
];

export default routes;
