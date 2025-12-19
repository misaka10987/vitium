// @refresh reload
import { mount, StartClient } from "@solidjs/start/client";

import 'solid-devtools' // for development only

mount(() => <StartClient />, document.getElementById("app")!);
