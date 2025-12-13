import type { Routes } from "@angular/router";

import { BucketRoute, KeyRoute, ObjectRoute } from "@shared/route-paths";

import { MainLayoutComponent } from "./presentation/layout/main-layout/main-layout.component";

// biome-ignore lint/style/useNamingConvention: singleton
export const app_route: Routes = [
	{
		children: [
			{
				path: "",
				// biome-ignore lint/style/useNamingConvention: Angular
				pathMatch: "full",
				// biome-ignore lint/style/useNamingConvention: Angular
				redirectTo: KeyRoute.LIST,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/access-key/list/access-key-list.component"
					).then((m) => m.AccessKeyListComponent),
				path: KeyRoute.LIST,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/access-key/create/access-key-create.component"
					).then((m) => m.AccessKeyCreateComponent),
				path: KeyRoute.CREATE,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/access-key/detail/access-key-detail.component"
					).then((m) => m.AccessKeyDetailComponent),
				path: KeyRoute.DETAIL,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/bucket/list/bucket-list.component"
					).then((m) => m.BucketListComponent),
				path: BucketRoute.LIST,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/bucket/create/bucket-create.component"
					).then((m) => m.BucketCreateComponent),
				path: BucketRoute.CREATE,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/bucket/detail/bucket-detail.component"
					).then((m) => m.BucketDetailComponent),
				path: BucketRoute.DETAIL,
			},
			{
				// biome-ignore lint/style/useNamingConvention: Angular
				loadComponent: () =>
					import(
						"./presentation/features/bucket/object/bucket-object.component"
					).then((m) => m.BucketObjectComponent),
				path: ObjectRoute.LIST,
			},
		],
		component: MainLayoutComponent,
		path: "",
	},
];
