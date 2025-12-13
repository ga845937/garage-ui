import { computed, Injectable, signal } from "@angular/core";

export interface Task {
	id: string;
	type: "upload" | "download";
	filename: string;
	progress: number; // 0-100
	status: "running" | "paused" | "error" | "completed" | "cancelled";
	// Navigation metadata
	bucket_id?: string;
	bucket_name?: string;
	object_key?: string;
}

@Injectable({
	// biome-ignore lint/style/useNamingConvention: Angular
	providedIn: "root",
})
export class TaskProgressService {
	public tasks = signal<Task[]>([]);
	private abort_controllers = new Map<string, AbortController>();

	public active_tasks_count = computed(
		() => this.tasks().filter((t) => t.status === "running").length,
	);

	public add_task(task: Omit<Task, "id" | "progress" | "status">): { id: string; signal: AbortSignal } {
		const id = crypto.randomUUID();
		const controller = new AbortController();
		
		const new_task: Task = {
			...task,
			id,
			progress: 0,
			status: "running",
		};
		
		this.abort_controllers.set(id, controller);
		this.tasks.update((tasks) => [...tasks, new_task]);
		
		return { id, signal: controller.signal };
	}

	public update_progress(id: string, progress: number): void {
		this.tasks.update((tasks) =>
			tasks.map((t) => (t.id === id ? { ...t, progress } : t)),
		);
	}

	public complete_task(id: string): void {
		this.abort_controllers.delete(id);
		this.tasks.update((tasks) =>
			tasks.map((t) =>
				t.id === id ? { ...t, progress: 100, status: "completed" } : t,
			),
		);
		setTimeout(() => {
			this.remove_task(id);
		}, 3000);
	}

	public cancel_task(id: string): void {
		const controller = this.abort_controllers.get(id);
		if (controller) {
			controller.abort();
			this.abort_controllers.delete(id);
		}
		this.tasks.update((tasks) =>
			tasks.map((t) =>
				t.id === id ? { ...t, status: "cancelled" } : t,
			),
		);
		setTimeout(() => {
			this.remove_task(id);
		}, 2000);
	}

	public remove_task(id: string): void {
		this.abort_controllers.delete(id);
		this.tasks.update((tasks) => tasks.filter((t) => t.id !== id));
	}
}
