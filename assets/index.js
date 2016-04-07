import "../node_modules/materialize-css/dist/css/materialize.css";
import "imports?$=>$!../node_modules/materialize-css/dist/js/materialize.js";
import "./styles/style.css";
import "./index.html";

import riot from "riot";

import "./tags/yggdrasil.tag";

riot.mount('*');

riot.route.base("/");
riot.route.start(true);
