import axios from "axios";

export class Wikired {
	constructor() {}

	async wikired(): Promise<string> {
		return await axios.get("http://wikired_api/wikired").then((r) => {
			return r.data.text;
		});
	}

	async ukranian(): Promise<string> {
		return await axios.get("http://wikired_api/ukranian").then((r) => {
			return r.data.text;
		});
	}
}
