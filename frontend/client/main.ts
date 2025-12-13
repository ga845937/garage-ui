import { bootstrapApplication } from "@angular/platform-browser";

import { App } from "./app/app";
import { app_config } from "./app/app.config";

bootstrapApplication(App, app_config).catch((err) => console.error(err));
