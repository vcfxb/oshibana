import {
	WorkflowEntrypoint,
	WorkflowEvent,
	WorkflowStep,
} from "cloudflare:workers";

/**
 * Welcome to Cloudflare Workers! This is your first Workflows application.
 *
 * - Run `npm run dev` in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your Workflow in action
 * - Run `npm run deploy` to publish your application
 *
 * Learn more at https://developers.cloudflare.com/workflows
 */
 
// // User-defined params passed to your Workflow
// type Params = {
// 	email: string;
// 	metadata: Record<string, string>;
// };

export class ScryfallPull extends WorkflowEntrypoint<Env, null> {
	async run(event: WorkflowEvent<null>, step: WorkflowStep) {
		// Can access bindings on `this.env`
		// Can access params on `event.payload`

		await step.do("trigger container to fetch and load scryfall bulk data", async () => {
			console.info("trigger cloudflare container here");
			return 0;
			// Replace with the actual URL of your scryfall-pull-container deployment
			// This Worker is part of the same project, but usually, we call via 
			// service binding or public URL.
			// Let's assume a Service Binding or call it by name.
			// For testing/simplicity, if we haven't defined a binding yet, we can use a dummy fetch
			// but we should probably add the service binding to wrangler.jsonc.
			
			// We'll use the environment's SC_CONTAINER binding if it exists
			// This is just a placeholder until we add the binding
			// return await fetch("https://oshibana-scryfall-pull-container.oshibana.workers.dev/pull");
		});

		// const files = await step.do("my first step", async () => {
		// 	// Fetch a list of files from $SOME_SERVICE
		// 	return {
		// 		inputParams: event,
		// 		files: [
		// 			"doc_7392_rev3.pdf",
		// 			"report_x29_final.pdf",
		// 			"memo_2024_05_12.pdf",
		// 			"file_089_update.pdf",
		// 			"proj_alpha_v2.pdf",
		// 			"data_analysis_q2.pdf",
		// 			"notes_meeting_52.pdf",
		// 			"summary_fy24_draft.pdf",
		// 		],
		// 	};
		// });

		// // You can optionally have a Workflow wait for additional data,
		// // human approval or an external webhook or HTTP request, before progressing.
		// // You can submit data via HTTP POST to /accounts/{account_id}/workflows/{workflow_name}/instances/{instance_id}/events/{eventName}
		// const waitForApproval = await step.waitForEvent("request-approval", {
		// 	type: "approval", // define an optional key to switch on
		// 	timeout: "1 minute", // keep it short for the example!
		// });

		// const apiResponse = await step.do("some other step", async () => {
		// 	let resp = await fetch("https://api.cloudflare.com/client/v4/ips");
		// 	return await resp.json<any>();
		// });

		// await step.sleep("wait on something", "1 minute");

		// await step.do(
		// 	"make a call to write that could maybe, just might, fail",
		// 	// Define a retry strategy
		// 	{
		// 		retries: {
		// 			limit: 5,
		// 			delay: "5 second",
		// 			backoff: "exponential",
		// 		},
		// 		timeout: "15 minutes",
		// 	},
		// 	async () => {
		// 		// Do stuff here, with access to the state from our previous steps
		// 		if (Math.random() > 0.5) {
		// 			throw new Error("API call to $STORAGE_SYSTEM failed");
		// 		}
		// 	},
		// );
	}
}
export default {
	async fetch(req: Request, env: Env): Promise<Response> {
		let url = new URL(req.url);

		if (url.pathname.startsWith("/favicon")) {
			return Response.json({}, { status: 404 });
		}

		// Get the status of an existing instance, if provided
		// GET /?instanceId=<id here>
		let id = url.searchParams.get("instanceId");
		if (id) {
			let instance = await env.SCRYFALL_PULL.get(id);
			return Response.json({
				status: await instance.status(),
			});
		}

		// Spawn a new instance and return the ID and status
		let instance = await env.SCRYFALL_PULL.create();
		// You can also set the ID to match an ID in your own system
		// and pass an optional payload to the Workflow
		// let instance = await env.MY_WORKFLOW.create({
		// 	id: 'id-from-your-system',
		// 	params: { payload: 'to send' },
		// });
		return Response.json({
			id: instance.id,
			details: await instance.status(),
		});
	},

	async scheduled(
		controller: ScheduledController,
		env: Env,
		ctx: ExecutionContext,
	) {
		
		// Spawn a new instance and return the ID and status
		let instance = await env.SCRYFALL_PULL.create();
		
		return Response.json({
			id: instance.id,
			details: await instance.status(),
		});
	},
};

