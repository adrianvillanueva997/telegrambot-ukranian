import {Context, Telegraf} from "telegraf";

require('dotenv').config();

import {commandArgsParser} from "./middleware/md_commandsArgument";
import {OpenWeather} from "./services/openWeather/sv_OpenWeather";
import {argsChecker} from "./middleware/md_argsChecker";
import {ImageSearch} from "./services/imageSearch/sv_imageSearch";
import {getRandomInt} from "./utilities/randomizer";
import {joinArgs} from "./middleware/md_argsJoiner";
import {Chan} from "./services/4chan/sv_4chan";


const app = new Telegraf(process.env.TelegramBot_Ukranian_Key!);

const sendErrorMessage = async (ctx: Context, err: Error) => {
    await ctx.telegram.sendMessage(ctx.message!.chat.id, err.toString())
}

app.command('test', async (ctx) => {
    const args = commandArgsParser(ctx);
    if (args != null) {
        const joinedArgs = joinArgs(args?.arguments);
        console.log(joinedArgs)
    }
    await ctx.telegram.sendMessage(ctx.message!.chat.id, "Patata",
        {reply_to_message_id: ctx.message!.message_id}
    ).then()
})

app.command('weather', async (ctx) => {
    await ctx.telegram.sendChatAction(ctx.message!.chat.id, "typing")
    const args = commandArgsParser(ctx);
    if (args != null) {
        try {
            argsChecker(args, 1, 1);
            const joinedArgs = joinArgs(args.arguments);
            const ow = await new OpenWeather().getWeather(joinedArgs)
            if (ow.status === 404) {
                await ctx.telegram.sendMessage(ctx.message!.chat.id, `City "${args.arguments[0]}" not found!`)
            } else if (ow.status === 200) {
                await ctx.telegram.sendMessage(ctx.message!.chat.id, (`<b>${ow.city},${ow.country} (${ow.cord.lon},${ow.cord.lat})</b>\n
<b>Current Temperature:</b> ${ow.main.temp}ºC
<b>Current weather:</b> ${ow.weather.main}, ${ow.weather.description}
<b>Max Temperature:</b> ${ow.main.temp_max}ºC
<b>Min Temperature:</b> ${ow.main.temp_min}ºC
<b>Temperature feels like:</b> ${ow.main.feels_like}ºC
<b>Wind:</b> ${ow.wind.speed}m/s, ${ow.wind.deg}º
<b>Pressure:</b> ${ow.main.pressure}hPa
<b>Humidty:</b> ${ow.main.humidity}%
<b>Visibility:</b>${ow.visibility}m
`), {parse_mode: "HTML"});
            }
        } catch (err) {
            await sendErrorMessage(ctx, err)
        }
    }
})

app.command('get', async (ctx) => {
    const args = commandArgsParser(ctx);
    await ctx.telegram.sendChatAction(ctx.message!.chat.id, "upload_photo")
    if (args != null) {
        try {
            argsChecker(args, 1, 1);
            const joinedArgs = joinArgs(args.arguments);
            const imageSearch = new ImageSearch();
            const images = await imageSearch.getImage(joinedArgs).then((r) => {
                return r as []
            })
            const image = await imageSearch.getRandomImage(images[getRandomInt(0, images.length - 1)])
            await ctx.telegram.sendPhoto(ctx.message!.chat.id, image, {caption: image, parse_mode: "HTML"})
        } catch (err) {
            await sendErrorMessage(ctx, err)
        }
    }
})

app.command('getboards', async (ctx) => {
    try {
        await ctx.telegram.sendChatAction(ctx.message!.chat.id, "typing")
        const chan = new Chan();
        const boards = await chan.getBoards();
        let message = "";
        for (let i = 0; i < boards.length; i++) {
            const board = boards[i].board;
            const title = boards[i].title;
            message += `<b>${board}</b>: ${title}\n`
        }
        await ctx.telegram.sendMessage(ctx.message!.chat.id, message, {parse_mode: "HTML"})
    } catch (err) {
        await sendErrorMessage(ctx, err)
    }
})

app.command('getfunnyshit', async (ctx) => {
    try {
        await ctx.telegram.sendChatAction(ctx.message!.chat.id, "record_video")
        const chan = new Chan();
        const webm = await chan.getRandomWsgWebm();
        await ctx.telegram.sendMessage(ctx.message!.chat.id, `<a href="${webm.fileURL}">${webm.fileTitle}</a>`,
            {parse_mode: "HTML"})
    } catch (err) {
        await sendErrorMessage(ctx, err)
    }
})

app.launch().then(r => console.log("Bot running!"))