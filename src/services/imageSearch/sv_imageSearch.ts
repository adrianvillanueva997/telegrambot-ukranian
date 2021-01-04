import axios from "axios";

const gis = require('g-i-s');

interface imageData {
    url: string,
    width: number,
    height: number
}

export class ImageSearch {
    constructor() {
    }

    async getImage(queryParam: string) {
        return new Promise((resolve, reject) => {
            gis(queryParam, (error: any, results: any) => {
                if (error) {
                    reject(error)
                } else {
                    resolve(results)
                }
            })
        })
    }

    async getRandomImage(imageUrl: imageData) {
        return imageUrl.url
    }

    async checkImageStatus(imageUrl: string) {
        try {
            return await axios.get(imageUrl).then(r => {
                return r.status;
            });
        } catch (err) {
        }
        return null
    }

}

