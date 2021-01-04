const gis = require('g-i-s');
const util = require('util');

interface imageQuery {
    url: string,
    width: number
    height: number
}


export class ImageSearch {
    public urls = {}

    constructor() {
    }

    async getImage(queryParam: string) {
        const test = util.promisify(gis(queryParam, this.logResults))
        await console.log(test)
    }

    logResults(error: any, results: any) {
        if (error) {
            console.log(error);
        } else {
            return (JSON.stringify(results, null, '  '));
        }
    }

}

