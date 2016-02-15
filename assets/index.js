import "../node_modules/material-design-lite/dist/material.css";
import "../node_modules/material-design-lite/dist/material.js";
import "./styles/style.css";
import "./index.html";

import riot from "riot";

import "./tags/yggdrasil.tag";

riot.mount('*');

riot.route.base("/");
riot.route.start(true);
