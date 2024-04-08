import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap-icons/font/bootstrap-icons.min.css";

import "./assets/styles.sass";
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app")!,
});

export default app;
