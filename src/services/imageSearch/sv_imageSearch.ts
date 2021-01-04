import {InputFile} from "telegraf/typings/telegram-types";

const gis = require('g-i-s');
import axios from "axios";

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

}

