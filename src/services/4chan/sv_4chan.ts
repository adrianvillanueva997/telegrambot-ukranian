import axios from "axios";
import { getRandomInt } from "../../utilities/randomizer";

interface chanBoards {
  board: string;
  title: string;
}

interface randomWebm {
  fileURL: string;
  fileTitle: string;
  fileBuffer: Buffer;
}

interface threadData {
  no: number;
  last_modified: number;
  replies: number;
}

export class Chan {
  constructor() {}

  async getBoards(): Promise<chanBoards[]> {
    return await axios.get("https://a.4cdn.org/boards.json").then((r) => {
      let data = [];
      for (let i = 0; i < r.data["boards"].length; i++) {
        const board = r.data["boards"][i].board as string;
        const title = r.data["boards"][i].title as string;
        const apiData = {
          board: board,
          title: title,
        } as chanBoards;
        data.push(apiData);
      }
      return data;
    });
  }

  async getRandomWsgWebm(): Promise<randomWebm> {
    const thread = await axios
      .get("https://a.4cdn.org/wsg/threads.json")
      .then((r) => {
        const pages = r.data;
        const randomPage = pages[getRandomInt(0, r.data.length - 1)];
        return randomPage.threads[
          getRandomInt(0, randomPage.threads.length - 1)
        ] as threadData;
      });
    return await axios
      .get(`https://a.4cdn.org/wsg/thread/${thread.no}.json`)
      .then((r) => {
        const data = r.data;
        let randomReply = data.posts[getRandomInt(0, data.posts.length - 1)];
        if (!this.checkWebmStatus(randomReply)) {
          let loop = true;
          while (loop) {
            randomReply = data.posts[getRandomInt(0, data.posts.length - 1)];
            if (this.checkWebmStatus(randomReply)) loop = false;
          }
        }
        return {
          fileTitle: randomReply.filename,
          fileURL: `https://i.4cdn.org/wsg/${randomReply.tim}.webm`,
        } as randomWebm;
      });
  }

  checkWebmStatus(post: any) {
    return !(post.ext === undefined || post.ext !== ".webm");
  }
}
