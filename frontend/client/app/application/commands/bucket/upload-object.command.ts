import { Injectable, inject } from "@angular/core";

import { BucketService } from "../../../infrastructure/api/bucket.service";
import { TaskProgressService } from "../../../presentation/layout/task-progress.service";

// biome-ignore lint/style/useNamingConvention: Angular
@Injectable({ providedIn: "root" })
export class UploadObjectCommand {
	private readonly service = inject(BucketService);
	private readonly task_service = inject(TaskProgressService);

	public async execute(
		bucket_id: string,
		bucket_name: string,
		key: string,
		file: File,
	): Promise<void> {
		const filename = key.split("/").pop() || key;

		const { id: task_id, signal } = this.task_service.add_task({
			bucket_id,
			bucket_name,
			filename,
			object_key: key,
			type: "upload",
		});

		try {
			await this.service.upload_file_stream(
				bucket_name,
				key,
				file,
				(progress: number) => {
					this.task_service.update_progress(task_id, progress);
				},
				signal,
			);
			this.task_service.complete_task(task_id);
		} catch (e) {
			// Don't throw on cancel/abort
			if (e instanceof DOMException && e.name === "AbortError") {
				return;
			}
			this.task_service.remove_task(task_id);
			throw e;
		}
	}
}
